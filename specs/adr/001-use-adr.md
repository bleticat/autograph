# 001. Use Architecture Decision Records

## Context

We use spec-driven development. Important decisions need a durable repo-local record near specs and code.

## Decision

Record architectural and product-shaping decisions as ADRs in `specs/adr/`.

ADRs should stay small enough that reading all of them remains practical.

Rules:

1. Name files `NNN-short-title.md`.
2. Use the next unused number. Do not renumber old ADRs.
3. Keep one decision per ADR.
4. Use these sections:
   - `# NNN. Title`
   - `## Links to Related ADRs` when related ADRs exist
   - `## Context`
   - `## Decision`
   - `## Pros`
   - `## Cons`
5. Put `## Links to Related ADRs` directly after the title so changes and dependencies are visible first.
6. Label ADR links, for example `Related`, `Changes`, or `Changed by`.
7. ADR links must be bidirectional.
8. Mention effects on specs, implementation, tests, migrations, and operations when relevant.
9. Link related specs, issues, PRs, code, and ADRs instead of copying long material.
10. Treat ADRs as historical records. Use a new ADR for changed decisions.

## Pros

Decisions become discoverable and reviewable with the code.

Small ADRs stay cheap to write and quick to read as a set.

## Cons

This adds a documentation step for architectural work.

There are no separate status, date, or alternatives fields; important nuance must fit the main sections.
