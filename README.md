# Rinf Todo App Demo

A todo application demonstrating Rinf framework usage with:
- **Actors pattern** for message handling
- **Shared app state** management
- **Router-based** signal routing
- **Cross-platform** support (native + WASM)

## Features Demonstrated
- Todo CRUD operations
- Actor-based architecture
- State management with Arc/Mutex
- Signal-based communication between Dart/Rust
- Router pattern for handling different message types

## Quick Start

### Native Build
```
flutter run
```

### Web WASM Build (Windows)
Due to a Windows-sys crate bug, manual WASM compilation is required:

1. Run one of the WASM build scripts:
   - `./make_wasm.ps1` (development)
   - `./make_wasm_release.ps1` (optimized)
   - `./make_wasm_release_max.ps1` (maximum optimization)

2. Then run Flutter:
```
flutter run --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
```

## WASM Build Process

### Build Scripts Explained
The project includes PowerShell scripts that handle WASM compilation:

- **make_wasm_release.ps1**: Production build with optimizations
  - Uses nightly Rust toolchain
  - Enables WASM features: atomics, bulk-memory, mutable-globals
  - Optimizes for size (-C opt-level=z)
  - Uses panic=abort for smaller binary
  - Single codegen unit for better optimization

### Manual Commands (Windows Workaround)
Instead of `rinf wasm`, we use `wasm-pack` directly:
```
powershell env:RUSTUP_TOOLCHAIN = "nightly" env:RUSTFLAGS = "-C target-feature=+atomics,+bulk-memory,+mutable-globals -C opt-level=z -C panic=abort -C codegen-units=1" wasm-pack build native/hub --out-dir ../../web/pkg --out-name hub --no-typescript --target web --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```