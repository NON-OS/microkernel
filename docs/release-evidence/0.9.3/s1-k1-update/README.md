# K1 on QEMU: prepare, update with a new bootloader, and the data volume opens

One lane under TCG, a software TPM (swtpm) kept across every boot, and an
8 GiB NVMe disk, on s1/k1-update at 4b94b17b05: K1's wrapped key, the
one-boot handover, Update NØNOS (keep my data), and the installer's ask for
the disk in use choosing it as a store write does. Two development images
from that tree:

- **A**, as committed (sha256 25399b4fba0cb075...);
- **B**, the same with one caption of the bootloader changed (`VERIFYING`
  to `VERIFYING.` in nonos-bootloader/src/display/boot/init.rs, reverted
  after the build; sha256 1c1fc1fb46968709...). A new bootloader is a new
  PCR 4, so A's own key cannot open the volume under B: only the handover
  can.

Every boot ended with no `[LOCK]` line and no trap.

| Step | Log | What it shows |
|---|---|---|
| A installed onto the blank disk, then its first boot from the disk | `a-prepare.log`, `a-prepared.png` | The block layer chooses the disk at the installer's ask (lines 1750-1753, `[BLOCK] NONOS disk on NVMe`). Install NØNOS offers `Prepare this computer for an update`; P answers `Ready. Restart from the new NØNOS stick and choose Update NØNOS (keep my data). Until you do, your data volume opens without the bootloader check for one start.` (the picture). The volume is formatted and opened under the release-approved chain (lines 1901-1902) |
| B's stick beside the disk: Update NØNOS (keep my data) | `b-update.log`, `b-confirm.png` | The Confirm screen: `Your data volume will be kept`, the update row lit, `kept: data volume (7.4 GB), its key, the plan and the table`, `not kept: store apps, settings since setup, the wallet's vault, Wi-Fi` |
| The disk alone under B, first boot | `b-open1.log` | `[DATA] the handover opened the volume; it now opens under this bootloader` (line 1827), then `[DATA] volume open` |
| The disk alone under B, second boot | `b-open2.log` | `[DATA] volume open` (line 1834) and no handover line: it opens under B's own key |

## The key header, step by step

Read from LBA 245761 of the disk after each boot. The bytes are not
published, only their shape and a digest of the way-3 header (bytes 0..128):

| After | Magic, way | Digest of bytes 0..128 | Handover record (128..196) |
|---|---|---|---|
| prepare (A) | NONOSDK1, 3 | 61ffcb30773f61a8 | `NONOSHO1`, sealed |
| the update (B's stick) | NONOSDK1, 3 | 61ffcb30773f61a8 | `NONOSHO1`, sealed |
| first boot under B | NONOSDK1, 3 | a646743799c7b26c | cleared |
| second boot under B | NONOSDK1, 3 | a646743799c7b26c | cleared |

The update wrote nothing to the key header, as docs/install/update.md says.
The first boot under B opened the volume through the record, wrapped the
volume key again under B's own approved key, and cleared the record; the
second boot needed nothing else.

## What else the logs show

- In `b-update.log` the stick's USB storage stopped answering under TCG
  (lines 1459 and 1502, `not ready ... errno -110`), and B's block layer then
  kept the NVMe disk's store in its place (line 1557). The update still went
  through, from the store the loader had read. That fallback is what main's
  `the kernel keeps the disk the loader was read from` (block_device
  other_disk.rs, settle.rs) now forbids, and this branch carries it since
  b74f8fdb27.
- `import refused: none of the plan's 0 files ...` after each opening boot
  is the Qwen app asking an empty, freshly made volume for its model. The
  lane opens Qwen only to make the volume open.
- The first prepare boot of this lane (before 4b94b17b05) did not offer the
  prepare row: nothing had chosen the disk yet, so the installer's ask found
  no disk in use and no `[BLOCK]` line was printed. 4b94b17b05 is that fix.
