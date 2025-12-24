# Development Workflow Guide

## Quick Development Setup

### Recommended: Use `tauri:dev` (Automatic Hot Reload)

**IMPORTANT**: Always use `npm run tauri:dev` to run the desktop application. Do NOT use `npm run dev` (browser mode) as it will show "Tauri API unavailable" errors because the frontend runs in a browser without the Tauri backend.

Run once and keep it running:
```bash
npm run tauri:dev
```

This command:
- ✅ **Automatically rebuilds Rust code** when you change `.rs` files (incremental compilation - fast after first build)
- ✅ **Hot-reloads frontend** when you change Svelte/TypeScript files (instant)
- ✅ **Watches both frontend and backend** for changes
- ✅ **No need to restart** unless you change `Cargo.toml` dependencies or `tauri.conf.json`

### What Gets Hot Reloaded Instantly (No Rebuild):

- All Svelte components (`.svelte` files)
- TypeScript files (`.ts`, `.tsx`)
- CSS/styles
- Store files
- Any frontend-only changes

### What Requires Incremental Rebuild (Automatic, but takes a few seconds):

- Rust backend files (`.rs` files)
- Command handlers
- Tauri configuration (only if you change `tauri.conf.json`)

### When You Need a Full Restart:

Only restart if you:
- Add/remove Rust dependencies in `Cargo.toml`
- Change Tauri configuration significantly
- Encounter weird caching issues

## Development Tips

1. **Keep `tauri dev` running** - Don't kill it between changes
2. **Frontend changes are instant** - See UI updates immediately
3. **Backend changes auto-rebuild** - Wait a few seconds after saving Rust files
4. **Check terminal output** - See compilation status and errors

## Alternative: Separate Frontend/Backend (Advanced)

If you want even faster frontend iteration:

Terminal 1 (Frontend only):
```bash
npm run dev
```

Terminal 2 (Backend only):
```bash
cd src-tauri && cargo watch -x 'build' -x 'run'
```

Then manually open `http://localhost:5173/app` in browser.

**Note**: Using separate frontend/backend (`npm run dev`) will result in "Tauri API unavailable" errors because the browser cannot access the Tauri backend. This approach is only for frontend-only UI development. `tauri:dev` is recommended as it handles everything automatically and provides full functionality.

---

## Testing

### Frontend Testing

We use **Vitest** for frontend testing with **@testing-library/svelte** for component testing.

#### Running Tests

```bash
# Run all tests once
npm test

# Run tests in watch mode (recommended during development)
npm run test:watch

# Generate coverage report
npm run test:coverage

# Open test UI for interactive testing
npm run test:ui
```

#### Test Structure

- Test files: `src/**/*.test.ts` or `src/**/*.spec.ts`
- Setup file: `src/test/setup.ts` (includes testing library configuration)
- Coverage reports: Generated in `coverage/` directory

#### Writing Component Tests

Example component test:

```typescript
import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import Component from './Component.svelte';
import { invoke } from '$lib/utils/tauri';
import * as tauriApi from '@tauri-apps/api';

// Mock Tauri IPC
vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn()
}));

describe('Component', () => {
  it('renders correctly', () => {
    render(Component);
    expect(screen.getByText('Expected Text')).toBeInTheDocument();
  });

  it('calls Tauri command on action', async () => {
    vi.mocked(tauriApi.invoke).mockResolvedValue({ data: 'test' });
    render(Component);

    // Trigger action that calls invoke
    // ... test logic
  });
});
```

#### Coverage Goals

Current coverage thresholds (in `vitest.config.ts`):
- Lines: 50%
- Functions: 45%
- Branches: 40%
- Statements: 50%

These are progressive goals. Increase over time as test coverage improves.

### Backend Testing

We use **Rust's built-in testing framework** for backend tests.

#### Running Tests

```bash
# Run all Rust tests
cd src-tauri && cargo test

# Run with verbose output
cargo test --verbose

# Run specific test
cargo test test_database_initialization

# Run tests in specific module
cargo test db::
```

