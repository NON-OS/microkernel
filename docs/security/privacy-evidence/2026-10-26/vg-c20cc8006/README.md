# Volume gate, re-booted on its own head

The I1 re-boot the volume gate lands on, on an image of privacy/volume-gate **c20cc8006** (`head.txt`; image sha256 `1871271a…8347`). This is the branch's own head, with main merged and rng_callers_proofs fixed; it is not a merge with another unit. The earlier run, kv-vg-a2e264892, booted volume-gate and keeping-volume merged together.

**Setup:**
- `tools/nonos-dev-image`, throwaway keys, every check passed, [F] included;
- profile `kvproof`, scratch only (`build.sh.txt`): the full desktop with the installer, capsule serial debug, and `nonos-capsule-proof-io`;
- QEMU, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`, `usb-kbd` and `usb-tablet`.

**The prober** is proof_io (`proof_main.rs.txt`).
- It is spawned with caps 0x400000158: its manifest's 0x400000058 (IPC, Memory, FileSystem, StreamImport) plus Debug. It holds **no StoreWrite and no Driver**.
- With keeping off at its start, it writes `/kv-ram` and reads it back.
- Once keeping is on, it looks for `/kv-ram` and `/kv-disk`, and writes `/kv-disk` if it isn't there.

`boot.sh.txt` boots the installed disk alone. It hashes the data volume's region of `install-target.img` (LBA 262144 to the end) before and after.

## The runs

| Run | What happened | Volume hash before → after |
|---|---|---|
| S: stick + blank NVMe, `--fresh --install-target` | Setup in Install mode (`S-03-mode-install.png`). The word "rget" was on screen (`S-23-erase.png`). The installer wrote 182.3 MB and every sector read back (`S-24-installed.png`). On the stick, the prober's RAM volume stayed after keeping turned on, since a live stick never switches. | n/a |
| I1: installed disk alone, first boot | Keeping was off at the prober's start, so the window **was** exercised this time: `[DATA] this boot keeps nothing yet` (960), `/kv-ram` written and read back = 1, `/kv-disk` not visible (-2). At the arm: `[DATA] keeping turned on: the volume held in memory is dropped; 2 names in it were not kept` (1199), then `[ZS] keeping turned on` (1200). The volume was formatted and opened on the NVMe (1381-1382). `/kv-ram` was gone, and **the prober, without StoreWrite, wrote `/kv-disk` and read it back = 1** (1404-1405). | 49ff5836 → e34049ee (formatted and written, keeping on) |
| I1, Terminal | `qwen tiers` as tool.model-fetch (caps 0x40000003D, no StoreWrite) read the volume's room, "no room on the data volume" for the largest tiers (`I1-30-qwen-tiers.png`). `qwen get --direct small` began its stream on the volume, found no mirror (no internet in this QEMU), and put it down with its mark, "0 KB of 491 MB kept" (`I1-34-qwen-direct.png`). There was no EIO and no `[DATA]` refusal. | |

## How each run ended

- **S:** QMP `quit` after the Installed screen. The installer's Enter-to-restart isn't on this branch's path (S-14 is a separate unit), so it wasn't pressed.
- **I1:** QMP `quit` after the two Terminal commands. The volume was hashed after QEMU exited.
- Neither serial log has a FATAL, PANIC, #PF, #GP or `panicked` line. The prober exited 42 on both boots.

## Not shown

- A download that completes: there is no internet here.
- A busy volume scope (16 callers inside): that is held by the unit's host proofs, not by a boot.
