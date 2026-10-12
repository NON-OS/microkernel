# Capsule isolation: IPC: who may send to whom

Part of [Capsule isolation](capsule-isolation.md).

## IPC: who may send to whom

The kernel copies every message; capsules share no queue memory. A message is at
most `MAX_MESSAGE_SIZE`, 1 MiB (`src/ipc/nonos_channel/limits.rs:26`), and the
size is checked against `MAX_MESSAGE_SIZE` before anything is allocated
(`src/syscall/microkernel/ipc/send.rs:45-47`).

The `IPC` bit admits the IPC calls through `can_ipc`
(`src/syscall/contract/cap_table/mk_ipc.rs:38-45`). It does not admit any
particular destination. Each [endpoint](../overview/glossary.md#endpoint) states
the bits a sender must hold, and `caller_satisfies_endpoint` refuses a send
unless the sender holds all of them
(`src/syscall/microkernel/ipc/send_caps.rs:36-63`). It also refuses a name that
is not registered yet, so a capsule cannot win a race for a service that has not
started, and an endpoint whose requirement is zero, which was never finished
being set up. A refusal passes `[CAP-DENY] pid=<pid> ipc to <name> needs caps
<mask>, holds <mask>` to the same structured log, so in this release that line
is dropped too.

Some endpoints are held to named callers whatever bits a sender holds. `HELD`
lists the driver endpoints and the services that may reach each one
(`src/services/registry/held_table.rs:20-36`). A caller counts as one of the
named when the kernel registered that endpoint to it at spawn.

| Endpoint | Who may send |
|---|---|
| `driver.virtio_net0`, `driver.e1000_0`, `driver.rtl8169_0`, `driver.rtl8139_0` | `net.core`, `net.l2` |
| `driver.iwlwifi0`, `driver.rtl8821ce0` | `net.core`, Settings in any of its three windows, the setup wizard |
| `driver.xhci0` | `driver.usb_hid0`, `driver.usb_msc0` |
| `driver.i2c_pci0` | `driver.i2c_hid0` |
| `driver.virtio_gpu0` | `compositor` |
| `driver.hda0` | `audio.server` |
| `driver.ps2_kbd0`, `driver.usb_hid0`, `driver.i2c_hid0`, `driver.usb_msc0`, `driver.virtio_rng` | no capsule; only the kernel's own sends reach them |

A [peer list](../overview/glossary.md#peer-list) goes further. A capsule named
in `PEERS` may send only to the endpoints listed for it, whatever its bits; the
list has one entry, `shield_prover`, which may reach only `shield.core`
(`src/services/registry/peers.rs:27-31`).

A send straight to a process's [inbox](../overview/glossary.md#inbox) by pid
(`MkIpcSendToPid`) must pass the gate of every endpoint that process serves,
because they are all read from that one inbox (`inbox_admits` in
`src/services/registry/held.rs:62-70`).

The kernel names the sender. `IpcMessage::new` is given `proc.<pid>` of the
calling process (`src/syscall/microkernel/ipc/send_to_pid.rs:68-70`), so a
service can trust the sender's pid rather than a pid written in the payload. The
keyring relies on this; see [Device secrets and
keys](device-secrets-and-keys.md).

Keystrokes have their own rule. Posting to the input ring needs
`can_input_source`, which takes `InputSource` alone, held by the input drivers;
draining it, or blocking until it has events, needs `can_input_consumer`, which
takes `InputConsumer` alone, held by input_router. [Driver
capsules](../overview/glossary.md#driver-capsule) hold `Irq`, and neither `Irq`
nor `Admin` opens either gate
(`src/syscall/contract/cap_table/mk_surface.rs:30-36`). A driver capsule cannot
read what is typed into another program.
