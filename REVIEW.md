# Code Review – `autograph-core`

This document provides a thorough review of the `core` crate as requested. It covers architecture, idiomatic Rust, lifetime usage, error handling, transaction safety, and concrete suggestions for improvement.

---

## 1. Architecture

The crate follows a **ports-and-adapters** (hexagonal) pattern:

```
shared/ports.rs   — generic Connection / Transaction / Database traits
shared/sqlite.rs  — SQLite implementations of those traits
tasks/ports.rs    — domain-facing TodoRepository / TaskQueries traits
tasks/sqlite.rs   — SQLite implementations of those traits
tasks/commands.rs — TaskCommands (write side, takes a &TodoRepository)
```

This separation is a good foundation, and for a first Rust project it is genuinely impressive. The ideas below are things to be aware of as the project grows, not blockers.

---

## 2. Idiomatic Rust

### 2.1 Constructors inside traits

Both `Transaction` and `TodoRepository` define a `fn new(…) -> Self` method inside the trait:

```rust
// shared/ports.rs
pub trait Transaction {
    type Conn: Connection;
    fn new(conn: &Self::Conn) -> Self where Self: Sized;
}

// tasks/ports.rs
pub trait TodoRepository {
    type Tx: Transaction;
    fn new(tx: &Self::Tx) -> Self where Self: Sized;
}
```

Rust does not prevent this, but it is unusual. Conventionally, constructors live in **inherent `impl` blocks** or are expressed as `From` / `Into` implementations, not in traits. The `where Self: Sized` guard is needed exactly because trait objects (`dyn Trait`) cannot call such methods, which is a signal that the method does not really belong on the trait boundary.

A more idiomatic approach is to separate construction from the capability interface:

```rust
// Trait only declares capabilities
pub trait Transaction {
    type Conn: Connection;
}

// Construction is an inherent impl concern
impl<'a> SqliteTransaction<'a> {
    pub fn new(conn: &SqliteConnection<'a>) -> Self { … }
}
```

### 2.2 Inconsistency between `TaskQueries::new` and `TodoRepository::new`

`TodoRepository::new` takes `&Self::Tx` (by shared reference), while `TaskQueries::new` takes `Self::Conn` (by value):

```rust
// TodoRepository – by reference
fn new(tx: &Self::Tx) -> Self where Self: Sized;

// TaskQueries – by value
fn new(conn: Self::Conn) -> Self where Self: Sized;
```

Because `SqliteConnection<'a>` is `struct SqliteConnection<'a>(&'a rusqlite::Connection)` – i.e. it is already just a copied reference – taking it by value is not wrong, but the inconsistency is surprising. Pick one convention and apply it everywhere.

### 2.3 `TaskCommands` is a thin wrapper

```rust
pub struct TaskCommands<'a, R: TodoRepository> {
    tasks: &'a R,
}

impl<'a, R: TodoRepository> TaskCommands<'a, R> {
    pub fn add(&self, title: &str) -> Result<i64, String> {
        self.tasks.add(title)
    }
    // …
}
```

At present `TaskCommands` delegates every call verbatim to `R`. Its value only becomes apparent once it holds **business logic** (validation, authorization, domain events). Until then it is extra indirection. Consider either adding validation here or, for now, just calling the repository directly to keep things simple.

### 2.4 `pub(crate)` on tuple-struct fields

```rust
pub struct SqliteConnection<'a>(pub(crate) &'a rusqlite::Connection);
pub struct SqliteTransaction<'a>(pub(crate) &'a rusqlite::Connection);
```

`pub(crate)` exposes the raw `rusqlite::Connection` pointer to every module inside the crate. If the internal representation changes later (e.g. switching to `Arc<Mutex<Connection>>`), every call site must be updated. A thin accessor method is safer:

```rust
impl<'a> SqliteConnection<'a> {
    pub(crate) fn raw(&self) -> &rusqlite::Connection { self.0 }
}
```

### 2.5 Schema migration in `Database::open`

```rust
fn open(path: &str) -> Result<Self, String> {
    let conn = rusqlite::Connection::open(path)…;
    conn.execute_batch("CREATE TABLE IF NOT EXISTS todos …")…;
    Ok(Self { conn })
}
```

