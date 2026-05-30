# 002. Separate Commands From Queries

## Links to Related ADRs

- Related: [003. Project Structure](./003-project-structure.md)
- Related: [004. Database Interactions](./004-database-interactions.md)
- Related: [005. Tests Structure](./005-tests-structure.md)
- Related: [006. Feature Specification Workflow](./006-feature-specification-workflow.md)

## Context

Core behavior has writes and reads. Writes enforce rules and change state. Reads retrieve data for views, decisions, and integrations.

Mixing them hides intent and pushes orchestration into boundary or adapter code.

## Decision

Separate commands from queries inside each bounded context.

Commands are state-changing use cases. Keep them simple: apply one business change, enforce rules, call needed ports, and leave the system valid. Boundary layers should call commands for writes instead of coordinating repositories directly.

Queries are read-side contracts. They can be optimized and adjusted for specific read needs without reshaping write commands. They may return entities, projections, or read models.

When a workflow needs a write and refreshed data, run a command, then a query. Do not duplicate command rules in the boundary layer.

## Pros

Write commands stay small and rule-focused.

Queries can evolve for performance, screens, reports, and integrations.

Read models can change without reshaping command APIs.

Tests can target write and read contracts separately.

## Cons

Small features get some extra ceremony.

Some workflows need both a command and a query.

Command return values require judgment: identifiers or created objects are fine; broad read composition belongs in queries.
