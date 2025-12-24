# Contributing to Pulito

Thank you for your interest in contributing to Pulito! We welcome contributions from the community and are grateful for any help you can provide.

## 🚀 **Getting Started**

### Prerequisites
- **Node.js 20+** (for frontend development)
- **Rust 1.80+** (for backend development)
- **Ubuntu 22.04+** or compatible Linux distribution
- **WebKitGTK 4.1** (for Tauri desktop integration)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/JoeyCacciatore3/pulito.git
cd pulito

# Install Node.js dependencies
npm install

# Run in development mode
npm run tauri dev

# Run tests
npm test
```

## 📋 **Development Workflow**

### 1. Choose an Issue
- Check the [Issues](https://github.com/JoeyCacciatore3/pulito/issues) page for open tasks
- Look for issues labeled `good first issue` or `help wanted`
- Comment on the issue to indicate you're working on it

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 3. Make Changes
- Follow the existing code style and conventions
- Write tests for new features
- Update documentation as needed
- Ensure all tests pass

### 4. Commit Changes
```bash
git add .
git commit -m "feat: add new feature description"
# or
git commit -m "fix: resolve issue description"
```

Use [Conventional Commits](https://conventionalcommits.org/) format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `style:` for formatting
- `refactor:` for code restructuring
- `test:` for testing
- `chore:` for maintenance

### 5. Create Pull Request
- Push your branch to GitHub
- Create a Pull Request with a clear description
- Reference any related issues
- Wait for review and address feedback

## 🧪 **Testing**

### Running Tests
```bash
# Frontend tests
npm test

# Rust tests
cd src-tauri && cargo test

# Full test suite
npm run check && npm test && cargo test
```

### Writing Tests
- Frontend: Add tests to `__tests__` directories
- Backend: Add tests in Rust modules
- Integration: Test IPC communication
- E2E: Test complete user workflows

## 📝 **Code Standards**

### Frontend (Svelte/TypeScript)
- Use TypeScript for all new code
- Follow ESLint configuration
- Use Svelte 5 runes (`$state`, `$derived`, `$effect`)
- Implement proper error handling
- Add accessibility features (aria-labels, semantic HTML)

### Backend (Rust)
- Follow Rust best practices
- Use `tracing` for logging
- Implement proper error handling with `thiserror`/`anyhow`
- Add documentation comments for public APIs
- Write unit tests for all modules

### Security
- Validate all user inputs
- Use safe file operations
- Follow principle of least privilege
- Report security issues privately

## 🐛 **Reporting Bugs**

### Bug Report Template
Please include:
1. **Description**: Clear description of the issue
2. **Steps to reproduce**: Step-by-step instructions
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Environment**: OS, versions, hardware
6. **Logs**: Relevant error messages or logs
7. **Screenshots**: If applicable

### Security Issues
- **DO NOT** report security vulnerabilities publicly
- Use [GitHub's private vulnerability reporting](https://github.com/JoeyCacciatore3/pulito/security/advisories/new)
- Include detailed reproduction steps
- Allow time for fix before public disclosure

## 💡 **Feature Requests**

### Feature Request Template
Please include:
1. **Problem**: What problem does this solve?
2. **Solution**: Proposed implementation
3. **Alternatives**: Other approaches considered
4. **Use Cases**: Real-world examples
5. **Impact**: How it affects users and system

## 📚 **Documentation**

### Types of Documentation
- **Code Comments**: Inline documentation for complex logic
- **API Docs**: Public function and struct documentation
- **User Guides**: How-to guides and tutorials
- **Architecture**: System design and data flow (see [architecture.md](architecture.md))
- **CHANGELOG**: Version history and changes

### Documentation Standards
- Use clear, concise language
- Include code examples where helpful
- Keep documentation current with code changes
- Use consistent formatting and terminology
- All documentation is located in the [`docs/`](../docs/) directory

## 🤝 **Community Guidelines**

### Code of Conduct
- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers learn and contribute
- Maintain professional communication

### Review Process
- All PRs require review before merging
- Automated tests must pass
- Documentation must be updated
- Security implications must be considered

### Recognition
Contributors will be:
- Listed in CHANGELOG for significant contributions
- Acknowledged in release notes
- Added to contributors file for major contributions

## 📞 **Getting Help**

- **Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Documentation**: Check README.md and docs/ folder first

## 📄 **License**

By contributing to Pulito, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to Pulito! 🎉**
