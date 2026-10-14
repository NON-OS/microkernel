# capsule_driver_i2c_hid: contract

What the capsule may call, what it answers, and what it keeps. The summary is
in [README.md](README.md).

## Microkernel contract

The manifest grants `IPC`, `Memory` and `InputSource`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
CAPSULE_OPTIONAL_CAPS = 0x100
```

The optional bit is `Debug`, which only a `capsule-serial-debug` build grants.
`InputSource` admits `MkInputEventPost`. The kernel's consumer gate for
`MkInputEventDrain` and `MkInputEventWait` also accepts `InputSource`, so this
capsule could drain the keystroke ring as `input_router` does; nothing in its
code calls those.

The capsule resolves `driver.i2c_pci0` with `MkServiceLookup`, sends bounded IPC
requests with `MkIpcSend`, receives replies with `MkIpcRecv`, and serves callers
with `MkIpcRecvFrom` plus `MkIpcSendToPid`. No capsule may send to
`driver.i2c_hid0`: the kernel holds the endpoint to an empty list
(`src/services/registry/held.rs`). The serving loop answers only a non-zero
sender (`src/server/runner/run.rs`), so at this commit the three ops below are
reachable by nobody; the driver's work is the input it posts.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | found flag, address, probe count |
| `OP_PROBE` | none | refreshed descriptor state |
| `OP_DESCRIPTOR` | none | cached HID descriptor bytes |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`. Requests
use the `NHID` capsule header, version `1`, and the shared 20-byte driver
envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian.

## Authority

The capsule may talk to the I2C controller capsule over IPC and post events to
the kernel input ring. It has no PCI, MMIO, IRQ, DMA, PIO, filesystem, network,
display, or focus-routing authority. Through `driver.i2c_pci0` it reaches only
the addresses that controller has in reach, and once it has read the pad's HID
descriptor there, the pad's alone; every other transfer is refused before the
bus.

## Privacy and persistence

The capsule stores no touch events, gestures, keystrokes, sensor samples, or
history. It keeps the I2C address, probe counter, HID and report descriptor
state, the last button mask and the current gesture in volatile memory; each
report is decoded and posted, then dropped.

## State ownership

`driver.i2c_hid0` owns HID-over-I2C class state: descriptor bytes, selected I2C
address, probe counters, and the decode state. `driver.i2c_pci0` owns controller
registers, bus transactions and the reach. `input_router` owns focus, event
routing, and policy.

## Operating rules

- Do not map hardware or request Driver/DeviceEnum/Mmio/Irq/Dma/Pio caps.
- Do not persist reports, touches, gestures, or keystrokes.
- Do not route input focus here.
- Keep every transfer bounded by the I2C controller capsule limits.
