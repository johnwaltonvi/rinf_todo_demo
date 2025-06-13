$env:RUSTUP_TOOLCHAIN = "nightly"
$env:RUSTFLAGS = "-C target-feature=+atomics,+bulk-memory,+mutable-globals"
wasm-pack build native/hub --out-dir ../../web/pkg --out-name hub --no-typescript --target web --dev -Z build-std=std,panic_abort