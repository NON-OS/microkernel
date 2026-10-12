# capsule_driver_usb_hid

## Role

`capsule_driver_usb_hid` is the USB HID class capsule. It asks `driver.xhci0`
for the devices on the root ports and behind hubs, binds boot keyboard and
boot mouse interfaces, and binds any other HID interface only when its report
descriptor names keys, media keys or a pointer it reads. It polls their
interrupt-IN endpoints through the controller and posts each decoded key,
motion and button event to the kernel input ring with `MkInputEventPost`.

```text
USB keyboard / mouse / tablet / touchscreen / media keys
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
empty list (`src/services/registry/held.rs`), and the service loop drops a
message from sender 0, which every kernel send is. The service has no op
that feeds a report in or reads a key back out: the ops that did (0x0003 to
0x0006) are gone, answered `E_BAD_OP` without a body and `E_INVAL` with one,
as any unknown op is, so the endpoint is no keystroke injection or readback
path whoever comes to hold it. The kernel mirror
(`src/userspace/capsule_driver_usb_hid`) embeds and spawns the capsule and
keeps a client for health and state readback only.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_PROBE_CONFIG` | raw USB configuration descriptor | HID bindings |
| `OP_GET_STATE` | none | counters |

`OP_PROBE_CONFIG` validates the descriptor header, walks variable-length USB
descriptor records, and returns every HID interface with an interrupt IN
endpoint: kind 1 and 2 are boot keyboards and mice, kind 3 one in report
protocol, which the driver binds only if its report descriptor is understood.
It does not infer policy from manufacturer, product, or serial strings.

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

The capsule keeps no key or pointer event: each is posted to the kernel input
ring and dropped (plan/50 C8). It used to keep the first 64 key events, with
their ASCII bytes, in a queue only the removed poll op drained, so the boot
password stayed in its heap. What it keeps per bound interface is the keys
and buttons down now and the key repeating, and counters. It does not
persist keystrokes, mouse movement, USB topology, serial numbers, product
strings, or descriptor snapshots.

## Runtime lifecycle

The capsule initializes its heap, looks up `driver.xhci0`, and enumerates the
root ports through it: Enable Slot and Address Device, the configuration
descriptor (up to 512 bytes), SET_CONFIGURATION, then for each HID interface
it can read, SET_PROTOCOL(boot) for a boot interface, or the report
descriptor for any other and for a boot interface that refuses it, and a
transfer ring for its interrupt endpoint. A
device it binds nothing on has its slot given back. It then loops
(`src/orchestrator/poll/run.rs`): read each endpoint that is due, at most 16
reports, into the state of its own interface; tick key repeat; look at
every root and hub port again after 64 ms with no report, and at hub ports
mid bring-up every 10 ms; then sleep on its inbox until the next endpoint is
due, at most 16 ms, answering a request if one comes. An endpoint is asked
as often as the controller serves it, and once quiet for a second every
8 ms. A new hub's ports are looked at once their power is good, and a
device below a hub is debounced, reset and addressed one step per look, each
wait a deadline the loop checks, never a sleep. Each root or hub port is
tried at most three times a minute, however often its device reconnects.

## Failure model

Malformed descriptors return `E_INVAL`. Valid descriptors without HID
interfaces return `E_NO_HID`. A report descriptor is walked under hard
bounds (512 items, 8 nested collections, 4 pushed globals, 8 report IDs,
report size times count and each report at most 2048 bits) and refused
whole when it passes one, is cut inside an item or longer than one read,
or leaves a collection open; its interface is then left unbound. An unknown
operation is answered `E_BAD_OP` without a body and `E_INVAL` with one.
Oversized or malformed reports are rejected and do not mutate event state.
On the interrupt path, a boot keyboard report that is not exactly eight bytes
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
- Boot keyboard and boot mouse report normalization.
- Bounded report descriptor parser (HID 1.11, 6.2.2) into a small layout:
  keyboard modifiers and key slots, mouse buttons, X, Y and wheel, absolute
  X and Y for tablets, pens and touchscreens (the first contact), Mute,
  Volume and Power on consumer and system control collections, with report
  IDs.
- Keyboard, mouse and pointer state per bound interface, released whole
  when the device leaves.
- Hubs, with port bring-up stepped from the poll loop.
- State counters for descriptor probes and report ingestion.
- Key, motion and button events posted to the kernel input ring.

## Wire format

Requests use the `NUHI` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word.

`OP_PROBE_CONFIG` returns a 32-bit binding count followed by 8-byte binding
records:

```text
kind, interface_number, endpoint_address, interval, max_packet_size_le16, pad[2]
```

