# vfs no longer waits on policy: booted before and after

The fix is 647ef50e8, on main from 354676751. vfs asks the kernel whether
the boot keeps (`MkKeeping`) and never calls the policy service. Before it,
policy's keeper wrote the settings record through vfs and waited for the
reply, while vfs asked policy back with a 200 ms deadline. Each waited on
the other, so the ask failed, vfs took the boot as keeping nothing, and the
write was refused. The keeper tried again every 5 s.

Every image is the standard profile's development twin (`tools/nonos-dev-image
--profile standard`), built with test keys, booted under QEMU (TCG, q35,
`--tpm`, the `iommu=intel` lane) and driven by keys and clicks.

| Run | Image | `unanswered policy` or `late policy` | What else |
|---|---|---|---|
| Install, before the fix | c43e4e3a9, sha256 `a3ed7fa25ebc1cab…` | 60: 59 from vfs_pool (0x23), 1 from input_router (0x7) | the vfs lines start at serial line 715, after `[ZS] keeping turned on`; the install stopped at -110, the NVMe fault fixed separately (`pre-install-09-stopped.png`) |
| Install, fixed | 354676751, sha256 `790ba4fe3455a371…` | 0 | keeping turned on; installed, every sector read back (`install-09-installed.png`) |
| Amnesic, fixed | the same | 0 | setup with the defaults, then ten minutes on the desktop; keeping never turned on (`amnesic-30-desktop.png`) |
| Kept, boot A, fixed | the same, as a stick | 0 | setup in Install mode, the installer left; Settings opened from the dock, the timezone raised from 0 to +3, the clock moved from 18:49 to 21:49 (`keepA-22-settings.png`, `keepA-23-timezone-3.png`); no `[VFS] refused persist`; Power Off went through the wipe |
| Kept, boot A2, fixed | boot A's stick | 0 | setup skipped, keeping turned on by the restore, the clock at 21:56, three hours ahead of UTC (`keepA2-10-desktop.png`) |

The install runs used hw's steps: `--fresh --install-target`, setup in
Install mode, then the installer's disks, Confirm, and ten minutes of writing.

After boot A the stick held `/nonos/settings/values`, 129 bytes, the
keeper's record, with the timezone at 3. Before the fix that write was
refused every time, so no Settings change made after setup was ever kept.

The full serial logs are kept outside the repository (`*.log` is ignored);
`serial-extract.txt` holds, per run, every policy line, every `[ZS]` line
and the Power Off line, with their line numbers.

## What it does not show

- One run of each, under TCG, on one virtual machine; real hardware is not covered.
- The other `late` lines in the fixed runs (virtio_net0, virtio_gpu0,
  wallpaper) are other services' replies, not policy's, and are not judged here.
