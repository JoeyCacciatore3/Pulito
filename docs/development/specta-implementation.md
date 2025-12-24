# Specta Type Generation Implementation Guide

## Overview

This guide documents how to implement Specta type generation for automatic Rust → TypeScript type synchronization.

## Current Status

- ✅ Specta dependencies are configured in `Cargo.toml` (dependencies and build-dependencies)
- ✅ Type generation is enabled in `src-tauri/src/main.rs` (runs in debug mode)
- ✅ All 32 structs have `#[derive(Type)]` and `#[specta(export)]` attributes
- ✅ Manual TypeScript types have been removed and replaced with imports from generated types
- ✅ All component imports have been updated to use generated types
- ✅ TypeScript compilation succeeds (remaining errors are pre-existing, unrelated to Specta)

## Implementation Complete

The Specta type generation implementation has been completed. All Rust structs that need TypeScript types now have the necessary derives, and all TypeScript code has been updated to import from the generated types file.

**Implementation Date**: Completed

**Generated Types Location**: `src/lib/generated/types.ts`

**Type Generation**: Types are automatically generated when the application runs in debug mode (`cargo build` or `cargo run`). The generation code is located in `src-tauri/src/main.rs` in the `setup()` function.

## Implementation Steps

### Step 1: Add specta to dependencies (not just build-dependencies)

Add to `[dependencies]` in `Cargo.toml`:
```toml
specta = { version = "2.0.0-rc.22", features = ["export"] }
```

### Step 2: Update struct definitions

For each struct that needs to be exported, add `specta::Type` to the derive macro:

**Before:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanItem {
    // ...
}
```

**After:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[specta(export)]
pub struct ScanItem {
    // ...
}
```

### Step 3: Structs that need updating

The following structs in `src-tauri/src/commands/mod.rs` need the `#[specta::Type]` derive:

1. `CacheAnalytics`
2. `CacheContributor`
3. `CacheGrowthPoint`
4. `SystemStats`
5. `AppSettings`
6. `TrashSettings`
7. `MonitoringSettings`
8. `NotificationSettings`
9. `ScanSettings`
10. `CacheEvent`
11. `DiskPulseHealth`
12. `OldFilesSummary`
13. `CacheItem`
14. `SystemHealthData`
15. `GpuInfo`
16. `Temperatures`
17. `NetworkInterfaceInfo`
18. `NetworkConnection`
19. `BatteryInfo`
20. `ProcessInfo`
21. `LoadAverage`
22. `TreeNode`
23. `CleanResult`

And in `src-tauri/src/scanner/mod.rs`:
1. `ScanItem`
2. `ScanResults`
3. `ScanOptions`
4. `FilesystemHealthResults`
5. `StorageRecoveryResults`
6. `DuplicateGroup`

And in `src-tauri/src/trash/mod.rs`:
1. `TrashItem`
2. `TrashMetadata`
3. `TrashData`

### Step 4: Enable type generation in main.rs

Type generation has been enabled in `src-tauri/src/main.rs`:

```rust
#[cfg(debug_assertions)]
{
    use specta::export;
    use specta_typescript::Typescript;

    let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("src/lib/generated");

    std::fs::create_dir_all(&output_dir).expect("Failed to create generated directory");

    let types = export();
    Typescript::default()
        .export_to(&output_dir.join("types.ts"), &types)
        .expect("Failed to export TypeScript types");

    tracing::info!("TypeScript types exported to: {}", output_dir.join("types.ts").display());
}
```

The `specta::export()` function automatically discovers all types with `#[specta(export)]`, so no manual registration is needed.

### Step 6: Update TypeScript imports

Once types are generated, update TypeScript files to import from generated types:

**Before:**
```typescript
// src/lib/stores/scanner.svelte.ts
export interface ScanItem {
    id: string;
    name: string;
    // ...
}
```

**After:**
```typescript
// src/lib/stores/scanner.svelte.ts
export type { ScanItem, ScanResults } from '$lib/generated/types';
```

### Step 7: Remove manual type definitions

After verifying generated types work correctly, remove manual TypeScript interface definitions that duplicate Rust structs.

## Build Script Alternative

Alternatively, create a build script in `build.rs`:

```rust
// src-tauri/build.rs
fn main() {
    // Generate TypeScript types using Specta
    // This runs at build time
}
```

## Testing

After implementation:

1. Run `cargo build` in `src-tauri/`
2. Verify `src/lib/generated/types.ts` is created
3. Check that generated types match expected TypeScript types
4. Update imports in TypeScript files
5. Run TypeScript compiler to verify no type errors
6. Remove duplicate manual type definitions

## Notes

- Specta type generation works best when structs use standard Rust types
- Complex generic types may need manual type definitions
- Enum types also need `#[specta::Type]` derive
- Option types are automatically converted to `Type | null` in TypeScript
- Vec types become `Type[]` in TypeScript

## References

- [Specta Documentation](https://github.com/oscartbeaumont/specta)
- [Tauri-Specta Integration](https://github.com/oscartbeaumont/tauri-specta)
