# Builds Jade for Horizon in WSL (Ubuntu) and installs it in Horizon.
#
# Why WSL: a wasm mod built on Windows is refused by Horizon builds older than the fix for
# horizon-rust/horizon#18261 (std symbol names depended on the build machine). Building on Linux
# gives the same names as the game's horizon-wasm-std.wasm. Once the game is on a nightly with
# that fix, `cargo horizon-mod build --wasm` on Windows works too.
#
# Usage: .\build.ps1            build and install
#        .\build.ps1 -NoInstall build only (result in target\horizon-mod\jade_horizon.hmod)
param([switch]$NoInstall)
$ErrorActionPreference = 'Stop'

$project = $PSScriptRoot
$wslProject = (wsl -d Ubuntu -- wslpath -a ($project -replace '\\', '/')).Trim()

# cargo reports progress on stderr: let it through and judge by the exit code alone.
$ErrorActionPreference = 'Continue'
wsl -d Ubuntu -- bash -lc @"
set -e
. ~/.cargo/env
rm -rf ~/jade-horizon-src && mkdir -p ~/jade-horizon ~/jade-horizon-src
cd '$wslProject'
tar --exclude=./target --exclude=./.horizon --exclude=./.git -cf - . | tar -xf - -C ~/jade-horizon-src
# Keep the build cache (target/, .horizon/) and replace only the sources.
cd ~/jade-horizon
find . -mindepth 1 -maxdepth 1 ! -name target ! -name .horizon -exec rm -rf {} +
cp -a ~/jade-horizon-src/. ~/jade-horizon/
find . -path ./target -prune -o -path ./.horizon -prune -o -type f -exec chmod 644 {} +
cargo horizon-mod build --wasm
mkdir -p '$wslProject/target/horizon-mod'
cp target/horizon-mod/jade_horizon.hmod '$wslProject/target/horizon-mod/'
"@
if ($LASTEXITCODE -ne 0) { throw "Build failed" }
$ErrorActionPreference = 'Stop'

if (-not $NoInstall) {
    $game = Join-Path $env:APPDATA 'horizon'
    Copy-Item (Join-Path $project 'target\horizon-mod\jade_horizon.hmod') (Join-Path $game 'horizon-mods') -Force
    $list = Join-Path $game 'horizon-mods.txt'
    if (-not (Select-String -Path $list -Pattern '^mod\.jade_horizon:true$' -Quiet)) {
        Add-Content -Path $list -Value 'mod.jade_horizon:true' -Encoding ascii
    }
    Write-Host "Installed in $game\horizon-mods. Restart Horizon to load it."
}

