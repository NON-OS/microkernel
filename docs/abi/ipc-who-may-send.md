# IPC: who may send where

Part of [IPC](ipc.md).

## Who may send where

On top of the `IPC` capability every IPC call needs, a send passes three gates before it is queued.

- Capabilities. `caller_satisfies_endpoint` requires every bit the endpoint asks for, and refuses an unknown name or an endpoint that asks for nothing (`src/syscall/microkernel/ipc/send_caps.rs`, `fn caller_satisfies_endpoint`). Every service endpoint asks for `IPC`; the ones in `NETWORK_SERVICES` ask for `Network` as well, through `required_caps` (`src/services/registry/policy.rs`, `fn required_caps`). No endpoint asks for any other bit, so `keyring`, `entropy_pool` and `vfs_pool` take a send from any capsule that holds `IPC`; see [Capabilities](capabilities-checks-outside-the.md#checks-outside-the-syscall-table). A send by pid with `MkIpcSendToPid` must pass the gate of every endpoint the target serves, since all of them are read from one inbox (`src/syscall/microkernel/ipc/send_caps.rs`, `fn caller_satisfies_inbox`).
- [Held endpoints](../overview/glossary-hardware-broker.md#held-endpoint). Some drivers serve raw hardware, so `HELD` lets only the services that drive them send to them (`src/services/registry/held_table.rs`, `const HELD`). The kernel's own sends are not checked against this list.
- [Peer lists](../overview/glossary-peer-list.md#peer-list). A capsule on `PEERS` reaches only the endpoints named for it; in 0.9.2 that is `shield_prover`, which reaches `shield.core` alone (`src/services/registry/peers.rs`, `const PEERS`).

| Endpoint | Who may send |
|---|---|
| `driver.virtio_net0` | `net.core`, `net.l2` |
| `driver.e1000_0` | `net.core`, `net.l2` |
| `driver.rtl8169_0` | `net.core`, `net.l2` |
| `driver.rtl8139_0` | `net.core`, `net.l2` |
| `driver.iwlwifi0` | `net.core`, `app.settings`, `app.settings.1`, `app.settings.2`, `app.setup_wizard` |
| `driver.rtl8821ce0` | `net.core`, `app.settings`, `app.settings.1`, `app.settings.2`, `app.setup_wizard` |
| `driver.ps2_kbd0` | the kernel only |
| `driver.usb_hid0` | the kernel only |
| `driver.i2c_hid0` | the kernel only |
| `driver.usb_msc0` | the kernel only |
| `driver.virtio_rng` | the kernel only |
| `driver.xhci0` | `driver.usb_hid0`, `driver.usb_msc0` |
| `driver.i2c_pci0` | `driver.i2c_hid0` |
| `driver.virtio_gpu0` | `compositor` |
| `driver.hda0` | `audio.server` |

Three lists in the registry name services by role: the names reserved for core services, the names a capsule may claim at run time, and the services that need `Network`.

| List | Names |
|---|---|
| `RESERVED_NAMES` | `keyring`, `entropy_pool`, `crypto_pool`, `vfs_pool`, `market.index` |
| `RUNTIME_REGISTRABLE` | `net.tcp`, `net.udp`, `net.dhcp.client`, `net.dns`, `net.ip` |
| `NETWORK_SERVICES` | `net.core`, `net.l2`, `net.ip`, `net.udp`, `net.tcp`, `net.dns`, `net.dhcp.client`, `net.sockets`, `net.nym`, `net.anon`, `net.socks5` |

Registering a name at run time with `MkServiceRegister` is narrow on purpose. `allowed` refuses a name that starts with `proc.` or `endpoint.`, and `is_reserved_service` refuses the reserved names and the ports of the core services and their replies, 4098 to 4107 (`src/syscall/microkernel/ipc/register_allowed.rs`, `fn allowed`; `src/services/registry/reserved.rs`, `fn is_reserved_service`). A capsule may claim a name it does not already hold only when it has `RegisterService` or `Admin` and the name is in `RUNTIME_REGISTRABLE`.
