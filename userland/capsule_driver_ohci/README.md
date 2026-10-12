# capsule_driver_ohci

Status: wired, not yet signed. The kernel mirror (`src/hardware/ohci_capsule`)
embeds and spawns it under `nonos-capsule-driver-ohci` (the
`microkernel-driver-ohci` build) when the inventory sees `UsbOhci`, a
function of class 0Ch, subclass 03h, interface 10h. `driver.ohci0` is held
to the USB class drivers. It has no owner certificate yet, so no image
carries it until the owner signs it.

## Role

`capsule_driver_ohci` drives an OpenHCI host controller, the USB 1.1 host
of AMD, SiS, NEC and Apple chipsets and of VirtualBox (106b:003f) and
QEMU's `pci-ohci`, and serves on `driver.ohci0` the host wire
`driver.xhci0` serves. The class drivers find it through
`src/services/registry/usb_hosts.rs` and use it unchanged.

```text
class driver --NXHC--> driver.ohci0 --MMIO grant, DMA region--> OHCI --> devices
```

## Microkernel contract

- `MkDeviceList` finds the first function of class 0Ch, subclass 03h,
  interface 10h whose BAR0 is a memory range covering the root ports.
- `MkDeviceClaim` claims it; `MkPciConfigWrite` sets Memory Space, Bus
  Master and Interrupt Disable. No vendor register is written.
- `MkMmioMap` maps BAR0. Firmware's SMM driver, when it owns the
  controller, is asked to give it up (OwnershipChangeRequest), and legacy
  keyboard emulation is turned off through HceControl.
- `MkDmaMap` maps one 68 KiB coherent region below 4 GiB: the HCCA, the
  EDs, the TDs and every buffer.

## Interface contract

The host wire of `driver.xhci0`, the ops a full-speed root port needs:
health (1), port status (3), enable (4) and disable (5) slot, address
device (6), device (7) and configuration (8) descriptors, interrupt IN
endpoint (9) and its reads (0xE), control transfer (0xB), bulk configure
(0x10), OUT (0x11), IN (0x12), reset (0x13) and the polled bulk IN (0x14).
Port status is in xHCI PORTSC form: connected, enabled, speed 1 full or 2
low, connect change. Isochronous ops answer -22. A bulk IN while
the polled IN is armed answers -16; each root port polls on its own. Errors:
-5 bus or timeout, -11 nothing yet, -16 busy, -22 malformed, -32 STALL.

## Authority

`CAPSULE_REQUIRED_CAPS = 0xB8018`: IPC, Memory, DeviceEnum, Driver, Mmio
and Dma. `CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, in a serial-debug build.
No Pio and no Irq: the controller is polled.

## Current implemented surface

One controller and up to 15 root ports, full- and low-speed devices, one
request at a time. One control ED, eight interrupt EDs on the periodic
list, two bulk EDs per slot; each ED's TDs run on one of two places and
the queue rests on the other, so the driver never rewrites a HeadP the
controller may be reading. Every wait is bounded by time; a run that
outlasts its bound is taken back with the ED skipped for a frame.

## Wire format

A 20-byte header (magic `NXHC`, version, op, flags, reserved, request id,
payload length), then a 4-byte status and the op's payload, as driver.xhci0.

## State ownership

The capsule owns the claim, the grants, the schedule and the slot table.
Each slot is the device's USB address. Toggles live in the EDs.

## Operating rules

A transfer's TDs are taken back (done, halt cleared, or settled) before
its reply; the polled IN and the interrupt TDs are the ones left armed. A
slot given back retires its EDs, skipped and empty, back to DATA0.

## Privacy and persistence

Transfers pass through without being kept or logged. One console line at
start names the controller's vendor and device.

## Runtime lifecycle

With no controller it logs one line and exits `EXIT_ABSENT` before
claiming anything; a failed bring-up is retried on the shared bounded
schedule and ends in `EXIT_GAVE_UP`. Once up it serves until stopped.

## Failure model

A transfer's failure is its reply. A refused grant or map, an SMM driver
that keeps the controller, a host reset that never clears, or a
controller that will not become operational fails the bring-up attempt,
which releases the claim and with it every grant.

## Release target

USB keyboards, mice, tablets and storage on VirtualBox's and QEMU's OHCI,
and on AMD, SiS, NEC and Apple chipsets' USB 1.1 ports.

## Release evidence

`ohci_proofs` on the host (38 tests). No boot yet.

## Release checklist

- Signed by the owner and added to the image's features.
- A QEMU `-device pci-ohci -device usb-tablet` boot moving the pointer.
- VirtualBox with USB 1.1 (OHCI) enabled: the tablet found through it.

Hubs are served: OP_CONFIGURE_HUB (0x21) marks a slot a hub, and
OP_ADDRESS_ROUTED (0x20) addresses a full- or low-speed device below one
(no TT: a full-speed hub has none) without resetting the root port. A
low-speed device's EDs say so, which below a full-speed hub has the
controller send the preamble. Giving such a device back leaves the root
port, and the hub, up.

## Explicit non-goals today

No isochronous TDs, no interrupts, no suspend, no second
controller of a chipset, no EHCI companion hand-off (that is EHCI's).

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/ohci_proofs && cargo test --release)`.
