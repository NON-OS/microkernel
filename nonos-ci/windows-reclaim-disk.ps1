# Free C: on a Windows runner before the WSL2 job: the WSL2 disk image grows
# on C:, and a from-source make check once ran it out of space. The image's
# preinstalled SDKs that nothing here uses are removed, all at once rather
# than one after another.
$ErrorActionPreference = 'Stop'
Get-PSDrive C | Format-Table Used, Free
$dirs = 'C:\Android', 'C:\hostedtoolcache\windows\CodeQL', 'C:\hostedtoolcache\windows\go',
        'C:\hostedtoolcache\windows\PyPy', 'C:\hostedtoolcache\windows\Ruby',
        'C:\Program Files\dotnet', 'C:\ghcup', 'C:\Strawberry', 'C:\Miniconda',
        'C:\Program Files (x86)\Android'
$jobs = foreach ($d in $dirs) {
    if (Test-Path $d) {
        Start-Process -FilePath cmd.exe -ArgumentList '/c', 'rmdir', '/s', '/q', "`"$d`"" -NoNewWindow -PassThru
    }
}
$jobs | Wait-Process
Get-PSDrive C | Format-Table Used, Free
