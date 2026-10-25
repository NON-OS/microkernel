# The meter and settle in development images, booted at b/meter-unit 717c5d6c5

Step 1 of the meter unit: `nonos-capsule-payment` (the meter) and
`nonos-capsule-nox-settle` in every development image, and in no release
(tools/nix/config.nix `marketDevFeatures`, the `config` check). Both images
were built from scratch/boot-meter ceacab503: b/meter-unit 717c5d6c5 with
b/mruby-cp and b/settle-endpoint, both on main since. `head.txt` in each
folder has the image's sha256; `build-receipt.json` is the build's own.

The keys were throwaway: `tools/nonos-dev-image` made them in its copy of the
tree, as CI does. No key under `.keys/` was read.

Each machine: `qemu.txt` (the command; Intel IOMMU lane, 4 GiB, fresh data
disk, no TPM). First-boot setup was walked by keystroke over QMP with every
default: amnesic, no network, the Nym route, Qwen3 0.6B, every app on.

## qemu-dev (image 2e203dae…)

- `meter-lines.txt`: `[ZK-ATTEST] ok payment vendor`, `[SPAWN] name=payment
  caps=0x10000079`, `[PAYMENT] capsule spawned`, `[ZK-ATTEST] ok nox_settle
  vendor`, `[SPAWN] name=nox_settle caps=0x3C`, `[NOX-SETTLE] capsule
  spawned`, and each one's first run (`[USER-FIRST]`).
- Neither process logged an exit, fault or reap in the six minutes the
  desktop was up (`serial.log`, 1766 lines, to the end of the run).
- `20-desktop.png`: the desktop, menu bar and the full dock, with the live
  session dialog (3347 colours). `21-installer.png`: Install NONOS opened the
  installer.
- Not spawned: the NVMe and SDHCI drivers, no controller in this machine.

## standard-dev (image bf839ff8…)

- `meter-lines.txt`: the same six lines and the two first runs.
- No exit, fault or reap of either in six minutes (`serial.log`, 914 lines).
- `20-desktop.png` (3348 colours), and `21-desktop-6min.png` six minutes on,
  with the dialog dismissed (5904 colours).
- `not-spawned.txt`: the 23 drivers this machine has no device for, each
  "no controller present" (vmbus: "no Hyper-V SynIC").

## Known, not this unit's

- `ipc.call unanswered driver.virtio_net0`, 10 (qemu-dev) and 7
  (standard-dev): build-and-boot's B-04.
- Nym, Anyone and SOCKS refusals: setup chose no network.
- `PCR extend failed: no TCG2 handle`: booted without `--tpm`.

## Not shown

A process list: QMP's absolute pointer events do not reach a machine with no
tablet device, so the Processes app was not opened. Liveness is the serial
log's.
