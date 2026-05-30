# 002. Project Structure

## Context

The codebase needs a core structure that can grow by product capability instead of by technical layer. Spec-driven development should make it clear where a new behavior belongs, which boundary it crosses, and which abstractions must be introduced before adapter details are added.

The structure should keep domain concepts close to their use cases and ports. It should also make shared infrastructure boundaries available without turning shared code into a place for unrelated domain behavior.

## Decision

The core codebase will use a feature-based ports and adapters structure.

Each bounded context gets its own folder. That folder owns the context's domain vocabulary and the abstractions needed to implement the related specs. A bounded context should usually contain:

- DDD abstractions, such as entities, value objects, aggregates, and domain-specific data structures.
- Commands for state-changing use cases.
- Ports that describe what the context needs from the outside world.
- Query ports for read-side access when reads are part of the context contract.
- Repository ports when the context needs persistence.
- Adapters that implement the context's ports for a concrete infrastructure choice.

The exact file split inside a bounded context may vary with the size of the feature. Small contexts can stay compact; larger contexts can split commands, ports, adapters, and domain abstractions into submodules. The important rule is ownership: context-specific behavior and contracts should live in the context folder.

The `shared/` folder is reserved for cross-context contracts and utilities. It may contain things such as database abstractions, transaction boundaries, generic repository contracts, shared errors, shared query helpers, and other infrastructure or utility code used by multiple bounded contexts.

`shared/` should not contain domain behavior that belongs to one bounded context. Code should move into `shared/` only when it represents an application-wide boundary or is genuinely reused across contexts.

## Pros

Feature work has an obvious home. New specs can usually map to one bounded context folder, which keeps the implementation easier to navigate.

Domain behavior stays close to the commands, ports, and adapters that support it. This makes the purpose of each abstraction easier to understand and reduces pressure to create broad technical-layer folders.

Adapters stay behind ports, so infrastructure choices can change without rewriting the domain-facing structure of the core.

The `shared/` folder gives common infrastructure boundaries a stable place without requiring every bounded context to redefine database and transaction concepts.

## Cons

The structure can create more small files and folders than a simple layer-based layout.

Bounded-context boundaries require judgment. If a feature is placed in the wrong context, future work may need module reshaping before the design feels clear again.

`shared/` needs active discipline. If it becomes a general-purpose dumping ground, bounded contexts will become less independent and harder to reason about.
