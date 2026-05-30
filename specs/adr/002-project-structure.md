# 002. Project Structure

## Context

The core should grow by product capability, not by technical layer. Specs need a clear home for behavior, boundaries, and abstractions.

## Decision

Use a feature-based ports and adapters structure.

Each bounded context gets its own folder. It owns its domain vocabulary and usually contains:

- DDD abstractions: entities, value objects, aggregates, and related data structures.
- Commands for state-changing use cases.
- Ports that describe external needs.
- Query ports for read access.
- Repository ports for persistence.
- Adapters that implement ports for concrete infrastructure.

The internal file split can vary by size. Ownership matters more than identical folders: context-specific behavior and contracts stay in the context.

Use `shared/` only for cross-context contracts and utilities, such as database abstractions, transactions, generic repositories, shared errors, and query helpers.

`shared/` must not hold domain behavior that belongs to one context.

## Pros

New specs usually map to an obvious context.

Domain behavior stays near its commands, ports, and adapters.

Infrastructure can change behind ports.

Shared infrastructure boundaries have one stable place.

## Cons

This creates more small files than a simple layer-based layout.

Bounded-context boundaries require judgment.

`shared/` can become a dumping ground if not curated.

## Links to Related ADRs

- [001. Use Architecture Decision Records](./001-use-adr.md)
