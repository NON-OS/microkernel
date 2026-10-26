# L55: init waits for the ACPI capsule's first publish pass, booted

Both images were built from b04b/net-late-bound 222d58dbc, which is main
458fbae92 (it has L55, 8d69daac91) with net.core's item 4, by
`tools/nonos-dev-image` with throwaway keys. Both booted on the Intel IOMMU
lane, with virtio-net on QEMU user networking, 6 GiB, a software TPM and
15 minutes each, under TCG.

## Image A: with the ACPI capsule (`A-spawn-order.txt`, `A-qemu.txt`)

The profile is p08acpi as Privacy used it, `microkernel-full-gui` with
`nonos-capsule-acpi`, plus `microkernel-driver-cdc-ecm` for another boot.
It is a scratch profile in a copy of the tree only.

In order on serial:
- the ACPI capsule's spawn and `[ACPI] capsule spawned` (line 342);
- `[ACPI] first publish pass done` (396);
- `[ACPI] first publish pass in after 324 ms` (405);
- then the driver families: `driver.ahci0` (408), `driver.xhci0` (422),
  then the input, NIC, USB and audio drivers.

No `[WARN] no ACPI first pass in 5000 ms` line appears.

**A finding:** the virtio family is spawned before the ACPI capsule, not
after its first pass: `driver.virtio_rng` (238), `driver.virtio_blk0`,
`driver.virtio_gpu0` and `driver.virtio_net0` (272). It is then followed
by `power` (299) and `acpi` (330). If L55 means every driver to wait, the
virtio drivers do not.

## Image B: standard-dev, no ACPI capsule (`B-spawn-order.txt`, `B-qemu.txt`)

No `[ACPI] capsule spawned` and no first-pass line of either kind appear.

The gap from `app.boot_splash` to the first driver, `driver.virtio_rng`,
is 64 ms. On standard-dev 69462d8b1, from before L55, it was 55 ms, on a
quieter machine. No wait was added.
