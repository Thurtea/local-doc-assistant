# Copy icons into src-tauri/icons so Tauri dev uses them
$script = Split-Path -Parent $MyInvocation.MyCommand.Definition
$repoRoot = Resolve-Path (Join-Path $script "..")
$src = Join-Path $repoRoot "icons"
$dest = Join-Path $repoRoot "src-tauri\icons"
if (-not (Test-Path $src)) { Write-Error "Source icons folder not found: $src"; exit 1 }
if (-not (Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }
Copy-Item -Path (Join-Path $src '*') -Destination $dest -Recurse -Force
Write-Output "Copied icons from $src to $dest"