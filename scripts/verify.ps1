$cargo = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"

if (-not (Test-Path -LiteralPath $cargo)) {
    throw "Cargo was not found at '$cargo'. Install Rust with rustup first."
}

Write-Host "Formatting workspace..."
& $cargo fmt --all

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Host "Running workspace tests..."
& $cargo test --workspace

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Host "Running Clippy..."
& $cargo clippy --workspace --all-targets -- -D warnings

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Host "LibFlux verification completed successfully."
