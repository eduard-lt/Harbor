# Poe Tasks Reference

Harbor uses [Poe the Poet](https://github.com/nat-n/poethepoet) for task automation. This document provides a quick reference for all available tasks.

## 📋 Quick Reference

View all available tasks:
```powershell
poe list
# or just
poe
```

## 🏗️ Build Tasks

| Command | Description |
|---------|-------------|
| `poe build` | Build release binaries |
| `poe build-debug` | Build debug binaries |
| `poe clean` | Remove build artifacts |

**Examples:**
```powershell
# Build optimized release binaries
poe build

# Build debug binaries (faster compilation, includes debug symbols)
poe build-debug

# Clean all build artifacts
poe clean
```

---

## ✨ Code Quality Tasks

| Command | Description |
|---------|-------------|
| `poe fmt` | Format code with rustfmt |
| `poe fmt-check` | Check formatting without modifying files |
| `poe clippy` | Run clippy linter with warnings as errors |
| `poe clippy-fix` | Automatically fix clippy warnings |
| `poe check` | Run all checks (fmt, clippy, test) |

**Examples:**
```powershell
# Format all code
poe fmt

# Check if code is formatted (CI-friendly)
poe fmt-check

# Run clippy linter
poe clippy

# Auto-fix clippy issues
poe clippy-fix

# Run ALL quality checks before committing
poe check
```

**Recommended workflow:**
```powershell
# Before committing
poe check
```

---

## 🧪 Testing Tasks

| Command | Description |
|---------|-------------|
| `poe test` | Run all tests |
| `poe test-unit` | Run unit tests only |
| `poe test-integration` | Run integration tests only |
| `poe test-core` | Run tests for core crate only |
| `poe test-verbose` | Run tests with verbose output |
| `poe test-watch` | Watch and run tests automatically |

**Examples:**
```powershell
# Run all tests
poe test

# Run only unit tests (faster)
poe test-unit

# Run only integration tests
poe test-integration

# Run tests for a specific crate
poe test-core

# See detailed test output
poe test-verbose

# Continuously run tests on file changes
poe test-watch
```

---

## 📚 Documentation Tasks

| Command | Description |
|---------|-------------|
| `poe doc` | Generate and open documentation |
| `poe doc-all` | Generate docs with dependencies |

**Examples:**
```powershell
# Generate and open docs in browser
poe doc

# Include dependency documentation
poe doc-all
```

---

## 🔧 Development Tasks

| Command | Description |
|---------|-------------|
| `poe dev` | Build and install locally for testing |
| `poe dev-debug` | Build debug and install locally |
| `poe update-local` | Copy binaries to local install |
| `poe watch` | Watch for changes and rebuild |
| `poe watch-run` | Watch, rebuild, and run CLI |

**Examples:**
```powershell
# Build release and install to %LOCALAPPDATA%\Harbor
poe dev

# Build debug version and install (faster iteration)
poe dev-debug

# Just copy already-built binaries
poe update-local

# Auto-rebuild on file changes
poe watch

# Auto-rebuild and run CLI on changes
poe watch-run
```

**Typical development workflow:**
```powershell
# Start watching for changes
poe watch-run

# In another terminal, make changes and save
# The app rebuilds and runs automatically
```

---

## 📦 Installer Tasks

| Command | Description |
|---------|-------------|
| `poe setup-wix` | Install WiX Toolset |
| `poe msi` | Build MSI installer |
| `poe release` | Build binaries + MSI |
| `poe release-check` | Validate release readiness |

**Examples:**
```powershell
# First time setup (requires admin)
poe setup-wix

# Build MSI (requires prior build)
poe msi

# Build everything for release
poe release

# Full release validation
poe release-check
```

**Release workflow:**
```powershell
# 1. Run full validation
poe release-check

# 2. If successful, create release
poe release

# MSI will be in target/wix/
```

---

## 🏷️ Version Management Tasks

| Command | Description |
|---------|-------------|
| `poe version` | Show current version |
| `poe bump-patch` | Bump patch (0.6.0 → 0.6.1) |
| `poe bump-minor` | Bump minor (0.6.0 → 0.7.0) |
| `poe bump-major` | Bump major (0.6.0 → 1.0.0) |

**Examples:**
```powershell
# Check current version
poe version

# Bug fix release
poe bump-patch

# New feature release
poe bump-minor

# Breaking changes
poe bump-major
```

---

## 📦 Dependency Management Tasks

| Command | Description |
|---------|-------------|
| `poe deps-update` | Update dependencies |
| `poe deps-outdated` | Check for outdated deps |
| `poe deps-tree` | Show dependency tree |

**Examples:**
```powershell
# Update all dependencies
poe deps-update

# Check which deps are outdated (requires cargo-outdated)
poe deps-outdated

# Visualize dependency tree
poe deps-tree
```

---

## 🛠️ Utility Tasks

| Command | Description |
|---------|-------------|
| `poe install-dev-tools` | Install dev tools |
| `poe size` | Show binary sizes |
| `poe list` | List all tasks |

**Examples:**
```powershell
# Install cargo-watch, cargo-outdated, etc.
poe install-dev-tools

# Check binary sizes
poe size

# List all available tasks
poe list
```

---

## 🎯 Common Workflows

### Daily Development
```powershell
# 1. Start development session
poe dev-debug

# 2. Watch for changes in another terminal
poe test-watch

# 3. Before committing
poe check
```

### Bug Fixing
```powershell
# 1. Write failing test
poe test-verbose

# 2. Fix the bug
poe watch

# 3. Verify all tests pass
poe test

# 4. Final check
poe check
```

### Feature Development
```powershell
# 1. Create feature branch
git checkout -b feature/my-feature

# 2. Develop with auto-reload
poe watch-run

# 3. Run full test suite
poe test

# 4. Check code quality
poe check

# 5. Test locally
poe dev
```

### Preparing a Release
```powershell
# 1. Update version
poe bump-minor

# 2. Update CHANGELOG.md manually

# 3. Run full validation
poe release-check

# 4. Build release artifacts
poe release

# 5. Commit and tag
git add .
git commit -m "chore: release v0.7.0"
git tag v0.7.0
git push origin main --tags
```

### Troubleshooting Build Issues
```powershell
# 1. Clean everything
poe clean

# 2. Update dependencies
poe deps-update

# 3. Rebuild from scratch
poe build

# 4. Check for issues
poe clippy
```

---

## 🎓 Tips & Tricks

### Speed Up Development
```powershell
# Use debug builds during development (much faster)
poe dev-debug

# Use watch mode to auto-rebuild
poe watch
```

### Before Every Commit
```powershell
# Run this to ensure everything is good
poe check
```

### Testing Specific Features
```powershell
# Run specific test
cargo test test_name

# Run tests for specific module
cargo test downloads
```

### Check What Changed
```powershell
# See dependency tree
poe deps-tree

# Check binary sizes
poe size
```

---

## 🔗 Integration with CI

These tasks align with our CI pipeline:
- `poe fmt-check` → Format Check job
- `poe clippy` → Clippy Lints job  
- `poe test` → Tests job
- `poe build` → Build Check job

Running `poe check` locally ensures your code will pass CI.

---

## ❓ Getting Help

```powershell
# See all available tasks
poe

# Get help for a specific task
poe <task-name> --help
```

For more information, see [CONTRIBUTING.md](CONTRIBUTING.md).
