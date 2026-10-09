# capsule_net_dhcp

## Role

`capsule_net_dhcp` is the DHCPv4 client of the split network stack. it runs the
DISCOVER, OFFER, REQUEST and ACK ladder over raw ethernet frames on `net.l2`,
installs the accepted lease into `net.ip`, tells `net.l2` the new address, and
answers lease status to anyone who asks.

```text
net.dhcp.client -- raw DHCP frames over MkIpcCall --> net.l2 --> NIC driver
    |
    `-- accepted lease (address, prefix, gateway) over MkIpcCall --> net.ip
```

it reads the link directly because there is no address yet for `net.ip` and
`net.udp` to receive on. the desktop image does not carry it: there `net.core`
runs DHCP inside smoltcp and registers the `net.dhcp.client` name itself. it is
built into the `microkernel-net-dhcp` and `microkernel-net-ntp` profiles and the
`microkernel-input-e2e-ps2` test image.

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x0003c`: Network (`0x04`), IPC (`0x08`), Memory
  (`0x10`) and Crypto (`0x20`). the kernel mirror
  (`src/userspace/capsule_net_dhcp/spawn.rs`) requests exactly
  `Capability::IPC | Capability::Memory | Capability::Crypto |
  Capability::Network`. no optional caps.
- service `service:4440:net.dhcp.client`, reply
  `reply:4441:endpoint.net.dhcp.client.reply`, declared for the manifest; the
  kernel registers the `net.dhcp.client` name at spawn.
- the feature is `nonos-capsule-net-dhcp`, namespace
  `systems.nonos.net.dhcp.client`.
- it reaches the kernel only over the `MkIpc*` syscall surface. it serves its
  own endpoint with `MkIpcRecvFrom` (`mk_ipc_recv_from`) and `MkIpcReply`
  (`mk_ipc_reply`); it resolves `net.l2` and `net.ip` with `MkServiceLookup`
  (`mk_service_lookup`) and drives both with `MkIpcCall` (`mk_ipc_call`),
  sending and polling bootp frames on `net.l2` and installing the lease and
  address on `net.ip` and `net.l2`. it paces its ladder with `MkYield`
  (`mk_yield`) and calls `MkExit` (`mk_exit`) only when the heap fails to
  initialise. Crypto draws each transaction id (`nonos_libc::crypto_random`,
  `src/state/global.rs`).

## Interface contract

requests carry the 20-byte header shared by the stack capsules, magic
`0x4E444843` ("NDHC"), version 1. the op set (`src/protocol/ops.rs`):

| Op | Value | Who may call | Meaning |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | anyone | liveness |
| `OP_LEASE_REQUEST` | 2 | `app.settings` | acquire a lease again |
| `OP_LEASE_STATUS` | 3 | anyone | the current lease |
| `OP_LEASE_RELEASE` | 4 | `app.settings` | give the lease back |
| `OP_LEASE_RENEW` | 5 | `app.settings` | REQUEST the bound address again |

