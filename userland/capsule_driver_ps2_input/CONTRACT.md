# capsule_driver_ps2_input: contract

What the capsule claims and calls, and what it may do. What it keeps is in
[PRIVACY.md](PRIVACY.md); the summary in [README.md](README.md).

## Microkernel contract

The kernel publishes two synthetic platform records:

| Record | Purpose |
|---|---|
| PS/2 keyboard | owns the shared i8042 PIO window and IRQ1 |
| PS/2 AUX | publishes IRQ12 for the same capsule, with no PIO BAR |

The capsule claims both records. Only the keyboard record grants PIO, because
the i8042 data and command ports are shared. The AUX record exists so IRQ12 is
owned, acknowledged, and released through the same broker path as every other
interrupt source.

The capsule uses only microkernel calls:

| Syscall family | Use |
|---|---|
| `MkDeviceList` | discover PS/2 keyboard and AUX platform records |
| `MkDeviceClaim` | bind device authority to this capsule's pid |
| `MkPioGrant` | obtain the i8042 PIO range |
| `MkPioRead` / `MkPioWrite` | access data/status/command ports |
| `MkIrqBind` / `MkIrqAck` | own and acknowledge IRQ1 and IRQ12 |
| `MkInputEventPost` | post each decoded key, motion and button event to the kernel input ring (`src/keymap/post.rs`, `src/mouse/post.rs`) |
| `MkIpcRecv` / `MkIpcSend` | serve the driver endpoint |

No capsule may send to `driver.ps2_kbd0`: the kernel holds the endpoint to an
empty list (`src/services/registry/held.rs`), so only the kernel's own sends
reach it, by name or by pid. There is no inline port assembly in the capsule.
The static gate rejects raw `in`/`out` instructions so every byte crosses the
broker grant checks.

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Irq`, `Pio` and
`InputSource` (`CAPSULE_REQUIRED_CAPS = 0x358018`). It does not grant `Mmio`,
`Dma`, filesystem, graphics, network, admin, debug, or persistent storage
authority. `InputSource` admits `MkInputEventPost`. The kernel's consumer gate
for `MkInputEventDrain` and `MkInputEventWait` also accepts `InputSource`, so
this capsule could drain the keystroke ring as `input_router` does; nothing in
its code calls those.

```text
allowed:   i8042 PIO through MkPio*; IRQ1 and IRQ12 through MkIrq*;
           a bounded mouse packet queue, emptied on every pass;
           posts to the kernel input ring; IPC replies to the kernel
forbidden: persistent key or mouse logs; focus decisions;
           input-method policy; window routing
```
