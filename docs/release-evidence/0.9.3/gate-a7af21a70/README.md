# Gate boot of main a7af21a70, 25 October 2026

The lead's boot gate for 0.9.3, run on main at a7af21a70 (merge 4 and the
K15 GATE fix included).

## How it ran

- Image: `make dev-image` (qemu-dev, throwaway keys, path-only attestation).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, Intel IOMMU lane, fresh data disk.
- Driven by `tools/nonos-drive`: wait for "Capsules spawned", 90 s, then
  Enter through all 13 setup steps (empty name, the amnesic default), 120 s,
  then a screendump.

## What it showed

- `serial.log`, the whole console:
  - `[IOMMU] intel-vt-d present, enforcing=1, unconfined grants=0, identity devices=0`
  - `[VT-D] enumerated devices blocked until claimed`
  - `[SMP] ap=1`, `ap=2` and `ap=3` online, so 4 CPUs ran.
  - `[INIT] Capsules spawned`
  - 55 `[ZK-ATTEST] ok` lines.
  - No FATAL, panic, `[LOCK]`, capsule exit or "not started" line.
- `s00.png`: first-boot setup, step 1 of 13, not one colour.
- `s12.png`: setup's review, step 13 of 13.
- `desktop.png`: the desktop, with the menu bar (NØNOS, Desktop, File, View,
  Go, Help, the clock) and the dock with its 16 icons.

## Open, seen in this run

- One `[DIAG] ipc.call unanswered wallpaper ... err=-110` as the desktop
  started. The wallpaper still drew.
- Setup still reads "NØNOS 0.9.2 BETA": VERSION is bumped at the freeze.
- This is a development image: capsules are admitted on their Merkle path,
  not the STARK proof. The release gate repeats this on the sealed image.
- Not run here: 1 CPU, the plain lane, and GO/NO-GO.
