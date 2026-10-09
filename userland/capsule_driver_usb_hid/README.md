# capsule_driver_usb_hid

## Role

`capsule_driver_usb_hid` is the USB HID class capsule. It asks `driver.xhci0`
for the devices on the root ports, binds boot keyboard and boot mouse
interfaces (and takes any other HID interface for a tablet), polls their
interrupt-IN endpoints through the controller, and posts each decoded key,
motion and button event to the kernel input ring with `MkInputEventPost`.

```text
USB keyboard / mouse / tablet
        |
        v
driver.xhci0 -- EP0 descriptors, control and interrupt-IN transfers
        |
        v
driver.usb_hid0 -- MkInputEventPost --> kernel input ring --> input_router
```

The capsule is not a host-controller driver. PCI enumeration, MMIO, IRQ,
DMA, xHCI command rings, event rings, port reset, slot lifecycle, endpoint
configuration, and interrupt-transfer scheduling remain in `driver.xhci0`.

## Microkernel contract

The manifest grants `IPC`, `Memory` and `InputSource`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
```

`InputSource` admits `MkInputEventPost`. The kernel's consumer gate for
`MkInputEventDrain` and `MkInputEventWait` also accepts `InputSource`, so this
capsule could drain the keystroke ring as `input_router` does; nothing in its
code calls those.

The capsule finds the controller with `MkServiceLookup` and reaches it with
`MkIpcCall`. Its own service receives requests with `MkIpcRecvFrom` and
replies with `MkIpcSendToPid`. It does not call `MkDeviceList`,
`MkDeviceClaim`, `MkMmioMap`, `MkIrqBind`, `MkDmaMap`, or `MkPioGrant`.

No capsule may send to `driver.usb_hid0`: the kernel holds the endpoint to an
empty list (`src/services/registry/held.rs`), so only the kernel's own sends
reach it. Before that, any capsule could poll keystrokes from it or feed it
reports it would then post as typed keys. The kernel mirror
(`src/userspace/capsule_driver_usb_hid`) embeds and spawns the capsule and
keeps a client for the service ops.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_PROBE_CONFIG` | raw USB configuration descriptor | HID bindings |
| `OP_FEED_KEYBOARD_REPORT` | 8-byte HID boot keyboard report | status |
| `OP_FEED_MOUSE_REPORT` | 3- or 4-byte HID boot mouse report | status |
| `OP_POLL_KEYS` | none | bounded key-event batch |
| `OP_POLL_MOUSE` | none | bounded mouse-event batch |
| `OP_GET_STATE` | none | counters and queue depths |

`OP_PROBE_CONFIG` validates the descriptor header, walks variable-length USB
descriptor records, and returns boot-protocol HID interfaces with interrupt IN
endpoints. It does not parse vendor report descriptors or infer policy from
manufacturer, product, or serial strings.

## Authority

The capsule has no hardware authority. Its capability mask is `0x200018`,
which is `IPC | Memory | InputSource`. It cannot enumerate PCI devices, claim
USB controllers, map registers, bind interrupts, allocate DMA, or touch I/O
ports.

```text
allowed:   descriptor parsing, report decoding, input-ring posts, IPC
forbidden: controller ownership, input routing, focus policy, persistence
```

## Privacy and persistence

The capsule keeps only runtime queues and counters. It does not persist
keystrokes, mouse movement, USB topology, serial numbers, product strings, or
descriptor snapshots. Polling drains event queues; process exit destroys the
remaining process-local memory under normal capsule teardown.

## Runtime lifecycle

The capsule initializes its heap, looks up `driver.xhci0`, and enumerates the
root ports through it: Enable Slot and Address Device, the configuration
descriptor (64 bytes at most), SET_CONFIGURATION, then for each HID interface
it binds SET_PROTOCOL (boot, for a boot interface) and a transfer ring for its
interrupt endpoint. A device it binds nothing on has its slot given back. It then loops: answer
one service request if one is waiting, read at most one 8-byte report from each
bound endpoint and post its events, and rescan the ports every 64 idle polls
while nothing is bound or a port another class driver held was let go
(`src/orchestrator/poll/run.rs`).

## Failure model

Malformed descriptors return `E_INVAL`. Valid descriptors without boot HID
interfaces return `E_NO_HID`. Unknown operations return `E_BAD_OP`. Oversized
or malformed reports are rejected and do not mutate event state. On the
interrupt path as over IPC, a keyboard report that is not exactly eight bytes
changes nothing, and a rollover report (an error code in a key slot) keeps
the held keys held and makes no key event.

The host controller is looked up through `MkServiceLookup` up to 100 times,
20 ms apart and asleep in between; with no `driver.xhci0` after that the
capsule logs one line and exits `EXIT_ABSENT` (2). It used to retry with
yields and no bound, a spin for the life of a machine without xHCI.

## Current implemented surface

