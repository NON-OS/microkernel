# net.core's learned receive bound and the kept single frame, booted at 222d58dbc

Two dev images built from b04b/net-late-bound 222d58dbc (main 458fbae92 with
item 4b, 6d38b31ef, and 4c, 222d58dbc), each by `tools/nonos-dev-image` with
throwaway keys; no key under `.keys/` was read:

- standard-dev, for the virtio-net boot;
- a scratch profile in a copy of the tree only: `microkernel-full-gui` with
  `nonos-capsule-acpi` and `microkernel-driver-cdc-ecm`, since no profile
  ships a USB NIC. Its ISO step overran the ISO's media size and was not
  used; the disk image is complete and is what booted.

Both on the Intel IOMMU lane (`--lane iommu=intel`), QEMU user networking,
6 GiB, a software TPM, 15 minutes each, TCG, with other builds running on
the same machine. `*-qemu.txt` is each command line.

## Batch path: virtio-net (`i4-virtio-net-lines.txt`)

The lease, then net.core's summary of OP_RX_BATCH at 256, 1024, 4096 and
16384 calls: 0, 0, 3 and 8 not answered in time, and the learned bound at
its 24 ms floor throughout. A fast driver keeps the floor; a late batch
reply is asked for again by number, so none of the 8 lost a frame.

## Single-frame path: cdc-ecm (`i4c-usbnet-net-lines.txt`)

`--lane nic=usb-net`: QEMU's usb-net on the xHCI. driver.cdc_ecm0 binds
the port, net.core binds `driver.cdc_ecm0` and leases 10.0.2.15. Its
OP_RX_PACKET summary: 0, 0, 0 and 5 not answered in time at 256, 1024,
4096 and 16384 calls, the bound learned at 18 then 16 ms, above the 8 ms
floor, as the USB driver is slower than a virtio one. Each of the 5 was
asked for again under the same number and the driver handed the kept
frame back (nonos_mac::resend): before 4c, each was a frame lost.
