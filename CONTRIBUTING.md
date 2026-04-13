# Contributing to HELIOS

Thank you for your interest in contributing to HELIOS!

## 🤝 How to Contribute

### Reporting Bugs
1. Check existing [issues](https://github.com/Manavarya09/HELIOS/issues) to avoid duplicates
2. Use the bug report template
3. Include:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Rust version and OS

### Suggesting Features
1. Check existing feature requests first
2. Open a new issue with `feature` label
3. Explain the use case and proposed solution

### Pull Requests
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes following our code style
4. Write tests for new features
5. Commit: `git commit -m 'Add amazing-feature'`
6. Push: `git push origin feature/your-feature`
7. Open a Pull Request

## 📋 Pull Request Guidelines

- ✅ **Do:**
  - Follow existing code style and conventions
  - Add tests for new features
  - Update documentation
  - Use clear commit messages
  - Reference related issues

- ❌ **Don't:**
  - Break existing tests
  - Add unnecessary dependencies
  - Commit secrets or credentials
  - Make massive changes in one PR

## 🎯 Code Style

```rust
// Use meaningful names
fn calculate_system_health() { ... }

// Use proper formatting
fn process_command(&mut self, input: &str) -> Result<String, Error> { ... }

// Add documentation for public APIs
/// Processes user commands and returns the output
pub fn execute(&mut self, command: &str) -> Result<String, Error> { ... }
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific tests
cargo test -- --test-threads=1

# Run with coverage
cargo tarpaulin
```

## 📝 Commit Messages

Format: `<type>: <description>`

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code refactoring
- `test`: Testing
- `chore`: Maintenance

Example: `feat: Add keyboard shortcuts support`

## 💼 Developer Setup

```bash
# Clone and setup
git clone https://github.com/Manavarya09/HELIOS.git
cd HELIOS

# Add remote
git remote add upstream https://github.com/Manavarya09/HELIOS.git

# Create feature branch
git checkout -b feature/my-feature

# Build
cargo build

# Test
cargo test

# Format
cargo fmt

# Lint
cargo clippy
```

## 📞 Getting Help

- [GitHub Discussions](https://github.com/Manavarya09/HELIOS/discussions)
- [Open an Issue](https://github.com/Manavarya09/HELIOS/issues/new)

---

*Thank you for contributing to HELIOS!*