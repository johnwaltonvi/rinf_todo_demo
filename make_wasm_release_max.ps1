# Maximum optimization build script for WebAssembly
Write-Host "Building optimized WebAssembly..." -ForegroundColor Green

# Set environment variables for maximum optimization
$env:RUSTUP_TOOLCHAIN = "nightly"
$env:RUSTFLAGS = "-C target-feature=+atomics,+bulk-memory,+mutable-globals -C opt-level=z -C panic=abort -C codegen-units=1"

# Build with wasm-pack
Write-Host "Running wasm-pack build..." -ForegroundColor Yellow
wasm-pack build native/hub --out-dir ../../web/pkg --out-name hub --no-typescript --target web --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort

# Check if build was successful
if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful! Optimizing with wasm-opt..." -ForegroundColor Green

    # Install wasm-opt if not already installed
    Write-Host "Checking for wasm-opt..." -ForegroundColor Yellow
    $wasmOptExists = Get-Command wasm-opt -ErrorAction SilentlyContinue
    if (-not $wasmOptExists) {
        Write-Host "Installing wasm-opt..." -ForegroundColor Yellow
        npm install -g wasm-opt
    }

    # Optimize the generated WASM file
    $wasmFile = "web/pkg/hub_bg.wasm"
    $wasmBackupFile = "web/pkg/hub_bg_backup.wasm"

    if (Test-Path $wasmFile) {
        # Show original file size
        $originalSize = (Get-Item $wasmFile).Length
        $originalSizeKB = [math]::Round($originalSize / 1024, 2)
        Write-Host "Original WASM size: $originalSizeKB KB" -ForegroundColor Cyan

        # Create backup of original file
        Write-Host "Creating backup..." -ForegroundColor Yellow
        Copy-Item $wasmFile $wasmBackupFile
        Write-Host "Backup saved as: hub_bg_backup.wasm" -ForegroundColor Green

        # Optimize with wasm-opt
        Write-Host "Optimizing WASM file with thread support..." -ForegroundColor Yellow
        wasm-opt $wasmFile -Oz --enable-threads --enable-bulk-memory -o $wasmFile

        if ($LASTEXITCODE -eq 0) {
            Write-Host "Optimization complete!" -ForegroundColor Green

            # Show optimized file size and comparison
            $optimizedSize = (Get-Item $wasmFile).Length
            $optimizedSizeKB = [math]::Round($optimizedSize / 1024, 2)
            $savedBytes = $originalSize - $optimizedSize
            $savedKB = [math]::Round($savedBytes / 1024, 2)
            $percentSaved = [math]::Round(($savedBytes / $originalSize) * 100, 1)

            Write-Host "`nOptimization Results:" -ForegroundColor Magenta
            Write-Host "  Original size: $originalSizeKB KB" -ForegroundColor White
            Write-Host "  Optimized size: $optimizedSizeKB KB" -ForegroundColor White
            Write-Host "  Space saved: $savedKB KB ($percentSaved%)" -ForegroundColor Green
        } else {
            Write-Host "wasm-opt failed! Restoring backup..." -ForegroundColor Red
            Copy-Item $wasmBackupFile $wasmFile
        }
    } else {
        Write-Host "Warning: WASM file not found at $wasmFile" -ForegroundColor Red
    }
} else {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "`nBuild process complete!" -ForegroundColor Green