#### Test Structure

- Tests are in `#[cfg(test)]` modules within each source file
- Integration tests can be added in `src-tauri/tests/` directory
- Use `tempfile` crate for temporary files in tests
- See [Specta Implementation Guide](development/specta-implementation.md) for type generation setup

#### Writing Backend Tests

Example backend test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_functionality() {
        let temp_dir = TempDir::new().unwrap();
        // Test implementation
        assert!(condition);
    }
}
```

#### Testing Tauri Commands

Use `tauri::test` utilities for testing commands:

```rust
use tauri::test;

#[test]
fn test_command() {
    let app = test::mock_app();
    // Test command invocation
}
```

### Test Best Practices

1. **Write tests for critical paths first** - Core functionality, error handling
2. **Mock external dependencies** - Tauri IPC, file system operations
3. **Use descriptive test names** - Clear what is being tested
4. **Test error cases** - Don't just test happy paths
5. **Keep tests fast** - Use mocks to avoid slow operations
6. **Isolate tests** - Each test should be independent

---

## Logging

### Frontend Logging

We use a structured logger utility (`src/lib/utils/logger.ts`) for consistent logging across the frontend.

#### Log Levels

- **DEBUG**: Development-only information (hidden in production)
- **INFO**: General informational messages
- **WARN**: Warning messages about potential issues
- **ERROR**: Error messages that require attention

#### Usage

```typescript
import { logger } from '$lib/utils/logger';

// Basic usage
logger.info('User logged in');
logger.error('Failed to save settings', { operation: 'save_settings' }, error);

// With context (recommended)
logger.debug('Component initialized', { component: 'Dashboard', action: 'mount' });
logger.warn('Invalid input detected', { component: 'Form', field: 'email' });
logger.error('API call failed', { component: 'API', action: 'fetch_data', endpoint: '/api/data' }, error);
```

#### Log Format

Logs follow this format:
```
[ISO_TIMESTAMP] [LEVEL] [context=value ...] message {...args}
```

Example:
```
[2025-12-21T10:30:45.123Z] [ERROR] [component=Dashboard action=load_stats operation=get_system_stats] Failed to get system stats Error: ...
```

#### Environment Configuration

Control log level via environment variable:
```bash
# .env or environment
VITE_LOG_LEVEL=debug  # Options: DEBUG, INFO, WARN, ERROR
```

Default behavior:
- Development: All levels (DEBUG and above)
- Production: WARN and ERROR only

### Backend Logging

We use **tracing** and **tracing-subscriber** for structured logging in Rust.

#### Log Levels

Log levels are controlled via `RUST_LOG` environment variable:
```bash
# Show all logs
RUST_LOG=debug

# Show only info and above for our app
RUST_LOG=pulito=info

# Show debug for specific module
RUST_LOG=pulito::commands=debug

# Show different levels for different modules
RUST_LOG=pulito=info,pulito::commands=debug
```

#### Usage

```rust
use tracing::{info, warn, error, debug};

// Basic usage
info!("Operation completed");
error!("Failed to process: {}", error);

