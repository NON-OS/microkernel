# Gate boot of main 23d57ac60, 26 October 2026

The lead's second boot gate for 0.9.3, on main at 23d57ac60. It carries the
aarch64 loader series (b6acf744c, whose x86 .efi bytes changed), the air-gapped
refusal (3d5569670), the Settings note wrap (74f56105e), the spawn report
(d3f2c0abc), the mmap interrupt windows (53fef8720) and the store fix
(ef2740b21).

## How it ran

- Image: `make dev-image PROFILE=standard` (standard-dev, throwaway keys,
  path-only attestation).
- Machine: QEMU TCG, 4 CPUs, Intel IOMMU lane, fresh data disk.
- Driven by `tools/nonos-drive`: wait for "Capsules spawned", 150 s, a shot,
  Enter through the 13 setup steps, 180 s, a shot.

## What it showed

- `serial.log`:
  - `[IOMMU] intel-vt-d present, enforcing=1, unconfined grants=0`
  - `[SMP-PROOF] cpu_count=4 PASS`
  - 67 `[ZK-ATTEST] ok` lines, and init's spawn report says the capsules
    spawned, none failed.
  - No FATAL, panic, `[LOCK]` or `unanswered` line.
- `setup.png`: first-boot setup.
- `desktop.png`: the desktop with the menu bar, the dock and the live-session
  notice.

## Open, seen in this run

- Another CPU's `[USER-FIRST]` line was written into the middle of init's
  `[INIT] Capsules spawned` line. Every check still reads it, since each looks
  for "Capsules spawned", but a console line must not be split.
- Not covered here: agent's receipt push (1241a4bd6) and L81b part B
  (45cc353a9), which landed after this image was copied.
