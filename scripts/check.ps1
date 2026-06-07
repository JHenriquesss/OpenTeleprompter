param()

function Check-Step {
    param([string]$Name, [scriptblock]$Block)
    Write-Host "=== $Name ==="
    & $Block
    if (-not $?) {
        Write-Host "FAILED: $Name" -ForegroundColor Red
        exit 1
    }
}

Check-Step -Name "Formatting check" -Block {
    cargo fmt --all -- --check
}

Check-Step -Name "Backend cargo check" -Block {
    cargo check -p openprompter-rs-tauri --all-targets
}

Check-Step -Name "Backend tests" -Block {
    cargo test -p openprompter-rs-tauri
}

Check-Step -Name "Backend clippy" -Block {
    cargo clippy -p openprompter-rs-tauri --all-targets --all-features -- -D warnings
}

Check-Step -Name "Frontend WASM build" -Block {
    trunk build
}

Check-Step -Name "Frontend WASM tests" -Block {
    wasm-pack test --headless --chrome
}

Write-Host "=== All checks passed ===" -ForegroundColor Green
