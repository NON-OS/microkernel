# capsule_driver_uhci

Status: built and proven on the host, not yet wired. No kernel mirror
spawns it and it has no Capsule.mk yet: its service and reply endpoint

## Role

`capsule_driver_uhci` drives a UHCI host controller, the USB 1.1 host of
Intel and VIA chipsets and of QEMU's PIIX3 (8086:7020) and VMware's
(15ad:0774) virtual machines, and serves on `driver.uhci0` the host wire
`driver.xhci0` serves. The class drivers find it through
`src/services/registry/usb_hosts.rs` and use it unchanged.

```text
class driver --NXHC--> driver.uhci0 --PIO grant, DMA region--> UHCI --> devices
```

## Microkernel contract

- `MkDeviceList` finds the first function of class 0Ch, subclass 03h,
  interface 00h whose BAR4 is an I/O range of at least 32 ports.
- `MkDeviceClaim` claims it. `MkPciConfigRead` checks that firmware left
  I/O decode on; `MkPciConfigWrite` sets Bus Master and Interrupt Disable
  and writes LEGSUP (0xC0), the bits `src/hardware/broker/pci/quirk_bits`
  allows a UHCI function, to turn firmware's keyboard and SMI traps off.
- `MkPioGrant` grants BAR4; every register access is `MkPioRead` or
  `MkPioWrite`.
- `MkDmaMap` maps one 20 KiB coherent region below 4 GiB: the frame list,
  the queue heads, the TDs and every buffer.

## Interface contract

The host wire of `driver.xhci0`, the ops a full-speed root port needs:
health (1), port status (3), enable (4) and disable (5) slot, address
device (6), device (7) and configuration (8) descriptors, interrupt IN
endpoint (9) and its reads (0xE), control transfer (0xB), bulk configure
(0x10), OUT (0x11), IN (0x12), reset (0x13) and the polled bulk IN (0x14).
Port status is in xHCI PORTSC form: connected, enabled, speed 1 full or 2
low, connect change. Hub and isochronous ops answer -22. A bulk IN while
the polled IN is armed answers -16. Errors: -5 bus or timeout, -11
nothing yet, -16 busy, -22 malformed, -32 STALL.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x198018`, to be declared with the Capsule.mk:
IPC, Memory, DeviceEnum, Driver, Dma and Pio. No Mmio and no Irq: the
controller is polled.

## Current implemented surface

One controller and its two root ports, full- and low-speed devices, one
request at a time: control, interrupt and bulk transfers on one queue per
kind, the polled bulk IN on its own queue, every wait bounded by time.

## Wire format

A 20-byte header (magic `NXHC`, version, op, flags, reserved, request id,
payload length), then a 4-byte status and the op's payload, as driver.xhci0.

## State ownership

The capsule owns the claim, the grants, the schedule and the slot table.
Each slot is the device's USB address. Devices own their own state.

## Operating rules

Every TD chain is taken back from its queue before the request answers,
so the controller runs nothing the driver is not watching; the polled IN
is the one chain left armed, and a reset or a slot's end takes it back.

## Privacy and persistence

Transfers pass through without being kept or logged. One console line at
start names the controller's vendor and device.

## Runtime lifecycle

With no controller it logs one line and exits `EXIT_ABSENT` before
claiming anything; a failed bring-up is retried on the shared bounded
schedule and ends in `EXIT_GAVE_UP`. Once up it serves until stopped.

## Failure model

A transfer's failure is its reply. I/O decode left off, a refused grant or
map, or a host reset that never clears fails the bring-up attempt, which
releases the claim and with it every grant.

## Release target

USB keyboards, mice, tablets and storage on QEMU's and VMware's UHCI, and
on Intel and VIA chipsets whose only USB 1.1 host it is.

## Release evidence

`uhci_proofs` on the host (33 tests). No boot yet.

## Release checklist

- Endpoint numbers assigned, Capsule.mk, kernel mirror and inventory.
- A QEMU `-usb -device usb-tablet` boot moving the pointer.

## Explicit non-goals today

No hubs, no isochronous transfers, no interrupts, no suspend, no second
controller of a chipset.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/uhci_proofs && cargo test --release)`.
