# Use Case Flow

This document describes the generic flow used by the application for **commands** (state changes) and **queries** (read operations).

## Generic command algorithm

Applicable to handlers such as `add_card`, `delete_card`, `update_card`, `add_project`, and `add_section`.

1. UI invokes a Tauri command.
2. Tauri handler validates/parses inputs.
3. Handler calls `db.begin(...)`.
4. Database opens a transaction and creates a `SqlxUnitOfWork`.
5. Domain command service (`*Commands`) executes business logic using repositories from the unit of work.
6. Unit of work commits the transaction.
7. Handler optionally executes a follow-up query adapter to return fresh state.
8. Result is returned to UI (or error mapped to `TauriErr`).

```mermaid
sequenceDiagram
    participant UI
    participant Tauri as Tauri Command Handler
    participant DB as Database::begin
    participant UoW as SqlxUnitOfWork
    participant Cmd as Domain *Commands
    participant Repo as Repositories
    participant Q as Sqlx*Queries

    UI->>Tauri: invoke(command, payload)
    Tauri->>Tauri: parse/validate input
    Tauri->>DB: begin(async |uow| ...)
    DB->>UoW: open transaction
    Tauri->>Cmd: new(uow)
    Cmd->>Repo: get/save/delete(...)
    Repo-->>Cmd: domain result
    Cmd-->>Tauri: Ok/Err
    Tauri->>UoW: commit()
    UoW-->>Tauri: transaction committed
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