- USB configuration descriptor validation.
- Boot HID keyboard and mouse interface discovery.
- Interrupt IN endpoint extraction.
- Boot keyboard report normalization.
- Boot mouse report normalization.
- Bounded key and mouse event queues.
- State counters for descriptor probes and report ingestion.
- Live enumeration and interrupt-IN polling through `driver.xhci0`.
- Tablet (absolute) positions, clamped to the logical range.
- Key, motion and button events posted to the kernel input ring.
- Kernel-side client for health, descriptor probe, report feed, event poll, and
  state readback.

## Wire format

Requests use the `NUHI` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word.

`OP_PROBE_CONFIG` returns a 32-bit binding count followed by 8-byte binding
records:

```text
kind, interface_number, endpoint_address, interval, max_packet_size_le16, pad[2]
```

Key events are 8 bytes:

```text
hid_usage, ascii, modifiers, pressed, pad[4]
```

Mouse events are 8 bytes:

```text
dx_le16, dy_le16, wheel_i8, buttons, flags, pad[1]
```

## State ownership

The capsule owns the bound endpoints, the keyboard previous-report snapshot,
Caps Lock state, keyboard event queue, mouse button snapshot, mouse event
queue, and diagnostic counters. `input_router` and the compositor own routing,
focus, cursor position, acceleration, gestures, and delivery.

## Operating rules

- Do not add MMIO, PIO, IRQ, DMA, or device-enumeration authority here.
- Do not parse HID reports in the kernel.
- Do not persist keys, movement, descriptor bytes, or USB identity strings.
- Keep endpoint scheduling in `driver.xhci0`.
- Keep focus, cursor, and gesture policy above this capsule.

## Release target

The finished USB HID path is a signed class-driver chain:

```text
driver.xhci0 -> driver.usb_hid0 -> kernel input ring -> input_router -> compositor
```

Interrupt endpoint configuration and polling are in place; what is left is a
boot that shows them on real devices.

## Release evidence

Release evidence requires a QEMU `qemu-xhci` boot with a USB keyboard and USB
pointer device, descriptor classification on serial, key press/release polling,
mouse delta polling, malformed-descriptor rejection, and no grant requests
outside `IPC | Memory | InputSource`.

The host proofs (`userland/usb_proofs`) cover the descriptor walk, the binding
rules and the report decodes. No boot log for this driver is committed.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm README, capability boundary, and matrix row.
- Kernel profile `microkernel-driver-usb-hid` resolves with the capsule client.
- Descriptor parser rejects malformed lengths and oversized payloads.
- Keyboard and mouse report feeds produce bounded event batches.
- QEMU xHCI live interrupt-report validation passes.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without xHCI the capsule looks `driver.xhci0` up for about two seconds, asleep between tries, then exits `EXIT_ABSENT` (2); its `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so confirm by the exit and by no CPU time accruing.
- Boot keyboards, mice and tablets report through xHCI; key repeat and modifiers behave; the layout cycles on Ctrl+Alt+Space.
- Hot-plugging a device after boot binds it on the next rescan.
- A keyboard or mouse whose interrupt endpoint has packets larger than 8 bytes (16, 32 or 64 are common) delivers keys and motion; each read asks `driver.xhci0` for at most 8 bytes, the most it accepts.
- Holding more keys than the keyboard can report, then letting go, leaves no key stuck and types no key twice.

## Explicit non-goals today

Report-descriptor parsing, vendor HID layouts, multitouch, LED output
reports, endpoint scheduling, USB hub traversal, and compositor focus policy
do not live in this slice. A non-boot HID interface is taken for a tablet, and
a configuration descriptor over 64 bytes is refused.

## Verification

- Build: `make -B nonos-mk-driver-usb-hid`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-usb-hid`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: no `Driver`, `DeviceEnum`, `Mmio`, `Irq`, `Dma`, or `Pio`
  capability appears in `Capsule.mk`.
- Host proofs: `cd userland/usb_proofs && cargo test --release` runs the
  shipping descriptor walk, binding rules, interrupt read length, and the
  keyboard, mouse, tablet and button decodes against malformed and hostile
  input: exhaustive edges, a 200,000-round seeded fuzz of each surface with
  the binding rules checked against a second statement of them, and a named
  boundary set. Breaking any of those rules in this source fails a proof.
  `userland/usb_proofs/README.md` says what each proves.
- Runtime proof target: QEMU `qemu-xhci` keyboard and pointer device, with
  descriptor classification and event polling confirmed on serial.
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [compositor](../../docs/handbook/desktop/compositor.md).

## Real hardware notes

- The configuration descriptor is read whole (its header, then
  wTotalLength bytes, up to 512), and SET_CONFIGURATION uses its own
  bConfigurationValue.
- Boot keyboards and mice get SET_PROTOCOL(boot); keyboards also get
  SET_IDLE(0). A device that STALLs either is still bound.
- A device with a boot keyboard or mouse interface binds only those: its
  media-key, system-control and vendor interfaces are not taken as a tablet.
- Every interrupt-IN endpoint of a composite device is polled on its own
  DCI, so a wireless keyboard and mouse receiver gives both.
- USB hubs are not supported. A device plugged into a hub, a dock or a
  monitor does nothing; on finding a hub the driver logs one line saying so,
  which reaches the console only with the Debug capability.