Embedding the `todos` DDL inside the generic `SqliteDatabase` couples the infrastructure layer to the domain model. If a second domain module is added (e.g. `notes`), it must either add its own `open` variant or the generic `SqliteDatabase` grows domain-specific SQL. A migration step that runs after construction (or a dedicated `migrate` function) keeps the abstraction clean.

---

## 3. Lifetime Analysis

This was highlighted as an area of doubt. Here is a step-by-step walkthrough.

### 3.1 Why lifetimes appear at all

`rusqlite::Connection::transaction()` requires `&mut self`, which makes it impossible to hold a `rusqlite::Transaction` alongside a shared `&Connection`. The design works around this by wrapping a **raw `&rusqlite::Connection`** in newtype structs so that multiple lightweight handles can coexist:

```
rusqlite::Connection  (owned, lives in SqliteDatabase)
        │
        ├─ SqliteConnection<'a>   wraps  &'a rusqlite::Connection
        └─ SqliteTransaction<'a>  wraps  &'a rusqlite::Connection
```

The lifetime `'a` in both newtypes simply means "this handle cannot outlive the `Connection` it points to", which is exactly what the borrow checker enforces.

### 3.2 Generic Associated Types (GATs) in `Database`

```rust
pub trait Database {
    type Conn<'a>: Connection where Self: 'a;
    type Tx<'a>: Transaction  where Self: 'a;

    fn conn(&self) -> Self::Conn<'_>;
    fn transaction<T>(
        &self,
        f: impl FnOnce(&Self::Tx<'_>) -> Result<T, String>,
    ) -> Result<T, String>;
}
```

The `where Self: 'a` bound on the associated types is the correct way to express "the returned `Conn<'a>` / `Tx<'a>` borrows from `self`". This is idiomatic usage of GATs (stabilised in Rust 1.65). The `'_` lifetime in the return position of `conn` and `transaction` is lifetime elision for the same constraint.

### 3.3 Lifetime of `Transaction::new`

```rust
pub trait Transaction {
    type Conn: Connection;
    fn new(conn: &Self::Conn) -> Self where Self: Sized;
}
```

The problem here is that **no lifetime is mentioned**, so the borrow checker uses elision and treats the `&Self::Conn` as having an anonymous lifetime that does not necessarily appear in `Self`. For the concrete implementation this accidentally works out:

```rust
impl<'a> Transaction for SqliteTransaction<'a> {
    type Conn = SqliteConnection<'a>;
    fn new(conn: &SqliteConnection<'a>) -> Self {
        Self(conn.0)  // copies the &'a rusqlite::Connection out of the wrapper
    }
}
```

`conn.0` is `&'a rusqlite::Connection` (not a borrow of `conn` itself), so the transaction lives for `'a`, not for the lifetime of the `conn` argument. This is **correct but subtle**. If a future implementation actually needed to borrow from `conn`, the missing lifetime would become a compiler error.

To make the intent explicit, the trait could be written with a lifetime parameter:

```rust
pub trait Transaction<'conn>: Sized {
    type Conn: Connection;
    fn new(conn: &'conn Self::Conn) -> Self;
}
```

This clearly states "the transaction may borrow from `conn` for `'conn`".

### 3.4 `TaskCommands<'a, R>`

```rust
pub struct TaskCommands<'a, R: TodoRepository> {
    tasks: &'a R,
}
```

The explicit lifetime `'a` is required because the struct stores a reference. This is straightforward and correct. No changes needed here.

### 3.5 Summary of lifetime verdict

All lifetimes are **correct** as written. The only nuance is the elided lifetime in `Transaction::new` (§3.3), which is worth making explicit if the trait is to be implemented by more types in the future.

---

## 4. Error Handling

Every fallible function returns `Result<_, String>`:

```rust
fn open(path: &str) -> Result<Self, String>;
fn add(&self, title: &str) -> Result<i64, String>;
```

Using `String` as the error type is the simplest possible approach and is a reasonable starting point, but it has drawbacks:

* Callers cannot programmatically distinguish one error from another.
* Error context is lost (e.g. which SQL statement failed, what the id was).
* It prevents implementing `std::error::Error`, which most Rust ecosystems expect.

**Recommended improvement:** define a crate-level error enum and derive `thiserror::Error` on it:

```rust
// In core/src/lib.rs or core/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("{0}")]
    Other(String),
}
```

