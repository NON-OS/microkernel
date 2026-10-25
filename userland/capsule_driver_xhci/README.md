# capsule_driver_xhci

## Role

`capsule_driver_xhci` is the USB 3 host-controller capsule. It owns xHCI
controller bring-up, controller rings, event processing, port status, and the
controller-owned slot lifecycle used by USB enumeration. USB class policy
belongs to separate HID, storage, audio, network, and hub capsules above it.

```text
driver.usb_hid0, usb_msc0, usb_audio0, USB NICs (the senders the kernel admits)
    |
    | controller service IPC
    v
driver.xhci0 -- MMIO / MSI-X or MSI / DMA broker grants --> xHCI controller
    |
    +-- command ring
    `-- event ring / ERST
```

## Microkernel contract

The capsule interacts with hardware only through broker grants:

- `MkDeviceList` locates the xHCI controller.
- `MkDeviceClaim` owns the controller claim.
- `MkMmioMap` maps the controller register window.
- `MkIrqBind` asks for one MSI-X vector, then for MSI on a controller without
  MSI-X (Intel PCH controllers). Each controller's event ring waits on its
  own grant. A refused bind leaves a zero grant and the driver polls the
  event ring; with a vector, the interrupt only parks it between polls
  (`src/setup/irq_bind.rs`).
- `MkDmaMap` and `MkDmaUnmap` allocate DCBAA, scratchpads, command ring,
  event ring, and ERST storage.
- `MkIpcRecv` and `MkIpcSend` serve `driver.xhci0` on
  `service:4206:driver.xhci0`.

The kernel does not enumerate USB devices, parse descriptors, or implement USB
class behavior. It grants resources and revokes them.

Only the USB class drivers may send to `driver.xhci0` (`driver.usb_hid0`,
`driver.usb_msc0`, `driver.usb_audio0` and the USB network drivers): the
kernel holds the endpoint to them (`src/services/registry/held_table.rs`), by
name and by pid, because the controller carries raw transfers to every device
behind it.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_STATUS` | controller register/ring/slot state; the DCBAA and scratchpad fields say only whether each exists, never where | 56-byte status |
| `OP_PORT_STATUS` | port state list; byte 1 says 0 free, 1 addressed, 2 claimed; a failed controller's ports show nothing attached | count plus 8-byte entries |
| `OP_ENABLE_SLOT` / `OP_DISABLE_SLOT` | issue xHCI Enable / Disable Slot | slot id / status word |
| `OP_ADDRESS_DEVICE` | reset a free root port and issue Address Device; `E_BUSY` if held | slot, port, speed, EP0 MPS |
| `OP_ADDRESS_ROUTED` (0x0020) | Address Device for a device below a hub: route string, speed and TT in the Slot Context, root port not reset; `E_INVAL` unless a hub holds the root port, the route is well formed and a TT names a hub's port for a full or low speed device | as `OP_ADDRESS_DEVICE` |
| `OP_CONFIGURE_HUB` (0x0021) | tell the controller an addressed slot is a hub: Configure Endpoint with only the Slot Context, its Hub flag, port count, TT think time and MTT | status word |
| `OP_GET_DEVICE_DESCRIPTOR` / `OP_GET_CONFIG_DESCRIPTOR` | run EP0 GET_DESCRIPTOR | the bytes the device sent |
| `OP_ALLOC_TRANSFER_RING` | give an interrupt endpoint of an addressed slot a transfer ring and a buffer for its whole payload per interval, its context typed IN or OUT by its address | status word |
| `OP_CONTROL_TRANSFER` | one EP0 control transfer for a class driver (for example SET_CONFIGURATION, SET_PROTOCOL, a sampling rate); an OUT request carries its wLength data bytes after the ten setup bytes, and is refused without them; an IN one carries none; up to 4096 bytes | status, the bytes the data stage moved (for an IN, those the device sent) |
| `OP_INTERRUPT_IN` | poll one interrupt-IN report, armed at the endpoint's payload (up to 3072 bytes) and handed up to the length asked; an OUT DCI is refused | report bytes, or pending |
| `OP_INTERRUPT_OUT` (0x25) | `slot, dci, len_le16, data`: one report, 1 to the endpoint's payload, to an interrupt OUT endpoint (an even DCI) whose ring ALLOC_TRANSFER_RING made, waited for; a failure brings the endpoint back first | moved le16, 0, 0 |
| `OP_CLAIM_INTERFACE` (0x26) | `slot, port, interface`: the caller drives that interface. A device whose driver claimed its interfaces may be shared: a second driver claims another. DISABLE_SLOT from a sharer gives back only its own claims; the slot is disabled with the last live holder, and refused (-16) to a caller holding none while another does | status word |
| `OP_SLOT_OF_PORT` (0x27) | `port`: the shared slot there, for a second driver to join without addressing or configuring the device again; -19 when none is shared | slot, 0, 0, 0, claimed le32 |
| `OP_USB_HELD` (0x28) | kernel only: the devices held for an answer | count, (slot, ticket le32)* |
| `OP_USB_AUTHORIZE` (0x29) | kernel only: `slot, allow, ticket le32`; settles only the held device on that slot with that ticket, -22 otherwise | status word |
| `OP_USB_POLICY` (0x2A) | kernel only: `0 allow, 1 ask, 2 deny` | status word |
| Shared slots | once any driver has claimed an interface of a slot, the slot's requests are served to its claimants only, and each pipe (an interrupt endpoint, a bulk pair, the isochronous IN or OUT) only to the claimant that configured it; others get -1, and a pipe another claimant configured is not configured again (-16). The device and first configuration descriptor reads stay open, for a driver deciding whether to join. A device no driver claimed an interface of is served only to the driver that enabled its slot: every request naming the slot, its first claim and DISABLE_SLOT among them, is refused (-1) to any other. Below the multiplexer each controller serves a slot only to the client that enabled it (`E_INVAL` otherwise), or to a claimant or a parked child's driver the multiplexer vouched for. The slots of a client that exited are disabled within a second, but a shared slot a live driver still claims and a child parked below a hub stay | -1 / -16 / -22 |
| `OP_CONFIGURE_BULK` / `OP_RESET_BULK` | add a bulk IN and OUT pair; recover one after a STALL | DCIs / status word |
| `OP_CONFIGURE_BULK2` (0x23) | a second bulk pair beside the first, on endpoints it does not hold (UAS's data pipes); bulk ops name it as pair 1 in their second byte, `OP_RESET_BULK` in bit 1 | DCIs |
| `OP_CONFIGURE_STREAMS` (0x24) | `slot, pair, dirs, log2`: SuperSpeed bulk streams on a pair's IN (dirs bit 0) and OUT (bit 1) endpoints, 2^log2 stream contexts (log2 1 to 4, within HCCPARAMS1 MaxPSASize), linear; BULK_IN and BULK_OUT then name a stream in bits 7:4 of their pair byte. A streamed endpoint is never polled or reset as one ring | usable streams le16 |
| `OP_BULK_OUT` / `OP_BULK_IN` | one transfer of up to 4096 bytes; a STALL replies `E_PIPE` | bytes moved, IN data |
| `OP_BULK_IN_POLL` | one look at the slot's armed bulk IN, its own page; `E_AGAIN` at once while nothing came | bytes moved, IN data |
| `OP_CONFIGURE_ISOCH` | add one isochronous endpoint, OUT or IN by its address, one of each per slot (interval, bursts and Mult by speed, CErr 0) | DCI |
| `OP_ISOCH_OUT` | queue up to 31 packets, a TD each, Start Isoch ASAP; waits for the batch before, so two are on the ring | that batch's code: success or missed service |
| `OP_PARK` (0x22) | from driver.usb_hub0, admitted on both the child's slot and the hub's (as its holder or a claimant; others get -1): a child it addressed below a hub, listed in `OP_PORT_STATUS` as a virtual port from 0x80 (nonos_usb_virt); `ADDRESS_DEVICE` naming a virtual port claims it, first claim wins, and the claimant's slot stands for the parked one | status |
| `OP_ISOCH_IN` | queue up to 31 service intervals, a TD each asking for its event; collects the batch before | that batch's code, its packet count, 31 u16 length slots, then the packets' bytes (at most 4034) |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8018`). It has no filesystem, input
routing, audio, network, graphics, admin, or debug authority.

```text
allowed:   xHCI claim, MMIO, MSI-X or MSI, controller DMA rings, controller IPC
forbidden: HID policy, mass-storage policy, CDC network policy, kernel USB
```

## Privacy and persistence

Controller rings and descriptors are runtime-only DMA state. The capsule does
not persist USB topology, device descriptors, keystrokes, storage payloads, or
audio data. Class capsules must request only the information they are
authorized to consume.

## Runtime lifecycle

The capsule claims xHCI, maps MMIO, asks for MSI-X or MSI, halts and resets the
controller, allocates controller DMA state, only then turns bus mastering on,
hands the controller its DCBAA and rings, starts it, runs a No-op command, and
serves controller status IPC. Callers may then enable a controller slot and must
disable it if enumeration fails or a device is removed. Each slot is its
enabling client's: no other may name it, and once that pid has exited, its slots
are disabled within a second and their ports come back. Teardown, and a bring-up
that fails once the controller was started, gives the controller up in one
order: halted (RUN cleared, HCH within a deadline), reset, bus mastering off,
the device released, and only then any DMA frame freed
(`src/setup/teardown.rs`). The kernel's `MkPciConfigWrite` ORs a Command write
Master Enable.

## Failure model

Setup failure rolls back every prior grant. Controller-not-ready and command
timeout paths abort promotion. With no xHCI controller in the device list the
capsule logs one line and exits `EXIT_ABSENT` (2) before claiming anything; a
controller that is present but fails setup is retried on the shared bounded
schedule (`nonos_libc::bring_up`: seven tries, sleeping between them), and
running out logs the last cause and exits `EXIT_GAVE_UP` (6). A completion wait
with no interrupt bound spins, then yields a bounded number of times, then
sleeps between polls, so a controller that stops answering costs wakeups, not a
core. Slot enable failures return deterministic protocol errors and do not mark
the slot table. Address Device owns its output device context, input context,
and EP0 ring through the slot table; Disable Slot clears the matching DCBAA
entry and drops those DMA grants. An Address Device that fails keeps them, and
the DCBAA entry, until Disable Slot, as it keeps what a failed controller may
still reach; a request buffer whose endpoint could not be brought back stays
until the controller has passed its TRBs. A command that times out is aborted,
the events that leaves are consumed, never taken for the next command's answer,
and one the controller never took up is made a No Op; Address Device is given up
after 500 ms, other commands after 5 s. A ring never holds more than 62 TRBs the
controller has not run. A controller whose USBSTS reads all ones or shows a Host
System or Host Controller Error, or whose command ring an abort cannot stop, is
refused (`E_NODEV`) and brought back by halt, reset, CNR clear and fresh rings,
at most three times (`src/setup/revive.rs`). After a revival every slot id a
client held is void: the client must enable a slot again before it uses one.
Device class requests remain outside this capsule.

## Current implemented surface

- Claims every xHCI controller on the machine, the chipset's first (a
  Thunderbolt or USB4 controller comes after it), and serves them all behind
  `driver.xhci0`: root ports are numbered across the controllers (the
  primary's from 1) and slot ids are the server's own, bound to a controller
  at Address Device (`src/server/mux.rs`). With one controller every request
  goes to it unchanged. Controllers after the first are best effort.
- Maps BAR0 up to 512 KiB, enough for the doorbells and the extended
  capability list (Intel puts DBOFF at 0x3000 and xECP near 0x8000), and
  binds one MSI-X vector when it can, or MSI on Intel PCH controllers, which
  have MSI only; INTx is disabled in the PCI command register either way. A
  controller whose port registers would lie past the mapping is refused.
- Takes the controller from firmware through USBLEGSUP inside the mapped
  window, waits for CNR, waits 1 ms after HCRST (an Intel host's need, as observed, unverified).
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
- Issues Disable Slot and frees the slot on success or Slot Not Enabled.
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
  complete in 5 s (Address Device: 500 ms).
- Addresses devices below hubs (route string, TT hub slot and port for low
  and full speed devices behind a high-speed hub) and sets a hub's Hub flag,
  Number of Ports, TT Think Time and MTT through Configure Endpoint; every
  value the hub class driver sends is bounded first (five tiers, the hub's
  port count, speeds the hub's ports carry).
- Gives interrupt-IN endpoints their whole payload per service interval:
  high-bandwidth high-speed and bursting SuperSpeed endpoints (Max Burst
  Size, Max ESIT Payload).
- Builds 32-byte or 64-byte xHCI input contexts from HCCPARAMS1.CSZ.
- Installs per-slot output contexts in DCBAA.
- Allocates a per-slot EP0 transfer ring.
- Issues Address Device and stores per-slot enumeration resources.
- Runs EP0 `GET_DESCRIPTOR(Device)` and returns the raw 18-byte descriptor.
- Runs EP0 `GET_DESCRIPTOR(Configuration)` and returns bounded raw bytes for
  class-capsule discovery.
- Runs class control transfers, interrupt-IN polls, and bulk IN and OUT
  transfers for the HID, mass-storage and network drivers, a polled bulk IN
  that answers at once (`OP_BULK_IN_POLL`), and isochronous OUT streams for
  the audio driver, two batches on a ring sized for both. A transfer event must name
  the slot and endpoint its TRB is on, and an interrupt-IN residual past the
  endpoint's payload hands up no bytes. A report that lands while another
  wait reads the ring is kept for its own poll, every one of them, and a
  disabled slot's are dropped; an IN data stage sets ISP, so a short answer
  is reported as the bytes the device sent.

## Wire format

Requests use the `NXHC` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Controller status
returns 56 bytes. Port status returns a count plus 8-byte port records.
Enable Slot returns a 4-byte payload whose first byte is the slot id. Disable
Slot takes a 1-byte request payload containing the slot id and returns only the
status word. Address Device takes `[slot_id, root_port]` and returns eight
bytes: slot, port, xHCI speed id, reserved, little-endian EP0 max packet size,
and reserved padding. Address Routed takes `[slot_id, root_port, speed,
route_le32, tt_hub_slot, tt_port]` and replies the same; Configure Hub takes
`[slot_id, ports, think_time, mtt]`. Device Descriptor takes `[slot_id]` and
returns the USB device descriptor bytes exactly as read from EP0, no more than
the device sent. Configuration Descriptor takes `[slot_id, index, len_lo,
len_hi]`, accepts any index, caps length at 4096 bytes, and prefixes the reply
body with the byte count the device sent. Alloc Transfer Ring takes `[slot_id,
ep_address, wMaxPacketSize_le16, bInterval, ss_burst]`, `ss_burst` being the
SuperSpeed Endpoint Companion's bMaxBurst (0 below SuperSpeed).

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
- A keyboard behind a high-speed hub, and a full-speed mouse behind it (TT), enumerate and deliver input.
- An Intel PCH controller binds MSI; its completion waits park on the interrupt.
- A full-speed device with a 64-byte EP0 reads its configuration (Evaluate Context ran).
- A device that STALLs SET_IDLE keeps working: its next control request succeeds.
- Halt, reset, CNR clear and run each finish within their one second bound; the NOOP command completes.
- Ports enumerate; USB keyboards, mice and mass-storage devices bind through the class drivers.
- With MSI-X refused, completion waits sleep after a short yield budget: the capsule does not sit at a full core while a device is slow.
- Well past the 64th event (a long typing session, or a mass-storage copy of a few hundred KiB), keys, mouse moves and transfers keep arriving: the event ring wraps in step with the controller and no wait times out.

## Not supported

- Configuration descriptors and control data stages past 4096 bytes.
- Bulk transfers past 4096 bytes in one request: the mass-storage client's
  wire contract is one page per request.
- EP0 request filtering: a class driver may send any request to the slots
  it enabled.
- The Intel xHCI/EHCI port switchover of 7, 8 and 9 series chipsets
  (Panther, Lynx and Wildcat Point, as observed, unverified):
  it writes PCI config 0xD0 and 0xD8, which the kernel's config-write
  allowlist refuses. On those machines ports the firmware left routed to
  EHCI stay there. Chipsets from the 100 series (6th generation Core) on,
  and Atom, Celeron and Pentium N parts such as Gemini Lake, have no EHCI.
- An isoch IN batch larger than one reply (video needs a shared-memory
  path), audio feedback endpoints, frame-scheduled isoch TDs, streams, USB
  power management (U1/U2, suspend).

## Explicit non-goals today

Hub descriptor parsing and port handling, HID reports, USB mass storage, USB
audio, CDC Ethernet, the pacing of an audio stream, and persistent USB
inventory do not live here. They are separate class-capsule
responsibilities above `driver.xhci0`.

## Verification

- Build: `make -B nonos-mk-driver-xhci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: xHCI must remain MMIO/IRQ/DMA broker-only and must not
  use raw PIO or kernel USB internals.
- Documentation check: this README is required by the driver docs gate.
- Proofs: `(cd userland/xhci_proofs && cargo test --release)`.
- Handbook: [drivers](../../docs/handbook/drivers.md).
