# Pulito Documentation

Welcome to the Pulito documentation. This directory contains comprehensive documentation organized by purpose and audience.

## Documentation Structure

Our documentation follows the [Diátaxis documentation framework](https://diataxis.fr/), which organizes content into four types:

- **Tutorials** - Learning-oriented, step-by-step guides
- **How-to Guides** - Task-oriented, problem-solving guides
- **Reference** - Information-oriented, technical details
- **Explanation** - Understanding-oriented, conceptual information

## Quick Navigation

### For Developers

- **[Architecture](architecture.md)** - System architecture, design patterns, and data flow (Explanation)
- **[API Reference](api.md)** - Complete Tauri IPC API documentation (Reference)
- **[Database Schema](database.md)** - Database structure and relationships (Reference)
- **[Components](components.md)** - Component library documentation (Reference)
- **[Stores](stores.md)** - State management documentation (Reference)
- **[Development Guide](development.md)** - Development workflow and setup (How-to/Tutorial)

### For Contributors

- **[Contributing Guide](contributing.md)** - How to contribute to the project (How-to)
- **[Development Guide](development.md)** - Setting up the development environment (Tutorial)

### For Security Researchers

- **[Security Policy](security.md)** - Security reporting and policy (Reference)
- **[Security Assessment](security-assessment.md)** - Security audit results (Explanation)

## Documentation Index

### Architecture & Design

- **[architecture.md](architecture.md)** - Complete system architecture documentation
  - Technology stack overview
  - Frontend and backend structure
  - IPC communication patterns
  - Data flow diagrams
  - Security architecture
  - Performance considerations

### API Documentation

- **[api.md](api.md)** - Tauri IPC API reference
  - All 26 IPC commands documented
  - Function signatures and parameters
  - Return types and error handling
  - Timeout values
  - Usage examples

### Database

- **[database.md](database.md)** - Database schema documentation
  - All 8 tables documented
  - Indexes and relationships
  - Data retention policies
  - Access patterns

### Frontend

- **[components.md](components.md)** - Component library
  - All Svelte components
  - Props and state management
  - Usage patterns
  - Base UI components

- **[stores.md](stores.md)** - State management
  - All Svelte stores
  - Store APIs
  - Reactive patterns
  - Usage examples

### Development

- **[development.md](development.md)** - Development workflow
  - Setup instructions
  - Hot reload guide
  - Build process
  - Testing
  - [Specta Type Generation](development/specta-implementation.md) - Rust → TypeScript type synchronization guide

- **[contributing.md](contributing.md)** - Contribution guidelines
  - Getting started
  - Code standards
  - Pull request process
  - Bug reporting

### Security

- **[security.md](security.md)** - Security policy
  - Security features
  - Vulnerability reporting
  - Best practices

- **[security-assessment.md](security-assessment.md)** - Security assessment
  - Vulnerability analysis
  - Dependency management
  - Code quality improvements

## Finding What You Need

- **New to the project?** Start with [architecture.md](architecture.md) for an overview
- **Want to contribute?** Read [contributing.md](contributing.md) first
- **Setting up development?** Follow [development.md](development.md)
- **Looking up API details?** Check [api.md](api.md)
- **Understanding components?** See [components.md](components.md) and [stores.md](stores.md)
- **Database questions?** Refer to [database.md](database.md)
- **Security concerns?** See [security.md](security.md)

## Documentation Standards

All documentation follows these standards:

- **Accuracy** - Documentation is kept in sync with the codebase
- **Clarity** - Clear, concise language with code examples
- **Completeness** - All public APIs and interfaces are documented
- **Consistency** - Consistent formatting and terminology throughout

## Archive

Historical documentation and completed audit reports are stored in the [archive/](archive/) directory:

- **AUDIT_REPORT.md** - December 2025 code audit report
- **COMPREHENSIVE_ANALYSIS_REPORT.md** - Comprehensive system analysis and roadmap
- **AUDIT_IMPLEMENTATION_SUMMARY.md** - Summary of completed audit implementations

## Contributing to Documentation

When updating documentation:

1. Ensure changes reflect actual code implementation
2. Update related documentation files if needed
3. Follow existing formatting and style
4. Include code examples where helpful
5. Update this index if adding new documentation files
6. Move completed audit/analysis reports to `archive/` directory

## External Resources

- [Tauri Documentation](https://tauri.app/)
- [Svelte Documentation](https://svelte.dev/)
- [Rust Documentation](https://doc.rust-lang.org/)
