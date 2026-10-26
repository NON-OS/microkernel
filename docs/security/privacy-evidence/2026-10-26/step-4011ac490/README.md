# Policy never waits on vfs: the step machine, booted

The image is the standard profile's development twin at 4011ac490, built
with test keys (sha256 `b190e329b814f91f…`), under QEMU (TCG, q35, `--tpm`).
Policy's restore asks vfs one thing a pass, every vfs call it makes is
bounded at 25 ms (`set_deadline_ms`), and kept fields answer `E_NOT_READY`
until the restore is done. Counts are over each whole serial log;
`serial-extract.txt` holds every `ipc.call`, `[ZS]` and Power Off line.

| Boot | `late policy` / `unanswered policy` | What else |
|---|---|---|
| Install, hw's steps (`iommu=intel`, `--install-target`) | 0 / 0 | installed, every sector read back (`install-09-installed.png`); no `ipc.call` line for any server |
| Kept A, a stick, setup in Install mode | 0 / 0 | Settings: timezone 0 to +3 (`keepA-23-timezone-3.png`); no `[VFS] refused`; Power Off through the wipe |
| Kept A2, boot A's stick | 0 / 0 | setup skipped, keeping turned on by the restore, the clock three hours ahead of UTC (`keepA2-20-desktop.png`) |
| One CPU, plain (`--smp 1`, no lanes, 15 minutes) | 0 / 0 | one `ipc.call unanswered vfs_pool` (below) |
| Amnesic, f8ca93ccd (the same change before main's 11ba1f1205) | 0 / 0 | setup, then ten minutes on the desktop |

No QEMU or swtpm was left running after any boot.

## Open

The one-CPU boot printed `ipc.call unanswered vfs_pool err=-110` at serial
line 794, sixteen lines after vfs began serving, and the restore still
finished (`[ZS] keeping turned on`, line 1066). It is a call that waited the
kernel's whole default timeout with no answer. Policy's calls to vfs wait
at most 25 ms, so it is most likely another capsule's call, with vfs busy
staging the store on one CPU. Kernel lines name no process; the caller is
