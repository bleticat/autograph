# 004. Database Interactions

Date: 2026-05-30

Status: Active

## Context

Bounded contexts need persistence without depending on database drivers or storage details.

Writes also need consistent transaction boundaries. Reads need room for optimized query shapes.

## Decision

Access persistence through shared core ports.

The database is a port in `shared/`. It owns access to persistence resources but does not expose bounded-context query classes directly.

For reads, the database can share a connection. Query classes are initialized from that connection and stay owned by their bounded contexts.

For writes, the database can spawn a unit of work. Command classes are initialized with that unit of work and use transaction-scoped repositories.

The unit-of-work spawn owns transaction lifecycle: commit on success, rollback on failure.

Repositories are write-side ports for loading, saving, and deleting domain entities inside a transaction.

Queries are read-side ports. They may use optimized joins, projections, filters, or read models without changing command or repository APIs.

Concrete database code lives in adapters. Domain code and commands depend on ports, not adapter internals.

## Alternatives

- Let commands and queries use database drivers directly. This is simpler at first but couples core behavior to storage details.
- Manage transactions in boundary layers. This gives callers control but makes transaction safety depend on each call site.
- Put context-specific query factories directly on concrete database adapters only. This keeps the shared port smaller but gives callers less common structure.

## Pros

Write behavior gets automatic transaction boundaries.

Commands stay focused on business changes.

Queries can be tuned for read needs without complicating writes.

The database port stays small and does not need to know every query class.

Database technology can change behind adapters.

Tests can use the same ports as production code.

## Cons

There are more abstractions than direct database calls.

Read and write paths may duplicate some mapping code.

Callers must initialize query and command classes instead of asking the database for context-specific APIs.

## Links to Related ADRs

- Related: [002. Separate Commands From Queries](./002-separate-commands-from-queries.md)
- Related: [003. Project Structure](./003-project-structure.md)
- Related: [005. Tests Structure](./005-tests-structure.md)