All `map_err(|e| e.to_string())` calls can then become `?` with an appropriate `From` impl, and callers can match on the variant.

---

## 5. Transaction Safety

`SqliteDatabase::transaction` is implemented by manually issuing raw SQL:

```rust
fn transaction<T>(&self, f: impl FnOnce(&Self::Tx<'_>) -> Result<T, String>) -> Result<T, String> {
    self.conn.execute_batch("BEGIN").map_err(|e| e.to_string())?;
    let tx = SqliteTransaction(&self.conn);
    match f(&tx) {
        Ok(val) => {
            self.conn.execute_batch("COMMIT").map_err(|e| e.to_string())?;
            Ok(val)
        }
        Err(e) => {
            let _ = self.conn.execute_batch("ROLLBACK");
            Err(e)
        }
    }
}
```

There are two problems:

1. **Panic safety**: If `f` panics, neither `COMMIT` nor `ROLLBACK` runs. The database is left with an open transaction. Subsequent operations will fail with "cannot start a transaction within a transaction".
2. **`COMMIT` failure leaves data inconsistent**: If `COMMIT` fails after `f` returns `Ok`, the data is not committed, but the function returns an `Err` with the commit error rather than rolling back explicitly. The transaction is still open, causing the same problem as above.

The reason `rusqlite::Connection::transaction()` is not used is that it requires `&mut self`, which conflicts with the `&self` receiver used everywhere. The cleanest solution is to store the connection behind `Mutex<rusqlite::Connection>`:

```rust
pub struct SqliteDatabase {
    conn: Mutex<rusqlite::Connection>,
}
```

This allows `&self` everywhere while still getting a `MutexGuard` when exclusive access is needed. Alternatively, moving to `&mut self` on `transaction` and `conn` is also valid if single-threaded use is assumed.

---

## 6. Minor Points

| # | File | Observation |
|---|------|-------------|
| 1 | `tasks/sqlite.rs` | `NOT completed` uses SQLite's logical NOT, which returns 1 for 0 and 0 for any non-zero value. Because the column only ever holds 0 or 1, the toggle is correct. If the column were ever written with a value other than 0/1 (e.g. by a third-party tool), `NOT 2` would return 0 instead of 1, silently clamping the result. Using `1 - completed` or `CASE WHEN completed = 0 THEN 1 ELSE 0 END` is more explicit and defensive. |
| 2 | `tasks/ports.rs` | `toggle` and `delete` silently succeed for non-existent ids. The tests (`toggle_nonexistent_id_is_noop`, `delete_nonexistent_id_is_noop`) treat this as expected behaviour, but it is worth documenting in a comment or returning a typed "not found" error so callers can react to it. |
| 3 | `shared/sqlite.rs` | `SqliteConnection` and `SqliteTransaction` are structurally identical (`&'a rusqlite::Connection`). The distinction is purely semantic (read-only vs write-inside-transaction), which is good for type safety. Consider a doc comment explaining the intent. |
| 4 | `core/Cargo.toml` | `rusqlite` is declared as `bundled`, which statically links SQLite. This is convenient but increases compile time and binary size. A feature flag (enabled by default, opt-out) would be more flexible. |
| 5 | `lib.rs` | All SQLite-specific types are re-exported from `lib.rs`. If a non-SQLite backend is added later, callers importing from the crate root would need to change. Consider re-exporting only the traits at the top level and putting SQLite types in a `sqlite` sub-module. |

---

## 7. Quick-Win Summary

In order of impact:

1. **Transaction safety** – wrap `rusqlite::Connection` in `Mutex` and use `rusqlite::Connection::transaction()` instead of manual `BEGIN`/`COMMIT`/`ROLLBACK`.
2. **Error type** – replace `String` with a proper `CoreError` enum (add `thiserror` dependency).
3. **Remove constructors from traits** – move `new` to inherent `impl` blocks; use `From` where conversion is natural.
4. **Make the lifetime in `Transaction::new` explicit** – add a `'conn` lifetime parameter to the trait.
5. **Separate schema migration from `Database::open`** – add a `migrate` or `setup` step.
6. **Fix `NOT completed` SQL** – use `1 - completed` to be explicit.
7. **Document the `Connection`/`Transaction` semantic split** – a one-line doc comment saves the next reader a lot of head-scratching.