// With structured fields (recommended)
info!(operation = "save_data", file = "/path/to/file", "Data saved successfully");
error!(operation = "load_data", error = %e, "Failed to load data");
```

#### Log Format

- **Debug builds**: Human-readable format with file names and line numbers
- **Release builds**: JSON format for machine parsing and log aggregation

#### Log Configuration

Logging is configured in `src-tauri/src/main.rs`. The configuration:
- Uses `RUST_LOG` environment variable for filtering
- Includes thread IDs and names in debug builds
- Uses JSON formatting in release builds
- Hides module paths for cleaner output

---

## Debugging

### VS Code Debug Configurations

VS Code debug configurations are available in `.vscode/launch.json`:

#### Frontend Debugging

1. **Debug SvelteKit Dev Server**
   - Launches the dev server with debugger attached
   - Breakpoints work in TypeScript/Svelte files
   - Hot reload enabled

2. **Debug Current Test File**
   - Runs the currently open test file with debugger
   - Allows stepping through test code

3. **Debug All Tests**
   - Runs all tests with debugger attached
   - Useful for debugging test failures

#### Backend Debugging

1. **Debug Rust Tests**
   - Runs Rust tests with LLDB debugger
   - Breakpoints work in Rust code
   - Requires CodeLLDB extension

2. **Debug Tauri App**
   - Builds and runs the Tauri app with debugger
   - Breakpoints work in Rust backend
   - Full application debugging

3. **Debug Tauri Dev**
   - Runs `tauri dev` with debugger attached
   - Debugs both frontend and backend simultaneously
   - Recommended for full-stack debugging

#### Compound Configurations

- **Debug Tauri App (Full Stack)**: Launches both frontend and backend debuggers simultaneously

### Debug Scripts

Additional npm scripts for debugging:

```bash
# Debug frontend dev server
npm run debug:frontend

# Debug tests with inspector
npm run debug:test

# Run tests in watch mode (useful for debugging)
npm run test:watch
```

### Debugging Tips

1. **Set breakpoints** - Click in the gutter next to line numbers in VS Code
2. **Inspect variables** - Hover over variables or use the Variables panel
3. **Step through code** - Use F10 (step over), F11 (step into), Shift+F11 (step out)
4. **Watch expressions** - Add expressions to watch panel
5. **Debug console** - Execute code in current context

### Debugging IPC Communication

To debug Tauri IPC calls:

1. Set breakpoints in:
   - Frontend: `src/lib/utils/tauri.ts` (invoke function)
   - Backend: Command handlers in `src-tauri/src/commands/mod.rs`

2. Use logging:
   - Frontend: Check browser console for logger output
   - Backend: Set `RUST_LOG=debug` to see all backend logs

3. Inspect IPC payloads:
   - Add `logger.debug()` calls before/after `invoke()` calls
   - Add `tracing::debug!()` in command handlers

### Common Debugging Scenarios

#### Frontend Issue

1. Open component file
2. Set breakpoint
3. Run "Debug SvelteKit Dev Server"
4. Trigger the code path
5. Inspect state and props

#### Backend Issue

1. Open Rust file
2. Set breakpoint
3. Run "Debug Tauri App" or "Debug Tauri Dev"
4. Trigger the command
5. Inspect variables and stack

#### Test Failure

1. Open test file
2. Set breakpoint in test or code being tested
3. Run "Debug Current Test File"
4. Step through to identify issue

### Troubleshooting

**"Tauri API unavailable" or "loading fails in all locations":**
- **Most common cause**: Running `npm run dev` instead of `npm run tauri:dev`
  - Solution: Stop `npm run dev` and run `npm run tauri:dev` instead
  - Browser mode (`npm run dev`) does not have access to Tauri backend APIs
- **App window not opening**: Check terminal for compilation errors
  - Rust compilation errors will prevent the desktop window from opening
  - Look for error messages in the terminal output
- **Backend not starting**: Verify Rust toolchain is installed correctly
  - Run `rustc --version` to check Rust installation
  - Run `cargo --version` to check Cargo installation
- **Check logs**: Enable debug logging to see what's happening
  - Frontend: Check browser DevTools console (F12)
  - Backend: Check terminal output, use `RUST_LOG=pulito=debug npm run tauri:dev`

**Debugger not attaching:**
- Ensure CodeLLDB extension is installed (for Rust)
- Check that build completed successfully
- Verify launch.json configuration is correct

**Breakpoints not hitting:**
- Ensure source maps are enabled (default in dev mode)
- Check that code path is actually executed
- For Rust, ensure debug symbols are enabled (default in debug builds)

**Logs not appearing:**
- Check log level configuration
- For frontend, check browser console (not VS Code console)
- For backend, check terminal where app is running
