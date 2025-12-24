# Security Policy

## 🔒 **Security Overview**

Pulito takes security seriously. As a system cleanup tool with access to user files and system information, we implement multiple layers of security to protect users and their data.

## 🛡️ **Security Features**

### Input Validation
- **Path Traversal Protection**: All file paths are validated to prevent directory traversal attacks
- **Canonical Path Resolution**: Symlinks are resolved and paths are normalized
- **Absolute Path Requirements**: Only absolute paths are accepted
- **User Directory Restriction**: Operations limited to user's home directory

### System Protection
- **Critical Directory Blocking**: Prevents deletion of system directories (`/bin`, `/etc`, `/usr`, etc.)
- **Root Directory Protection**: Blocks operations on root filesystem
- **System File Exclusion**: Protects essential system files and configurations

### Access Control
- **Principle of Least Privilege**: Application runs with minimal required permissions
- **Sandboxed Operations**: File operations are contained within safe boundaries
- **Permission Validation**: Checks file access permissions before operations

## 🚨 **Reporting Security Vulnerabilities**

### **DO NOT** Report Security Issues Publicly

Security vulnerabilities should **never** be reported on public GitHub issues, discussions, or any other public forum.

### How to Report
1. **GitHub Security Advisory**: Use [GitHub's private vulnerability reporting](https://github.com/JoeyCacciatore3/pulito/security/advisories/new)
2. **Subject**: Brief description of vulnerability
3. **Include**:
   - Detailed description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Your contact information for follow-up

### Response Process
1. **Acknowledgment**: Within 24 hours of receiving your report
2. **Investigation**: Security team will investigate and validate the issue
3. **Updates**: Regular updates on progress (at least weekly)
4. **Fix Timeline**: Based on severity and complexity
5. **Disclosure**: Coordinated disclosure after fix is deployed

## 🏷️ **Severity Levels**

### Critical (CVSS 9.0-10.0)
- Remote code execution
- Privilege escalation
- Complete system compromise
- **Response Time**: 24 hours

### High (CVSS 7.0-8.9)
- Data loss or corruption
- Unauthorized access to sensitive data
- System instability
- **Response Time**: 72 hours

### Medium (CVSS 4.0-6.9)
- Information disclosure
- Limited functionality impact
- **Response Time**: 1 week

### Low (CVSS 0.0-3.9)
- Minor issues with limited impact
- **Response Time**: 2 weeks

## 🔧 **Security Best Practices**

### For Users
1. **Download from Official Sources**: Only download from official GitHub releases
2. **Verify Checksums**: Always verify file integrity using provided SHA256 hashes
3. **Keep Updated**: Install security updates promptly
4. **Run as Regular User**: Don't run with elevated privileges unless necessary
5. **Monitor Activity**: Check system logs for unusual activity

### For Developers
1. **Code Reviews**: All changes undergo security review
2. **Automated Testing**: Security tests run on every commit
3. **Dependency Scanning**: Regular vulnerability scans of dependencies
4. **Input Validation**: All inputs validated and sanitized
5. **Error Handling**: Secure error messages that don't leak sensitive information

## 📋 **Security Audit Checklist**

### Pre-Release
- [ ] Static code analysis completed
- [ ] Dependency vulnerability scan passed
- [ ] Input validation tests pass
- [ ] File operation security tests pass
- [ ] Memory safety verification (Rust)
- [ ] Type safety verification (TypeScript)

### Release Process
- [ ] Code signing implemented
- [ ] Binary integrity verification
- [ ] Distribution channel security
- [ ] Update mechanism security

### Post-Release
- [ ] Vulnerability monitoring active
- [ ] Incident response plan ready
- [ ] Security contact information published
- [ ] Responsible disclosure policy documented

## 🔍 **Third-Party Dependencies**

Pulito uses the following security-reviewed dependencies:

### Rust Dependencies
- `tauri`: Cross-platform desktop framework (actively maintained)
- `rusqlite`: SQLite bindings with bundled SQLite (secure, widely used)
- `tokio`: Async runtime (battle-tested, widely used)
- `tracing`: Logging framework (secure, widely adopted)

### Node.js Dependencies
- `svelte`: Frontend framework (actively maintained)
- `@tauri-apps/api`: Tauri JavaScript API (official, maintained)
- `vitest`: Testing framework (secure, actively maintained)

## 📞 **Contact Information**

- **Security Issues**: security@pulito.dev
- **General Support**: support@pulito.dev
- **PGP Key**: Available upon request for encrypted communications

## 📜 **Security Hall of Fame**

We appreciate security researchers who help keep Pulito safe. With permission, we'll acknowledge responsible disclosures here.

---

**Last Updated**: December 19, 2025
**Version**: 1.0.0
