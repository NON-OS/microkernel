# capsule_driver_ehci

Status: wired, not yet signed. The kernel mirror (`src/hardware/ehci_capsule`)
embeds and spawns it under `nonos-capsule-driver-ehci` (the
`microkernel-driver-ehci` build) when the inventory sees `UsbEhci`, a
function of class 0Ch, subclass 03h, interface 20h. `driver.ehci0` is held
to the USB class drivers. It has no owner certificate yet, so no image
carries it until the owner signs it.

## Role

`capsule_driver_ehci` drives an EHCI host controller, the USB 2.0 host of
chipsets before xHCI and of QEMU (`usb-ehci`, `ich9-usb-ehci1`),
VirtualBox (8086:265c) and VMware (15ad:0770), and serves high-speed
devices on `driver.ehci0` with the host wire `driver.xhci0` serves. Full-
and low-speed devices are handed to the companion host (UHCI or OHCI),
whose own capsule serves them; the class drivers search every host.

```text
class driver --NXHC--> driver.ehci0 --MMIO grant, DMA region--> EHCI --> high speed
                                                    |
                                     port owner --> companion (driver.uhci0 / ohci0)
```

## Microkernel contract

- `MkDeviceList` finds the first function of class 0Ch, subclass 03h,
  interface 20h whose BAR0 is a memory range of at least 256 bytes.
- `MkDeviceClaim`; `MkPciConfigWrite` sets Memory Space, Bus Master and
  Interrupt Disable.
- `MkMmioMap` maps BAR0. Firmware is let go through the USB legacy support
  capability at HCCPARAMS.EECP: `MkPciConfigWrite` sets USBLEGSUP's
  OS-owned semaphore and clears USBLEGCTLSTS's SMI enables and status, the
  bits `src/hardware/broker/pci/quirk_bits` allows an EHCI function there.
- `MkDmaMap` maps one 76 KiB coherent region below 4 GiB: the frame list,
  the QHs, the qTDs and every buffer.

## Interface contract

The host wire of `driver.xhci0`, the ops a high-speed root port needs:
health (1), port status (3), enable (4) and disable (5) slot, address
device (6), device (7) and configuration (8) descriptors, interrupt IN
endpoint (9) and its reads (0xE), control transfer (0xB), bulk configure
(0x10), OUT (0x11), IN (0x12), reset (0x13) and the polled bulk IN (0x14).
Port status shows only high-speed devices, at speed 3; a port handed to
the companion reads as empty here. Isochronous ops answer -22. A bulk IN
while the polled IN is armed answers -16; each armed polled IN holds a
place of its own, so devices poll side by side, below one hub too, and
with every place held a further device's first poll answers -16. Errors:
-5 bus, timeout or not this host's device, -11 nothing yet, -16 busy, -22
malformed, -32 STALL.

## Authority

`CAPSULE_REQUIRED_CAPS = 0xB8018`: IPC, Memory, DeviceEnum, Driver, Mmio
and Dma. `CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, in a serial-debug build.
No Pio and no Irq: the controller is polled.

## Current implemented surface

Every EHCI controller on the machine is served, as one host
(server/many, shared with driver.uhci0 and driver.ohci0): an Intel ICH
or PCH has two. Their root ports are numbered on in PORT_STATUS; slots
are the host's, each bound to the controller whose port it is addressed
on. Each controller hands its own full- and low-speed ports to its own
companions, and the companions' drivers serve every one of those too.

Up to 15 root ports a controller, high-speed devices on them. Each
port is decided once connected: K-state (low speed) goes to the companion
unreset; after a reset, a port left disabled (full speed) goes too. The
async ring holds a head, address 0's control QH, a control QH per slot
and two bulk QHs per slot, all built before the controller runs; eight
interrupt QHs sit on the periodic list, each without a micro-frame until
armed. A QH is changed or taken back only out of the ring after the async
advance doorbell (or, periodic, after a frame with no micro-frame), so the
controller never holds a stale copy. Every wait is bounded by time.

## Wire format

A 20-byte header (magic `NXHC`, version, op, flags, reserved, request id,
payload length), then a 4-byte status and the op's payload, as driver.xhci0.

## State ownership

The capsule owns the claim, the grants, the schedule and the slot table.
Each slot is the device's USB address. Toggles live in the bulk and
interrupt QHs; control takes them from each qTD.

## Operating rules

Every port goes to this controller (CONFIGFLAG) before any is decided.
A slot given back takes its bulk QHs back at DATA0 and parks its
interrupt QHs. A port that turns out not high speed is the companion's.

## Privacy and persistence

Transfers pass through without being kept or logged. One console line at
start names the controller's vendor and device.

## Runtime lifecycle

With no controller it logs one line and exits `EXIT_ABSENT` before
claiming anything; a failed bring-up is retried on the shared bounded
schedule and ends in `EXIT_GAVE_UP`. Once up it serves until stopped.

## Failure model

A transfer's failure is its reply. Firmware that keeps the controller
past its bound, a refused grant or map, a controller that will not halt
or reset, or schedules that will not start fail the bring-up attempt,
which releases the claim and with it every grant.

## Release target

USB 2.0 sticks, disks and network adapters on QEMU's, VirtualBox's and
VMware's EHCI and on chipsets with EHCI and companion hosts.

## Release evidence

`ehci_proofs` on the host (34 tests). No boot yet.

## Release checklist

- Signed by the owner and added to the image's features.
- A QEMU `-device usb-ehci -device usb-storage` boot reading the stick.
- VirtualBox with USB 2.0 (EHCI + OHCI): a stick on EHCI, the tablet on
  OHCI.

## Explicit non-goals today

No isochronous transfers, interrupts, suspend or 64-bit addressing.
Hubs are served: OP_CONFIGURE_HUB (0x21) marks
a slot a hub, and OP_ADDRESS_ROUTED (0x20) addresses a device below one
without resetting the root port. A full- or low-speed device below a
high-speed hub (the rate-matching hub of an Intel PCH from the 5 series
on, or any USB 2 hub) is reached by split transactions through that
hub's TT: its QHs carry its speed, the TT's hub address and port, and an
interrupt QH completes in micro-frames 2 to 4 (sched/link.rs).

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/ehci_proofs && cargo test --release)`.