request, renew and release change the machine's address, so only the Settings
app may send them; any other sender gets `E_PERM` (8). `LEASE_ADMINS` and
`may_change_lease` in `src/server/lease_admin.rs` hold the rule.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x0003c` is the whole authority the capsule asks for:
Network, IPC, Memory and Crypto. Network because the kernel registers a network
service only for a holder of it; Crypto to draw each bootp transaction id. no
CoreExec, FileSystem, Debug or hardware bits: the card belongs to the NIC driver
capsule, reached through `net.l2`. `net.l2` lets only this capsule and `net.ip`
move frames, and only this capsule set the address. the kernel installs the mask
from the verified manifest at spawn and the capsule cannot widen it.

## Privacy and persistence

the lease (address, prefix, gateway, DNS server, lease time, server identifier)
lives in memory (`src/state/lease.rs`). nothing is written to storage and no
secret is held. a DISCOVER broadcast carries the card's MAC on the wire, as the
protocol requires.

## Runtime lifecycle

`_start` (`src/main.rs`) initialises the heap, loops `setup::run` behind
`MkYield` until `net.l2` and `net.ip` are both registered, then
`acquire_initial_lease` tries the DORA ladder up to sixteen times with a longer
`MkYield` backoff between tries. once a lease is taken or the tries are spent it
serves inbox 0 forever (`src/server/runner.rs`). it is not restarted.

## Failure model

a frame that fails header parsing is still answered, under the op and request id
it names, or under zeros when it is too short. a lease-changing op from any
sender but Settings gets `E_PERM`. during acquisition `wait_for` reads at most
`MAX_POLL_ITERATIONS` frames for the matching transaction id and gives up
otherwise, so the ladder retries rather than hanging. `dhcp_payload`
(`src/frame/extract.rs`) drops a reply unless its framing passes the checks
`net.ip` would make: the IPv4 header checksum, a total length that covers the
header and fits the frame, no fragment, and the UDP checksum when the sender
computed one. if the heap cannot initialise at start, `_start` calls
`mk_exit(1)`.

## Current implemented surface

the five ops above, each with a handler in `src/server/handlers`, and the DORA
ladder in `src/dora` (`discover`, `request`, `acquire`, `install`, `release`).
`install` writes the lease into `net.ip` with `OP_SET_CONFIG` and the address
into `net.l2` with `OP_SET_IP`, both over `MkIpcCall`. the capsule frames and
unframes ethernet, IPv4 and UDP itself (`src/frame`) because it runs before
`net.ip` has an address.

## Wire format

on its own endpoint, the 20-byte v1 envelope: `u32` magic (`0x4E444843`), `u16`
version (1), `u16` op, `u16` flags (errno on a response), `u16` reserved, `u32`
request_id, `u32` payload_len, then the lease payload. on the link it builds and
reads standard DHCPv4 over UDP over IPv4 over ethernet (`src/dhcp`, `src/frame`)
and moves those frames through `net.l2`'s "NL2\0" envelope with `MkIpcCall`.

## State ownership

the capsule owns the resolved `net.l2` and `net.ip` ports, the current lease and
the active transaction id (`src/state`). all of it lives in memory for the life
of the process. there is no on-disk or shared state.

## Operating rules

- one interface, the one `net.l2` bound.
- the transaction id comes from Crypto, never a counter, so replies are matched
  to the request that drew it.
- only Settings may force a re-acquire, renew or release; everyone else may read
  status.
- it frames the whole bootp exchange itself and does not depend on `net.ip`
  being configured.

## Release target

0.9.2.

## Release evidence

`userland/dhcp_proofs` runs the request header decode and the reply a refused
frame gets. `userland/net_proofs` runs the DHCP parser, the frame checks and
`may_change_lease`. the `microkernel-net-dhcp` and `microkernel-net-ntp`
profiles and the `microkernel-input-e2e-ps2` image boot the capsule against a
real `net.l2` and `net.ip`.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x0003c`, kernel mirror requests the
      same four caps.
- [ ] the five ops answer, and authz refuses request, renew and release to any
      sender but Settings.
- [ ] the transaction id is drawn from Crypto and replies are matched to it.
- [ ] frame intake applies the IPv4 and UDP checks before accepting a reply.
- [ ] `dhcp_proofs` and `net_proofs` pass.

## Explicit non-goals today

- no expiry or renewal timer. the lease runs out unless Settings sends
  `OP_LEASE_RENEW`, and no profile that carries this capsule carries Settings.
- no DHCPv6 and no DHCP server.
- one interface, the one `net.l2` bound.

## Verification

the magic, version and header layout are fixed in `src/protocol`; the op set in
`src/protocol/ops.rs`; the cap mask in `Capsule.mk` and
`src/userspace/capsule_net_dhcp/spawn.rs`. the `MkIpc*` calls are the only
kernel IPC surface the capsule touches (`mk_ipc_recv_from`, `mk_ipc_reply`,
`mk_ipc_call`, `mk_service_lookup`, plus `mk_yield` and `mk_exit`), and the
transaction id comes from `nonos_libc::crypto_random`. `dhcp_proofs` and
`net_proofs` check the behavior end to end. see
[the network stack](../../docs/handbook/network/stack.md).
