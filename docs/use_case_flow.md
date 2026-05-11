# Use Case Flow

This document describes the generic flow used by the application for **commands** (state changes) and **queries** (read operations).

## Generic command algorithm

Applicable to handlers such as `add_card`, `toggle_card`, `delete_card`, `add_project`, and `add_section`.

1. UI invokes a Tauri command.
2. Tauri handler validates/parses inputs.
3. Handler constructs a domain command service with a reference to the database (`*Commands::new(&db)`).
4. Domain command service calls `db.begin(...)` internally to open a transaction.
5. Business logic executes using repositories from the unit of work.
6. Unit of work commits (or rolls back on error) the transaction.
7. Handler optionally executes a follow-up query adapter to return fresh state.
8. Result is returned to UI (or error mapped to `TauriErr`).

```mermaid
sequenceDiagram
    participant UI
    participant Tauri as Tauri Command Handler
    participant Cmd as Domain *Commands
    participant DB as Database::begin
    participant UoW as SeaOrmUnitOfWork
    participant Repo as Repositories
    participant Q as SeaOrm*Queries

    UI->>Tauri: invoke(command, payload)
    Tauri->>Tauri: parse/validate input
    Tauri->>Cmd: new(&db)
    Cmd->>DB: begin(async |uow| ...)
    DB->>UoW: open transaction
    Cmd->>Repo: get/save/delete(...)
    Repo-->>Cmd: domain result
    Cmd->>UoW: commit()
    UoW-->>Cmd: transaction committed
    Cmd-->>Tauri: Ok/Err
    Tauri->>Q: new(conn).filter(...) / get_project(...) (optional)
    Q-->>Tauri: refreshed data
    Tauri-->>UI: result or TauriErr
```

## Generic query algorithm

Applicable to handlers such as `filter_cards`, `get_project`, `filter_projects`, and `filter_sections`.

1. UI invokes a Tauri query command.
2. Tauri handler validates/parses inputs.
3. Handler constructs query adapter with shared DB connection.
4. Query adapter executes SQL read operation(s).
5. Rows are mapped to domain entities.
6. Data is returned to UI (or error mapped to `TauriErr`).

```mermaid
sequenceDiagram
    participant UI
    participant Tauri as Tauri Query Handler
    participant Q as Sqlx*Queries
    participant DB as SQLite (read)

    UI->>Tauri: invoke(query, params)
    Tauri->>Tauri: parse/validate input
    Tauri->>Q: new(conn).filter(...) / get_project(...)
    Q->>DB: SELECT ...
    DB-->>Q: rows
    Q->>Q: map rows -> entities
    Q-->>Tauri: Vec<Entity>
    Tauri-->>UI: data or TauriErr
```
