# 001. Use Architecture Decision Records

## Context

We use spec-driven development. Important decisions need a durable repo-local record near specs and code.

## Decision

Record architectural and product-shaping decisions as ADRs in `specs/adr/`.

Rules:

1. Name files `NNN-short-title.md`.
2. Use the next unused number. Do not renumber old ADRs.
3. Keep one decision per ADR.
4. Use only these sections:
   - `# NNN. Title`
   - `## Context`
   - `## Decision`
   - `## Pros`
   - `## Cons`
   - `## Links to Related ADRs`
5. Mention effects on specs, implementation, tests, migrations, and operations when relevant.
6. Link related specs, issues, PRs, code, and ADRs instead of copying long material.
7. Treat ADRs as historical records. Use a new ADR for changed decisions.

## Pros

Decisions become discoverable and reviewable with the code.

The short template keeps ADRs cheap to write and read.

## Cons

This adds a documentation step for architectural work.

There are no separate status, date, or alternatives fields; important nuance must fit the main sections.

## Links to Related ADRs

- None.
