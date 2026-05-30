# 005. Tests Structure

## Links to Related ADRs

- Related: [002. Separate Commands From Queries](./002-separate-commands-from-queries.md)
- Related: [003. Project Structure](./003-project-structure.md)
- Related: [004. Database Interactions](./004-database-interactions.md)

## Context

Core behavior depends on commands, queries, and persistence boundaries working together.

Most useful tests should verify those boundaries, not isolated implementation details.

## Decision

Use mostly integration-style tests for core behavior.

Each test initializes its own database and destroys it within the test lifecycle. Tests must not depend on state created by another test.

Tests call commands or queries, not adapters or repositories directly. Each test should focus on one command or one query as the behavior under test.

Test folder structure should mirror bounded-context separation. A context's tests live in the matching test module or folder.

Shared test helpers are allowed when they reduce setup noise, but they must not hide the command or query being tested.

## Pros

Tests exercise the same core ports used by application code.

Fresh databases keep tests independent and repeatable.

Mirrored folders make test ownership obvious.

Command and query tests stay aligned with the core architecture.

## Cons

Integration-style tests are slower than pure unit tests.

Per-test database setup adds boilerplate.

Helpers need discipline so tests still show the behavior under test clearly.
