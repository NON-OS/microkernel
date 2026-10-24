# capsule_driver_usb_hub

Status: wired, not yet signed. The kernel mirror
(`src/hardware/usb_hub_capsule`) embeds and spawns it under
`nonos-capsule-driver-usb-hub` wherever the inventory sees a USB host,
after the host drivers. It has no owner certificate yet, so no image
carries it until the owner signs it.

## Role

`driver.usb_hub0` is the one walker of every USB hub. On every USB host
(`driver.xhci0`, `driver.ehci0`, `driver.ohci0`, `driver.uhci0`) it looks at
the root ports for hubs, brings each hub up, and walks its ports. A hub below
a hub is brought up the same way. Every other device below a hub is addressed
through the host and parked with it (OP_PARK). The host then lists it beside
its root ports as a virtual port, and a class driver claims it as it claims a
root port.

```text
hub port --reset by usb_hub0--> OP_ADDRESS_ROUTED --> OP_PARK --> host
class driver --ENABLE_SLOT, ADDRESS_DEVICE(virtual port)--> host
```

The hub class, the walk, the root-port pass and the parking are
`nonos_usb_hub`'s. This capsule holds the host wire client (`src/wire.rs`,
`src/control.rs`, `src/host.rs`, `src/link.rs`), the root port list
(`src/ports.rs`) and the loop (`src/run.rs`, `src/serve.rs`).

## Microkernel contract

- `MkServiceLookup` finds each USB host; every request to a device is an
  IPC call to its host.
- `MkIpcRecvFrom` serves `service:4516:driver.usb_hub0`; `MkIpcReply`
  answers.
- `MkIdleMs` and `MkUptimeMs` for port resets and debounce; `MkMmap` for
  the heap.
- `MkDebug` for the `[usb-hub]` lines, when Debug is granted.

## Interface contract

- `HEALTHCHECK` (1): 0. Any other op: -22.

The walker's work is all on the host wire: OP_PORT_STATUS, ENABLE_SLOT,
DISABLE_SLOT, ADDRESS_DEVICE (root ports), CONTROL_TRANSFER,
ALLOC_TRANSFER_RING (a hub's status endpoint), OP_ADDRESS_ROUTED (0x20),
OP_CONFIGURE_HUB (0x21) and OP_PARK (0x22).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18`: IPC, Memory. `CAPSULE_OPTIONAL_CAPS =
0x100`: Debug, in a serial-debug build only, for the log lines. No Driver,
DeviceEnum, Mmio, Irq, Dma or Pio: it reaches devices only through a host.

## Current implemented surface

- Hubs on root ports and up to five tiers below them, USB 2 and SuperSpeed
  (`SET_HUB_DEPTH`).
- A root port holding anything but a hub is given back at once and left
  alone until it empties or changes.
- A port that fails three times is left alone until its device leaves.
- A device that leaves its hub port: its slot is given back, and the host
  tells its claimant as a root-port detach does. A hub that leaves takes
  every device below it along.

## Wire format

The host wire: a 20-byte header (`NXHC`, version 1, op, flags, request id,
payload length), then the body. A reply's payload starts with a 4-byte
status.

## State ownership

The capsule owns the hubs it brought up and the slots of the children it
parked, until each is claimed (the claimant's from then on) or gone.

## Operating rules

No device access but through a host. A hub's status endpoint is claimed,
so no class driver takes the hub, but it is not read: each pass reads the
ports.

## Privacy and persistence

Nothing is kept past the process: the hubs and tries held in memory.

## Runtime lifecycle

Spawned once, after the USB hosts. It looks the hosts up for about two
seconds and leaves if there is none. Otherwise it walks every 250 ms and
answers its inbox in between.

## Failure model

A hub that cannot be brought up is given back. A child whose reset,
address or park fails is retried up to three times. A host that stops
answering keeps its hubs held until the next pass reads its ports.

## Release target

Keyboards, mice, sticks, webcams and Bluetooth adapters behind external,
internal and rate-matching hubs, on every USB host.

## Release evidence

`usb_hub_proofs` covers the hub class, the walk, parking and the
virtual-port table. Host lanes in `uhci_proofs`, `ohci_proofs` and
`ehci_proofs` cover the walk and the root-port pass over each driver's own
dispatch, with a modelled hub. No boot yet.

## Release checklist

- Signed by the owner and added to an image's features.
- A QEMU `-device usb-hub` boot with a keyboard and a stick behind it.

## Real hardware bring-up checklist

- A powered USB 2 hub on an xHCI port with a keyboard and a stick behind
  it: both reached, the stick by `driver.usb_msc0`.
- The same hub on an EHCI port (split transactions through its TT) and on
  a UHCI or OHCI port.
- A laptop's internal hub (webcam, Bluetooth, card reader) and an Intel
  PCH's rate-matching hub behind its EHCI.

## Explicit non-goals today

The hub's status endpoint is not read: each pass reads the ports. No hub
power budgeting, no port indicators, no selective suspend, no TT per port
(a multi-TT hub runs single-TT).

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/usb_hub_proofs && cargo test --release)` and the hub
lanes, `(cd userland/uhci_proofs && cargo test --release)`, likewise
`ohci_proofs` and `ehci_proofs`.
Handbook: [USB hubs](../../docs/drivers/usb/hubs.md).
