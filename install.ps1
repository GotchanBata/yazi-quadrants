$ErrorActionPreference = "Stop"

Write-Host "[1/4] Checking dependencies..." -ForegroundColor Blue
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Rust (cargo) is not installed. Please install it from https://rustup.rs/" -ForegroundColor Red
    exit 1
}
if (-not (Get-Command ffmpeg -ErrorAction SilentlyContinue)) {
    Write-Host "Error: ffmpeg is not installed. Please install it and add it to your system PATH." -ForegroundColor Red
    exit 1
}

Write-Host "[2/4] Compiling yazi-quadrants..." -ForegroundColor Blue
$TmpDir = [System.IO.Path]::GetTempFileName()
Remove-Item $TmpDir
New-Item -ItemType Directory -Path $TmpDir | Out-Null

git clone https://github.com/georg/yazi-quadrants.git $TmpDir
Push-Location $TmpDir
cargo build --release
Pop-Location

Write-Host "[3/4] Installing Yazi plugin..." -ForegroundColor Blue
$PluginDir = "$env:APPDATA\yazi\config\plugins\video-quadrants.yazi"
if (-not (Test-Path $PluginDir)) {
    New-Item -ItemType Directory -Path $PluginDir -Force | Out-Null
}

Copy-Item "$TmpDir\target\release\yazi-quadrants.exe" -Destination "$PluginDir\yazi-quadrants.exe" -Force
Copy-Item "$TmpDir\yazi-plugin\main.lua" -Destination "$PluginDir\main.lua" -Force

Remove-Item -Recurse -Force $TmpDir

Write-Host "Installation completed successfully!" -ForegroundColor Green
Write-Host "Please add the following lines to your %APPDATA%\yazi\config\yazi.toml:" -ForegroundColor Yellow
Write-Host ""
Write-Host "  [plugin]"
Write-Host "  prepend_previewers = ["
Write-Host "      { mime = `"video/*`", run = `"video-quadrants`" }"
Write-Host "  ]"