`OP_GET_STATE` returns 48 bytes: configurations probed, keyboard and mouse
reports read (u64 each), 8 zero bytes where the removed queue depths were,
then the key and pointer posts the input ring refused (u64 each).

## State ownership

The capsule owns the bound endpoints and, for each, its own interface state:
the keys its last report held, its modifiers and Caps Lock, the key it
repeats, and its buttons; and diagnostic counters. A report from one device
never changes another's state. `input_router` and the compositor own routing,
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
pointer device, descriptor classification on serial, key presses, releases and
mouse motion reaching the input ring, malformed-descriptor rejection, and no grant requests
outside `IPC | Memory | InputSource`.

The host proofs (`userland/usb_proofs`) cover the descriptor walk, the binding
rules and the report decodes. No boot log for this driver is committed.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm README, capability boundary, and matrix row.
- Kernel profile `microkernel-driver-usb-hid` resolves with the capsule client.
- Descriptor parser rejects malformed lengths and oversized payloads.
- Hostile report descriptors are refused whole and their interfaces left
  unbound.
- QEMU xHCI live interrupt-report validation passes.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without xHCI the capsule looks `driver.xhci0` up for about two seconds, asleep between tries, then exits `EXIT_ABSENT` (2); its `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so confirm by the exit and by no CPU time accruing.
- Boot keyboards, mice and tablets report through xHCI; key repeat and modifiers behave; the layout cycles on Ctrl+Alt+Space.
- Hot-plugging a device after boot binds it on the next rescan.
- A keyboard or mouse whose interrupt endpoint has packets larger than 8 bytes (16, 32 or 64 are common) delivers keys and motion; each read asks `driver.xhci0` for at most 8 bytes, the most it accepts.
- Holding more keys than the keyboard can report, then letting go, leaves no key stuck and types no key twice.
- A keyboard's volume and mute keys work; a gamepad or UPS moves and clicks nothing.
- Pulling out a keyboard with a key held stops its repeat and releases the key.

## Explicit non-goals today

Vendor HID layouts, bitmap (n-key rollover) keyboards, multitouch past the
first contact, touchpads, reports over 8 bytes (the most one
`driver.xhci0` read returns), report descriptors over 512 bytes, endpoint
scheduling, USB suspend and compositor focus policy do not live in this
slice. Boot interfaces stay in boot protocol. Caps, Num and Scroll Lock
LEDs wait on D10: they need a SET_REPORT whose one-byte data stage
`driver.xhci0`'s control transfer cannot carry yet (it takes the setup
packet alone and sends a zeroed data stage), and D10's EP0 contract work
adds OUT data stages. The loop still asks each endpoint over IPC; completions sent by
`driver.xhci0` would let it sleep until one comes.

## Verification

- Build: `make -B nonos-mk-driver-usb-hid`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-usb-hid`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: no `Driver`, `DeviceEnum`, `Mmio`, `Irq`, `Dma`, or `Pio`
  capability appears in `Capsule.mk`.
- Host proofs: `cd userland/usb_proofs && cargo test --release` runs the
  shipping descriptor walks, binding rules, interrupt read length and pace,
  the keyboard, mouse, report-descriptor and button decodes, per-interface
  state, unplug release and hub port bring-up against malformed and hostile
  input: exhaustive edges, seeded fuzz of each surface, and a named boundary
  set. Breaking any of those rules in this source fails a proof.
  `userland/usb_proofs/README.md` says what each proves.
- Runtime proof target: QEMU `qemu-xhci` keyboard and pointer device, with
  descriptor classification and input events confirmed on serial.
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [compositor](../../docs/handbook/desktop/compositor.md).

## Real hardware notes

- The configuration descriptor is read whole (its header, then
  wTotalLength bytes, up to 512), and SET_CONFIGURATION uses its own
  bConfigurationValue.
- Boot keyboards and mice get SET_PROTOCOL(boot); keyboards also get
  SET_IDLE(0), whose STALL is fine. One that STALLs SET_PROTOCOL stays in
  report protocol and is read through its report descriptor, so a report ID
  is never taken for modifier keys; it binds only if that is understood.
- A device's interfaces in report protocol (media keys, system control, a
  report-protocol mouse or touchscreen) bind only through their report
  descriptors; a vendor or gamepad interface binds nothing.
- Every interrupt-IN endpoint of a composite device is polled on its own
  DCI, so a wireless keyboard and mouse receiver gives both.
- Devices behind hubs, docks and monitors bind through the hub's ports;
  what each port came to is logged, which reaches the console only with the
  Debug capability.

