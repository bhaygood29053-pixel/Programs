$ErrorActionPreference = "Stop"

Write-Host "Installing JS deps"
yarn install

Write-Host "Building Anchor program"
anchor build

Write-Host "Running tests"
anchor test --skip-build
