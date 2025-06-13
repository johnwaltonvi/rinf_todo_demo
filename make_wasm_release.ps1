$env:RUSTUP_TOOLCHAIN = "nightly"
$env:RUSTFLAGS = "-C target-feature=+atomics,+bulk-memory,+mutable-globals -C opt-level=z -C panic=abort -C codegen-units=1"
wasm-pack build native/hub --out-dir ../../web/pkg --out-name hub --no-typescript --target web --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort