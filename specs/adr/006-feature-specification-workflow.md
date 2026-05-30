# 006. Feature Specification Workflow

## Links to Related ADRs

- Related: [002. Separate Commands From Queries](./002-separate-commands-from-queries.md)
- Related: [003. Project Structure](./003-project-structure.md)
- Related: [005. Tests Structure](./005-tests-structure.md)

## Context

We are moving to spec-driven development. New features need a written use case before code so tests and implementation have a shared target.

## Decision

Put feature specs in `specs/features/`.

Mirror bounded-context folders inside `specs/features/`. A feature spec belongs to the primary context it changes. Cross-context specs should name affected contexts and split only when they describe separate use cases.

Feature workflow:

1. Write or update the feature spec.
2. List all test cases before implementation.
3. Add tests matching the listed cases.
4. Implement through commands, queries, ports, and adapters.
5. Keep the spec and tests in sync when behavior changes.

Suggested feature spec structure:

- `# Feature Name`
- `## Use Case`
- `## Behavior`
- `## Commands and Queries`
- `## Test Cases`
- `## Open Questions`

`## Test Cases` is required. It must list every expected test case, including success, validation, edge, and regression cases.

## Pros

Features start from behavior instead of code shape.

The full test list makes scope visible before implementation.

Mirrored folders keep specs aligned with bounded contexts.

Specs, tests, and code can be reviewed together.

## Cons

Writing the spec adds upfront work.

The test-case list must be maintained as behavior changes.

Cross-context use cases may require judgment about spec ownership.
