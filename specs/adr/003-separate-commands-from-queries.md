# 003. Separate Commands From Queries

## Context

Core behavior has writes and reads. Writes enforce rules and change state. Reads retrieve data for views, decisions, and integrations.

Mixing them hides intent and pushes orchestration into boundary or adapter code.

## Decision

Separate commands from queries inside each bounded context.

Commands are state-changing use cases. They own write orchestration, enforce rules, call needed ports, and leave the system valid. Boundary layers should call commands for writes instead of coordinating repositories directly.

Queries are read-side contracts. They return the data a context exposes without changing state. They may return entities, projections, or read models.

When a workflow needs a write and refreshed data, run a command, then a query. Do not duplicate command rules in the boundary layer.

## Pros

Use-case intent is clear.

Write rules stay in the core.

Read models can evolve without reshaping command APIs.

Tests can target write and read contracts separately.

## Cons

Small features get some extra ceremony.

Some workflows need both a command and a query.

Command return values require judgment: identifiers or created objects are fine; broad read composition belongs in queries.

## Links to Related ADRs

- [001. Use Architecture Decision Records](./001-use-adr.md)
- [002. Project Structure](./002-project-structure.md)
