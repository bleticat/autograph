# 001. Use Architecture Decision Records

Date: 2026-05-30

Status: Accepted

## Context

This project uses spec-driven development. Some implementation choices will shape future specs, code structure, tooling, data models, and operational behavior. Those decisions need a durable home that is easy to review, reference, and amend without relying on memory or scattered discussion.

## Decision

We will record significant architectural and product-shaping decisions as Architecture Decision Records (ADRs) in `specs/adr/`.

Each future ADR must follow the rules in this document.

## Rules For Future ADRs

1. Store every ADR in `specs/adr/`.
2. Name files with a zero-padded sequence number and a short kebab-case title: `NNN-short-title.md`.
3. Assign the next unused sequence number. Do not renumber existing ADRs.
4. Keep each ADR focused on one decision.
5. Use concise, decision-oriented writing. Capture the reasoning needed by a future maintainer, not every detail from the discussion.
6. Include these sections, in this order:
   - `# NNN. Title`
   - `Date`
   - `Status`
   - `Context`
   - `Decision`
   - `Consequences`
   - `Alternatives Considered`
   - `References`
7. Use one of these statuses: `Proposed`, `Accepted`, `Superseded`, or `Rejected`.
8. When an ADR supersedes another ADR, link both documents by adding `Supersedes: NNN-title` to the new ADR and updating the old ADR status to `Superseded`.
9. Describe how the decision affects specs, implementation, tests, migrations, and operations when relevant.
10. Prefer linking to related specs, issues, pull requests, code, or external references over copying long source material into the ADR.
11. Treat accepted ADRs as historical records. Amend them only for typo fixes, broken links, or status changes; use a new ADR for changed decisions.
12. ADRs should be written before or alongside the implementation work they justify.

## Consequences

Architectural intent will be visible in the repository and can be reviewed together with specs and code. Future contributors will have a stable trail for why important decisions were made, which should reduce repeated debate and accidental reversals.

This adds a small documentation step to changes that carry architectural weight. Small implementation details that do not affect the project direction do not need an ADR.

## Alternatives Considered

- Keep decisions only in issue or pull request discussion. This is convenient during review, but the reasoning becomes harder to find later.
- Keep a single living architecture document. This can work for current-state documentation, but it loses the historical sequence of decisions and tradeoffs.

## References

- `specs/adr/`
