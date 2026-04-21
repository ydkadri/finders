# Architecture Decision Records (ADR)

This directory contains Architecture Decision Records for the FindeRS project.

## What is an ADR?

An ADR captures an important architectural decision along with its context and consequences. It helps future maintainers understand why certain choices were made.

## When to Write an ADR

ADRs are **encouraged but not required**. Consider writing an ADR for:

- **Major dependency additions** - New crates, frameworks, or significant libraries
- **Breaking API changes** - Changes that affect public interfaces or CLI
- **Error handling strategies** - How errors are handled throughout the application
- **Performance architecture** - Decisions about parallelism, caching, algorithms
- **File formats and protocols** - Data storage, configuration formats
- **Tool selection** - Choice of benchmarking tools, CI platforms, etc.

**Don't write ADRs for:**
- Simple bug fixes
- Documentation updates  
- Refactoring within existing patterns
- Minor feature additions that follow established patterns

## How to Create an ADR

1. Copy `template.md` to a new file: `NNNN-title-of-decision.md`
   - Number sequentially (0001, 0002, etc.)
   - Use lowercase with hyphens in title

2. Fill in the template:
   - **Status**: Start with "Proposed", change to "Accepted" when implemented
   - **Context**: Explain the problem or need
   - **Decision**: State what you decided to do
   - **Consequences**: List positive, negative, and neutral outcomes
   - **Alternatives**: Document what else was considered and why rejected

3. Include the ADR in the same PR as the implementation

4. Update this README with a link to the new ADR

## ADRs in This Project

- [Template](template.md) - ADR template for new decisions
- [0001: Comparison Benchmark Architecture](0001-comparison-benchmark-architecture.md) - Release-based benchmarks with GitHub Pages deployment
- [0002: Output Abstraction and v3 Breaking Changes](0002-output-abstraction-and-v3-breaking-changes.md) - Output trait for flexible result formatting
- [0003: GitHub Pages Deployment Strategy](0003-github-pages-deployment-strategy.md) - Branch-based deployment for docs and benchmarks
- [0004: Benchmark Report Generation with Historical Trends](0004-benchmark-report-generation-with-historical-trends.md) - Custom HTML reports with Jinja2 templating

## Status Workflow

- **Proposed** - Decision is being discussed
- **Accepted** - Decision is approved and implemented
- **Deprecated** - No longer follows this decision, but not replaced
- **Superseded** - Replaced by a newer ADR (reference it)

## References

- [ADR GitHub org](https://adr.github.io/) - ADR best practices
- [Joel Parker Henderson's ADR templates](https://github.com/joelparkerhenderson/architecture-decision-record)
