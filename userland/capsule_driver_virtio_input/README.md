# capsule_driver_virtio_input

## Role

`capsule_driver_virtio_input` drives virtio-input (PCI 1af4:1052): QEMU's
`virtio-tablet-pci`, `virtio-keyboard-pci` and `virtio-mouse-pci`, every
function it finds. The tablet is what lets the pointer follow the host's
mouse without capturing it. Linux input events become NONOS input events on
the kernel input ring, which the input router reads.

```text
virtio-input (tablet / keyboard / mouse)
    |
    | event queue DMA (struct virtio_input_event)
    v
driver.virtio_input0 --MkInputEventPost--> kernel input ring --> input_router
```

## Microkernel contract

- `MkDeviceList` finds every virtio-input function (at most four).
- `MkDeviceClaim` claims each; `MkPciConfigWrite` turns on memory decode and
  bus mastering; `MkPciConfigRead` reads the virtio capabilities.
- `MkMmioMap` maps the common, notify and device configuration regions
  through the shared transport `userland/nonos_virtio`.
- `MkDmaMap` takes one coherent page per function: the event queue's split
  ring and an 8-byte buffer per descriptor.
- No `MkIrqBind`: the driver polls every 4 ms when idle, and the available
  ring asks for no interrupt.
- `MkInputEventPost` posts each event. The service endpoint
  `service:4278:driver.virtio_input0` is held to the kernel alone.

## Interface contract

No IPC ops: the capsule serves nothing. What it posts:

| Linux event | NONOS event |
|---|---|
| `EV_ABS` `ABS_X`/`ABS_Y`, then `SYN_REPORT` | `INPUT_KIND_POINTER_ABS`, x and y scaled from the axis range into 0..`INPUT_ABS_MAX` |
| `EV_REL` `REL_X`/`REL_Y` | `INPUT_KIND_POINTER_REL`, summed per report |
| `EV_REL` `REL_WHEEL`, `BTN_GEAR_UP`/`DOWN` | `INPUT_KIND_WHEEL`, delta_y +1 up |
| `BTN_LEFT`/`RIGHT`/`MIDDLE`/`SIDE`/`EXTRA` | `INPUT_KIND_BUTTON_DOWN`/`UP`, buttons 1 to 5 |
| `EV_KEY` below `BTN_MISC` | `INPUT_KIND_KEY_DOWN`/`UP`, the code the PS/2 driver posts for the same key |

## Authority

`IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Dma` and `InputSource`
(`CAPSULE_REQUIRED_CAPS = 0x2B8018`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Irq`, `Pio`, `Network` or `Crypto`.

```text
allowed:   virtio-input claims, their regions, one DMA page each, posting input events
forbidden: draining the input ring, reaching any capsule but the policy store, kernel drivers
```

## Privacy and persistence

Events are posted and forgotten: nothing is logged or kept beyond the keys
held down at the moment and the report being gathered.

## Runtime lifecycle

Per function: claim, PCI command, capability walk, map common/notify/device,
the DMA page; reset, ACKNOWLEDGE, DRIVER, features (VERSION_1, and
ACCESS_PLATFORM when offered), FEATURES_OK, the event queue (status queue
left off), the ABS_X and ABS_Y ranges from the configuration, every buffer
offered, DRIVER_OK, a kick. Then one loop drains every function's used ring,
offers each buffer again, and sleeps 4 ms when nothing came.

## Failure model

Each function is brought up on the shared bounded schedule
(`nonos_libc::bring_up`); one that fails is reset and its regions, page and
claim given back, and the others run on. Exit 2 with none present, 6 with
none up. A used element naming no offered buffer is skipped; `SYN_DROPPED`
throws the gathered report away.

## Current implemented surface

- Tablet, mouse and keyboard; any number of each up to four functions.
- Keys resolved through the active layout exactly as the PS/2 driver does
  (Linux codes 1 to 88 are scan set 1's make codes), Ctrl+Alt+Space cycling
  the layout, mute and power acting once per press.

## Wire format

Device side: struct virtio_input_event, le16 type, le16 code, le32 value.
Host side: the 32-byte `InputEvent` of `abi/wire.toml` `[input.event_v1]`.

## State ownership

The capsule owns its rings, the gathered report and the keyboard state. The
input router owns the cursor and where every event goes.

## Operating rules

- Absolute positions always leave the driver in 0..`INPUT_ABS_MAX`.
- A report's position is posted before its buttons.
- A release carries the code its press went down with.

## Release target

Under QEMU with `-device virtio-tablet-pci`, the pointer follows the host
mouse without a grab, and a virtio keyboard types as a PS/2 one does.

## Release evidence

`virtio_input_proofs` holds the ring over host memory, the event-to-report
translation, the post order, the keyboard against the PS/2 driver's own
tables, and libc's scaling. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_virtio_input` keys).
- `virtio_input_proofs` passes.
- QEMU `virtio-tablet-pci`: pointer follows the host mouse, clicks land.

## Explicit non-goals today

No LEDs (status queue), no multitouch, no force feedback.

## Verification

- Build: `make -B nonos-mk-driver-virtio-input`
- Proofs: `cd userland/virtio_input_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
