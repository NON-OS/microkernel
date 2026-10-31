# Boot a released ISO under VirtualBox on a Windows runner and bring back what
# it did: an Intel e1000 card instead of QEMU's virtio, the serial port to a
# file, a screenshot, and VBox.log, which records which execution engine the
# runner's own virtualisation allowed.
#
# Usage: windows-virtualbox.ps1 <fetch|create|boot|report>
# fetch reads RELEASE_TAG, ASSET, GH_TOKEN and GITHUB_REPOSITORY; boot reads
# BOOT_SECONDS.
param([Parameter(Mandatory = $true)][ValidateSet('fetch', 'create', 'boot', 'report')][string]$Step)
$ErrorActionPreference = 'Stop'
$vb = 'C:\Program Files\Oracle\VirtualBox\VBoxManage.exe'
$art = Join-Path $PWD 'artifacts'

switch ($Step) {
    'fetch' {
        New-Item -ItemType Directory -Force -Path $art | Out-Null
        gh release download $env:RELEASE_TAG --repo $env:GITHUB_REPOSITORY --pattern $env:ASSET --dir $art --clobber
        Move-Item (Join-Path $art $env:ASSET) (Join-Path $art 'nonos.iso') -Force -ErrorAction SilentlyContinue
        Get-FileHash (Join-Path $art 'nonos.iso') -Algorithm SHA256
    }
    'create' {
        & $vb createvm --name nonos --ostype Linux_64 --register
        & $vb modifyvm nonos --memory 4096 --cpus 2 --firmware efi --graphicscontroller vmsvga --vram 128 `
            --nic1 nat --nictype1 82540EM --accelerate3d off
        & $vb modifyvm nonos --uart1 0x3F8 4 --uartmode1 file (Join-Path $art 'serial.log')
        & $vb storagectl nonos --name SATA --add sata --controller IntelAhci
        & $vb storageattach nonos --storagectl SATA --port 0 --device 0 --type dvddrive --medium (Join-Path $art 'nonos.iso')
    }
    'boot' {
        & $vb startvm nonos --type headless
        Start-Sleep -Seconds ([int]$env:BOOT_SECONDS)
        & $vb controlvm nonos screenshotpng (Join-Path $art 'screen.png')
        & $vb controlvm nonos poweroff
    }
    'report' {
        Copy-Item "$env:USERPROFILE\VirtualBox VMs\nonos\Logs\VBox.log" $art -ErrorAction SilentlyContinue
        Write-Host '=== execution engine ==='
        Select-String -Path (Join-Path $art 'VBox.log') -Pattern 'NEM|VT-x|AMD-V|Driverless' -ErrorAction SilentlyContinue |
            Select-Object -First 6
        Write-Host '=== serial, last 40 lines ==='
        $serial = Join-Path $art 'serial.log'
        if (Test-Path $serial) { Get-Content $serial -Tail 40 }
        else { Write-Host 'no serial output: the guest never reached a console' }
    }
}
