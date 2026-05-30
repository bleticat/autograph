# 001. Use Architecture Decision Records

Date: 2026-05-30

Status: Active

## Context

We use spec-driven development. Important decisions need a durable repo-local record near specs and code.

## Decision

Record architectural and product-shaping decisions as ADRs in `specs/adr/`.

ADRs should stay small enough that reading all of them remains practical.

Rules:

1. Name files `NNN-short-title.md`. Use `000-template.md` only for the ADR template.
2. Use the next unused number. Do not renumber old ADRs.
3. Keep one decision per ADR.
4. Use these sections:
   - `# NNN. Title`
   - `Date: YYYY-MM-DD`
   - `Status: Active` or `Status: Superseded by [NNN. Title](./NNN-title.md)`
   - `## Context`
   - `## Decision`
   - `## Alternatives` when useful
   - `## Pros`
   - `## Cons`
   - `## Links to Related ADRs` when related ADRs exist
5. Put `## Links to Related ADRs` at the bottom so the decision and consequences stay first.
6. Label ADR links, for example `Related`, `Changes`, or `Changed by`.
7. ADR links must be bidirectional.
8. Mention effects on specs, implementation, tests, migrations, and operations when relevant.
9. Link related specs, issues, PRs, code, and ADRs instead of copying long material.
10. Treat ADRs as historical records. Use a new ADR for changed decisions.

## Alternatives

- Keep decisions in issues, PRs, or commit messages only. This keeps documentation lighter but makes old decisions harder to discover from the repo.
- Use a heavier ADR format with mandatory owners, status history, and full alternatives. This captures more nuance but makes small decisions more expensive to record.
- Put ADRs in external documentation. This can help broader audiences but increases the chance that decisions drift away from code and specs.

## Pros

Decisions become discoverable and reviewable with the code.

Small ADRs stay cheap to write and quick to read as a set.

## Cons

This adds a documentation step for architectural work.

Status, date, and alternatives add a little more writing to each ADR.
