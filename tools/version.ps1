# Version management script for Harbor
# Usage: .\tools\version.ps1 [show|bump-patch|bump-minor|bump-major]

param(
    [Parameter(Position = 0)]
    [ValidateSet("show", "refresh", "bump-patch", "bump-minor", "bump-major")]
    [string]$Action = "show"
)

$RootDir = Split-Path -Parent $PSScriptRoot
$CargoToml = Join-Path $RootDir "Cargo.toml"
$PyProjectToml = Join-Path $RootDir "pyproject.toml"

function Get-Version {
    $content = Get-Content $CargoToml -Raw
    if ($content -match 'version\s*=\s*"([^"]+)"') {
        return $Matches[1]
    }
    throw "Version not found in Cargo.toml"
}


function Update-Poe-Help {
    param([string]$CurrentVersion)
    
    $parts = $CurrentVersion -split '\.'
    $major = [int]$parts[0]
    $minor = [int]$parts[1]
    $patch = [int]$parts[2]
    
    $nextPatch = "$major.$minor.$($patch + 1)"
    $nextMinor = "$major.$($minor + 1).0"
    $nextMajor = "$($major + 1).0.0"
    
    $content = Get-Content $PyProjectToml -Raw
    
    # Update bump-patch help
    $content = $content -replace 'help = "Bump patch version \(.*?\)"', "help = `"Bump patch version ($CurrentVersion -> $nextPatch)`""
    # Update bump-minor help
    $content = $content -replace 'help = "Bump minor version \(.*?\)"', "help = `"Bump minor version ($CurrentVersion -> $nextMinor)`""
    # Update bump-major help
    $content = $content -replace 'help = "Bump major version \(.*?\)"', "help = `"Bump major version ($CurrentVersion -> $nextMajor)`""
    
    Set-Content $PyProjectToml $content -NoNewline
    Write-Host "  - pyproject.toml (updated help strings)" -ForegroundColor Gray
}

function Set-Version {
    param([string]$NewVersion)
    
    # Update Cargo.toml
    $cargoContent = Get-Content $CargoToml -Raw
    $cargoContent = $cargoContent -replace '(version\s*=\s*")[^"]+(")', "`${1}$NewVersion`$2"
    Set-Content $CargoToml $cargoContent -NoNewline
    
    # Update pyproject.toml (version field)
    $pyContent = Get-Content $PyProjectToml -Raw
    $pyContent = $pyContent -replace '(version\s*=\s*")[^"]+(")', "`${1}$NewVersion`$2"
    Set-Content $PyProjectToml $pyContent -NoNewline
    
    Write-Host "Updated version to $NewVersion" -ForegroundColor Green
    Write-Host "  - Cargo.toml (workspace)" -ForegroundColor Gray
    Write-Host "  - pyproject.toml (version)" -ForegroundColor Gray

    # Update InfoPage.tsx
    $InfoPage = Join-Path $RootDir "packages\ui\src\pages\InfoPage.tsx"
    if (Test-Path $InfoPage) {
        $infoContent = Get-Content $InfoPage -Raw
        # Replace "Version X.Y.Z" with "Version $NewVersion"
        if ($infoContent -match 'Version \d+\.\d+\.\d+') {
            $infoContent = $infoContent -replace 'Version \d+\.\d+\.\d+', "Version $NewVersion"
            Set-Content $InfoPage $infoContent -NoNewline
            Write-Host "  - packages\ui\src\pages\InfoPage.tsx" -ForegroundColor Gray
        }
        else {
            Write-Host "  ! Version string not found in InfoPage.tsx" -ForegroundColor Yellow
        }
    }
    else {
        Write-Host "  ! InfoPage.tsx not found" -ForegroundColor Yellow
    }

    # Update Poe help strings
    Update-Poe-Help $NewVersion
}

function Bump-Version {
    param([string]$BumpType)
    
    $current = Get-Version
    $parts = $current -split '\.'
    
    if ($parts.Count -ne 3) {
        throw "Invalid version format: $current"
    }
    
    $major = [int]$parts[0]
    $minor = [int]$parts[1]
    $patch = [int]$parts[2]
    
    switch ($BumpType) {
        "major" { $newVersion = "$($major + 1).0.0" }
        "minor" { $newVersion = "$major.$($minor + 1).0" }
        "patch" { $newVersion = "$major.$minor.$($patch + 1)" }
    }
    
    Set-Version $newVersion
    
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "  1. Review changes: git diff" -ForegroundColor White
    Write-Host "  2. Commit: git commit -am 'chore: bump version to $newVersion'" -ForegroundColor White
    Write-Host "  3. Tag: git tag v$newVersion" -ForegroundColor White
    Write-Host "  4. Push: git push && git push --tags" -ForegroundColor White
}

# Main execution
switch ($Action) {
    "show" {
        $version = Get-Version
        Write-Host "Current version: $version" -ForegroundColor Cyan
    }
    "refresh" {
        $version = Get-Version
        Write-Host "Refreshing version info for: $version" -ForegroundColor Cyan
        Update-Poe-Help $version
    }
    "bump-patch" {
        Bump-Version "patch"
    }
    "bump-minor" {
        Bump-Version "minor"
    }
    "bump-major" {
        Bump-Version "major"
    }
}
