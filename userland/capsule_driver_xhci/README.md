# capsule_driver_xhci

## Role

`capsule_driver_xhci` is the USB 3 host-controller capsule. It owns xHCI
controller bring-up, controller rings, event processing, port status, and the
controller-owned slot lifecycle used by USB enumeration. USB class policy
belongs to separate HID, storage, audio, network, and hub capsules above it.

```text
driver.usb_hid0, driver.usb_msc0 (the only senders the kernel admits)
    |
    | controller service IPC
    v
driver.xhci0 -- MMIO / MSI-X / DMA broker grants --> xHCI controller
    |
    +-- command ring
    `-- event ring / ERST
```

## Microkernel contract

The capsule interacts with hardware only through broker grants:

- `MkDeviceList` locates the xHCI controller.
- `MkDeviceClaim` owns the controller claim.
- `MkMmioMap` maps the controller register window.
- `MkIrqBind` asks for one MSI-X vector. A refused bind leaves a zero grant
  and the driver polls the event ring; with a vector, the interrupt only parks
  it between polls (`src/setup/irq_bind.rs`).
- `MkDmaMap` and `MkDmaUnmap` allocate DCBAA, scratchpads, command ring,
  event ring, and ERST storage.
- `MkIpcRecv` and `MkIpcSend` serve `driver.xhci0` on
  `service:4206:driver.xhci0`.

The kernel does not enumerate USB devices, parse descriptors, or implement USB
class behavior. It grants resources and revokes them.

Only `driver.usb_hid0` and `driver.usb_msc0` may send to `driver.xhci0`: the
kernel holds the endpoint to them (`src/services/registry/held.rs`), by name
and by pid, because the controller carries raw transfers to every device
behind it.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_STATUS` | controller register/ring/slot state | 56-byte status |
| `OP_PORT_STATUS` | port state list; byte 1 says 0 free, 1 addressed, 2 claimed | count plus 8-byte entries |
| `OP_ENABLE_SLOT` / `OP_DISABLE_SLOT` | issue xHCI Enable / Disable Slot | slot id / status word |
| `OP_ADDRESS_DEVICE` | reset a free root port and issue Address Device; `E_BUSY` if held | slot, port, speed, EP0 MPS |
| `OP_GET_DEVICE_DESCRIPTOR` / `OP_GET_CONFIG_DESCRIPTOR` | run EP0 GET_DESCRIPTOR | raw descriptor bytes |
| `OP_ALLOC_TRANSFER_RING` | give an endpoint of an addressed slot a transfer ring | status word |
| `OP_CONTROL_TRANSFER` | one EP0 control transfer for a class driver (for example SET_CONFIGURATION, SET_PROTOCOL) | status, IN data |
| `OP_INTERRUPT_IN` | poll one interrupt-IN report of at most `HID_REPORT_MAX` bytes | report bytes, or pending |
| `OP_CONFIGURE_BULK` / `OP_RESET_BULK` | add a bulk IN and OUT pair; recover one after a STALL | DCIs / status word |
| `OP_BULK_OUT` / `OP_BULK_IN` | one transfer of up to 4096 bytes; a STALL replies `E_PIPE` | bytes moved, IN data |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8018`). It has no filesystem, input
routing, audio, network, graphics, admin, or debug authority.

```text
allowed:   xHCI claim, MMIO, MSI-X, controller DMA rings, controller IPC
forbidden: HID policy, mass-storage policy, CDC network policy, kernel USB
```

## Privacy and persistence

Controller rings and descriptors are runtime-only DMA state. The capsule does
not persist USB topology, device descriptors, keystrokes, storage payloads, or
audio data. Class capsules must request only the information they are
authorized to consume.

## Runtime lifecycle

The capsule claims xHCI, maps MMIO, asks for MSI-X, allocates controller DMA state,
halts and resets the controller, starts command/event rings, runs a No-op
command, and serves controller status IPC. Runtime enumeration callers may then
enable a controller slot and must disable that slot if enumeration fails or a
device is removed. Teardown releases DMA, IRQ, MMIO, and claim grants.

## Failure model

Setup failure rolls back every prior grant. Controller-not-ready and command
timeout paths abort promotion. With no xHCI controller in the device list the
capsule logs one line and exits `EXIT_ABSENT` (2) before claiming anything; a
controller that is present but fails setup is retried on the shared bounded
schedule (`nonos_libc::bring_up`: seven tries, sleeping between them), and
running out logs the last cause and exits `EXIT_GAVE_UP` (6). A completion
wait with no interrupt bound spins, then yields a bounded number of times,
then sleeps between polls, so a controller that stops answering costs
wakeups, not a core. Slot enable failures return deterministic
protocol errors and do not mark the slot table. Address Device owns its output
device context, input context, and EP0 ring through the slot table; Disable Slot
clears the matching DCBAA entry and drops those DMA grants. Device class
requests remain outside this capsule.

## Current implemented surface

- Claims every xHCI controller on the machine, the chipset's first (a
  Thunderbolt or USB4 controller comes after it), and serves them all behind
  `driver.xhci0`: root ports are numbered across the controllers (the
  primary's from 1) and slot ids are the server's own, bound to a controller
  at Address Device (`src/server/mux.rs`). With one controller every request
  goes to it unchanged. Controllers after the first are best effort.
- Maps BAR0 up to 512 KiB, enough for the doorbells and the extended
  capability list (Intel puts DBOFF at 0x3000 and xECP near 0x8000), and
  binds one MSI-X vector when it can. Intel PCH controllers have MSI only,
  so they poll; INTx is disabled in the PCI command register either way.
- Takes the controller from firmware through USBLEGSUP inside the mapped
  window, waits for CNR, waits 1 ms after HCRST (Linux XHCI_INTEL_HOST).
- Reads the Supported Protocol capabilities, so each root port is known as
  USB 2 or USB 3, and powers every port at start.
- Serves controllers without 64-bit addressing (AC64 clear) with DMA kept
  below 4 GiB, and sizes scratchpads by the PAGESIZE register.
- Allocates DCBAA, scratchpads, command ring, event ring, and ERST.
- Halts and resets the controller.
- Waits for CNR clear and starts the controller.
- Issues a No-op command.
- Serves controller and port status over IPC.
- Issues Enable Slot and returns the controller-assigned slot id.
- Tracks enabled slots in capsule-local bounded state.
- Issues Disable Slot and clears the local slot table on success.
- Debounces a connect for 100 ms, resets USB 2 ports (with 50 ms
  recovery), uses an already enabled USB 3 port as it is, warm-resets a USB
  3 link in Inactive or Compliance, and never writes PED back as one.
- Settles a full-speed device's EP0 max packet size from the first 8 bytes
  of its device descriptor and Evaluate Context.
- Converts bInterval to the controller's Interval by speed, configures
  several interrupt-IN endpoints per device (a keyboard and mouse receiver),
  and copies the output Slot Context into every Configure Endpoint.
- Recovers an endpoint after a STALL, other error or timeout (Reset or Stop
  Endpoint, then Set TR Dequeue Pointer), and aborts a command that does not
  complete in 5 s.
- Builds 32-byte or 64-byte xHCI input contexts from HCCPARAMS1.CSZ.
- Installs per-slot output contexts in DCBAA.
- Allocates a per-slot EP0 transfer ring.
- Issues Address Device and stores per-slot enumeration resources.
- Runs EP0 `GET_DESCRIPTOR(Device)` and returns the raw 18-byte descriptor.
- Runs EP0 `GET_DESCRIPTOR(Configuration)` and returns bounded raw bytes for
  class-capsule discovery.
- Runs class control transfers, interrupt-IN polls, and bulk IN and OUT
  transfers for the HID and mass-storage drivers. A transfer event must name
  the slot and endpoint its TRB is on, and an interrupt-IN residual past the
  request hands up no bytes.

## Wire format

Requests use the `NXHC` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Controller status
returns 56 bytes. Port status returns a count plus 8-byte port records.
Enable Slot returns a 4-byte payload whose first byte is the slot id. Disable
Slot takes a 1-byte request payload containing the slot id and returns only the
status word. Address Device takes `[slot_id, root_port]` and returns eight
bytes: slot, port, xHCI speed id, reserved, little-endian EP0 max packet size,
and reserved padding. Device Descriptor takes `[slot_id]` and returns the USB
device descriptor bytes exactly as read from EP0. Configuration Descriptor takes
`[slot_id, index, len_lo, len_hi]`, currently accepts index `0`, caps length at
512 bytes, and prefixes the reply body with the returned byte count.

## State ownership

The capsule owns MMIO mapping, IRQ grant, DCBAA, scratchpads, command ring,
event ring, ERST, port state snapshot, enabled-slot table, and controller
command state. USB class capsules own device-class policy.

## Operating rules

- Do not parse HID, hub, mass-storage, audio, or CDC descriptors here.
- Keep descriptor parsing out of the kernel.
- Bound controller-reported port lists.
- Pair every successful Enable Slot with Disable Slot if Address Device or
  descriptor fetch fails.
- Roll back DMA, IRQ, MMIO, and claim grants on setup failure.

## Release target

The finished xHCI capsule is a signed USB host-controller service with slot
enable/disable, Address Device, endpoint-zero control transfers, event processing,
port-change handling, reset recovery, and class-capsule handoff. It owns the
controller mechanics only; HID, storage, audio, CDC, and hub policy stay in
separate USB class capsules.

## Release evidence

Release requires QEMU `qemu-xhci` validation, No-op completion proof, Enable Slot
/ Disable Slot proof, port-change proof, endpoint-zero GetDescriptor validation,
teardown DMA revocation, and class capsule handoff tests.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU xHCI No-op validation passes.
- Slot enable/disable, port-change, and GetDescriptor validation pass.
- Teardown proof shows DMA/IRQ/MMIO/device claim revocation.
- HID or mass-storage class capsule handoff is proven over IPC.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, the bounded retry, and the event ring against a producer written from the specification (`userland/xhci_proofs`, `src/event_ring/`); these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- The USBLEGSUP handoff completes on firmware that owns the controller.
- On a machine with a Thunderbolt or second xHCI, ports on both controllers enumerate.
- A full-speed device with a 64-byte EP0 reads its configuration (Evaluate Context ran).
- A device that STALLs SET_IDLE keeps working: its next control request succeeds.
- Halt, reset, CNR clear and run each finish within their one second bound; the NOOP command completes.
- Ports enumerate; USB keyboards, mice and mass-storage devices bind through the class drivers.
- With MSI-X refused, completion waits sleep after a short yield budget: the capsule does not sit at a full core while a device is slow.
- Well past the 64th event (a long typing session, or a mass-storage copy of a few hundred KiB), keys, mouse moves and transfers keep arriving: the event ring wraps in step with the controller and no wait times out.

## Not supported

- USB hubs. Nothing configures a hub or addresses the devices behind it, so
  a keyboard, mouse or USB stick plugged into a hub, a dock or a monitor's
  USB ports does nothing. Internal devices a laptop wires behind an internal
  hub are not seen either. The HID driver says so in one line when it finds
  a hub on a root port (`[usb] port N: USB hub found ...`); that line reaches
  the serial console only when the capsule holds the Debug capability, and
  nothing yet shows it on screen.
- The Intel xHCI/EHCI port switchover of 7, 8 and 9 series chipsets
  (Panther, Lynx and Wildcat Point; Linux `usb_enable_intel_xhci_ports`):
  it writes PCI config 0xD0 and 0xD8, which the kernel's config-write
  allowlist refuses. On those machines ports the firmware left routed to
  EHCI stay there. Chipsets from the 100 series (6th generation Core) on,
  and Atom, Celeron and Pentium N parts such as Gemini Lake, have no EHCI.
- Isochronous transfers, streams, USB power management (U1/U2, suspend).

## Explicit non-goals today

Hub traversal, HID reports, USB mass storage, USB audio, CDC Ethernet,
isochronous scheduling, and persistent USB inventory do not live here. They are
separate class-capsule responsibilities above `driver.xhci0`.

## Verification

- Build: `make -B nonos-mk-driver-xhci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: xHCI must remain MMIO/IRQ/DMA broker-only and must not
  use raw PIO or kernel USB internals.
- Documentation check: this README is required by the driver docs gate.
- Proofs: `(cd userland/xhci_proofs && cargo test --release)`.
- Handbook: [drivers](../../docs/handbook/drivers.md).
