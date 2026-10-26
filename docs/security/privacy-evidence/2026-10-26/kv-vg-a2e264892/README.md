# Keeping volume and volume gate, booted on the installed-NVMe lane

One scratch image serves both units: a local merge of privacy/keeping-volume (2d3d13361) and privacy/volume-gate (415d0d668), **a2e264892** (`head.txt`; image sha256 `54a8c0d6…2f9d1`). The merge's only conflicts were the scope and the keeping gate side by side in `cryptoblock/device.rs` and `key_header_io.rs`.

**Setup:**
- `tools/nonos-dev-image`, throwaway keys;
- profile `kvproof`, scratch only (`build.sh.txt`): the full desktop with installer, capsule serial debug, and `nonos-capsule-proof-io`;
- QEMU 11.1.1, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`, `usb-kbd` and `usb-tablet`.

**The prober** is proof_io, whose `main.rs` is `proof_main.rs.txt`.
- Its manifest is 0x400000058: IPC, Memory, FileSystem and StreamImport. It has **no StoreWrite and no Driver**.
- With keeping off at its start, it writes `/kv-ram` and reads it back.
- After keeping turns on, it looks for `/kv-ram` and `/kv-disk`, and writes `/kv-disk` if it isn't there.

`boot.sh.txt` boots the installed disk alone. It hashes the data volume's region of `install-target.img` (LBA 262144 to the end) before and after each boot.

## The runs

| Run | What happened | Volume hash before → after |
|---|---|---|
| S: stick + blank NVMe, `--fresh --install-target` | Setup in Install mode (`S-03-mode-install.png`); the installer wrote the NVMe and every sector read back (`S-23-erase.png`, `S-24-installed.png`). On the live stick the prober's RAM volume stayed after `[ZS] keeping turned on`, since a live stick never switches. | n/a |
| I1: installed, the first boot | The disk wasn't ready at the prober's first call (`put /kv-ram = -11`, EAGAIN), so the window wasn't exercised. `[ZS] keeping turned on` (1190). The volume was formatted and opened on the NVMe (1457-1458). **The prober, without StoreWrite, wrote and read `/kv-disk` on it** (`put = 21`, `reads back now = 1`). Before the volume gate, the same image (with keeping-volume only) refused every call with `Device(AccessDenied)`. | before the format: a6551df3 → n/a |
| I1, Terminal | `qwen tiers` as tool.model-fetch (`caps=0x40000003D`) read the volume's room: "no room on the data volume" for the largest tiers (`I1-30-qwen-tiers.png`). `qwen get --direct small` began its stream on the volume, found no mirror (QEMU has no internet here), and put the stream down with its mark: "0 KB of 491 MB kept" (`I1-34-qwen-direct.png`). | |
| I2: installed, keeping off at the prober's start | `[DATA] this boot keeps nothing yet: this session's volume is in memory` (963). `/kv-ram` was written and read back, and `/kv-disk` was **not** visible: the disk's volume wasn't opened. At the arm: `[DATA] keeping turned on: the volume held in memory is dropped; 2 names in it were not kept` (1203), then `[ZS] keeping turned on` (1204). The disk's volume opened (1394): `/kv-ram` was gone (no migration), and **`/kv-disk` from I1 read back = 1** (it persisted). | 9645e75c → 9645e75c |
| I3, I4 | The same as I2, keeping on 4 s in. The attempt to remove the setup record didn't take: `rm /nonos/setup/{done,answers}` in I1 and again in I3 (`I1-35-rm-setup.png`, `I3-31-rm-setup.png`), and the restore still turned keeping on. | 9645e75c → 9645e75c |

None of the runs has a FATAL, PANIC, #PF, #GP or `panicked` line, and the prober exited 42 on every boot.

## How each run ended

- S: QMP quit at the Installed screen.
- I1: QMP quit at 01:22:45, which didn't take: QEMU ran on until it was stopped with SIGTERM at about 01:35, before I2 ("tpm-emulator: could not cleanly shut down").
- I2 and I4: QMP quit (01:35:59, 01:42:03).
- I3: the shell's Power key, then confirm; QEMU exited by itself.

## Not shown

- **A boot with keeping off throughout** (no record) wasn't arranged: removing the record from the Terminal didn't stop the restore. That path is the same `place(installed, false) == Ram` that I2 booted, without the arm that follows. It's host-proven (zerostate_proofs `volume_rule`).
- **A full model download:** QEMU has no internet here. model_fetch's feed_begin and feed_end (put down, with a mark) and its volume reads passed the gate. Its feed of real bytes didn't run.
- **The disk-not-ready answer on I1:** the first call got EAGAIN (-11) before the disk was chosen. That's the existing behaviour, and nothing reached the disk.
- **The first attempt's raw logs:** the attempt with keeping-volume alone, where I1 refused every call with `Device(AccessDenied)`, lost its serial logs to a disk-space cleanup. The lines are quoted in the report to the Lead, not kept here.
