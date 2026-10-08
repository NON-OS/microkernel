# capsule_net_dns

## Role

`capsule_net_dns` is the DNS resolver of the split network stack. it builds a
query, sends it over `net.udp` to one upstream resolver, checks that the answer
is a reply to that query, and keeps a small cache of A records.

```text
net.sockets / client capsule
    |
    | net.dns requests over MkIpc (20-byte "NDNS" envelope)
    v
net.dns -- query, answer check, cache --> net.udp --> upstream resolver
            (MkIpcCall to net.udp; MkServiceLookup finds it)
```

the desktop image does not carry it: there `net.core` registers the `net.dns`
name and answers A lookups through smoltcp. it is built into the
`microkernel-input-e2e-ps2` test image.

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x0003c`: Network (`0x04`), IPC (`0x08`), Memory
  (`0x10`) and Crypto (`0x20`). the kernel mirror
  (`src/userspace/capsule_net_dns/spawn.rs`) requests exactly
  `Capability::IPC | Capability::Memory | Capability::Crypto |
  Capability::Network`. no optional caps.
- service `service:4450:net.dns`, reply `reply:4451:endpoint.net.dns.reply`,
  declared for the manifest; the kernel registers the `net.dns` name at spawn.
- the feature is `nonos-capsule-net-dns`, namespace `systems.nonos.net.dns`.
- it reaches the kernel only over the `MkIpc*` syscall surface. it serves its
  endpoint with `MkIpcRecvFrom` (`mk_ipc_recv_from`) and `MkIpcReply`
  (`mk_ipc_reply`); it resolves `net.udp` and `net.dhcp.client` with
  `MkServiceLookup` (`mk_service_lookup`) and drives the UDP socket (bind, send,
  recv) and the lease lookup with `MkIpcCall` (`mk_ipc_call`). it reads the
  clock for resend and deadline timing with `MkTimeMillis` (`mk_time_millis`),
  paces its waits with `MkYield` (`mk_yield`), and calls `MkExit` (`mk_exit`)
  only when the heap fails to initialise. Crypto draws each query id and source
  port.

## Interface contract

requests carry the 20-byte header shared by the stack capsules, magic
`0x4E444E53` ("NDNS"), version 1. the op set (`src/protocol/ops.rs`):

| Op | Value | Who may call | Meaning |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | anyone | liveness |
| `OP_RESOLVE_A` | 2 | anyone | IPv4 address for a name |
| `OP_RESOLVE_AAAA` | 3 | anyone | IPv6 address for a name |
| `OP_FLUSH_CACHE` | 4 | `net.admin` | empty the cache |
| `OP_SET_UPSTREAM` | 5 | `net.admin` | change the upstream resolver |

no capsule registers `net.admin`, so the last two answer `E_PERM` to everyone
(`src/server/authz.rs`). errnos: `E_TIMEOUT`, `E_NXDOMAIN`, `E_SERVFAIL`,
`E_NAME_INVALID`, `E_PERM` and the header errors.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x0003c` is the whole authority the capsule asks for:
Network, IPC, Memory and Crypto. Network because the kernel registers a network
service only for a holder of it; Crypto to draw each query id and source port.
no CoreExec, FileSystem, Debug or hardware bits. the kernel installs the mask
from the verified manifest at spawn and the capsule cannot widen it.

## Privacy and persistence

a lookup names the host to the upstream resolver and to everyone on the path:
queries go in the clear. the cache lives in memory and nothing is logged or
written to storage. this capsule reads no network route; callers that must not
look a name up in the clear ask `nonos_route_link::direct_refusal` first.

## Runtime lifecycle

`_start` (`src/main.rs`) initialises the heap, loops `setup::run` behind
`MkYield` until `net.udp` is resolved (`src/setup.rs`), and asks
`net.dhcp.client` for the lease; the leased DNS server replaces
`DEFAULT_UPSTREAM`, 1.1.1.1, when there is one
(`src/dhcp_upstream`). it then serves inbox 0 forever
(`src/server/runner.rs`). it is not restarted.

## Failure model

a frame that fails header parsing is still answered, under the op and request id
it names, or under zeros when it is too short. the two `net.admin` ops answer
`E_PERM` to everyone. `exchange` sends from a random high port, resends every
`RESEND_MS` (400 ms) and gives up with `E_TIMEOUT` after `DEADLINE_MS` (3 s),
timed with `MkTimeMillis`. it reads a packet further only when it comes from the
upstream address and port 53 and is a response with this query's id and
question; anything else is ignored and the wait goes on, so a stray or forged
packet cannot end the lookup. a server failure maps to `E_SERVFAIL`, a missing
name to `E_NXDOMAIN`, a malformed name to `E_NAME_INVALID`. if the heap cannot
initialise at start, `_start` calls `mk_exit(1)`.

## Current implemented surface

the five ops above, each with a handler in `src/server/handlers`. the resolver
(`src/dns`): `first_address` takes a record of the asked type owned by the asked
name, or by the name a chain of up to `MAX_CHAIN`, eight, CNAMEs leads to; the
TTL is the least along the chain and a loop ends with no answer. names are read
with compression pointers that must point strictly backwards, labels of at most
63 bytes and names of at most 255. A records are cached for their TTL, never
longer than one day, in `ENTRY_CAP`, 128 slots; AAAA answers are not cached.

## Wire format

on its own endpoint, the 20-byte v1 envelope: `u32` magic (`0x4E444E53`), `u16`
version (1), `u16` op, `u16` flags (errno on a response), `u16` reserved, `u32`
request_id, `u32` payload_len, then the name in the request and the address in
the reply. on the link it builds and parses standard DNS messages (`src/dns`)
and carries them over `net.udp`'s socket envelope with `MkIpcCall`.

## State ownership

the capsule owns the resolved `net.udp` port, the current upstream resolver
address and port, and the A-record cache (`src/state.rs`, `src/dns/cache`). all
of it lives in memory for the life of the process. there is no on-disk or shared
state.

## Operating rules

- one upstream at a time, and it cannot be changed while nothing holds
  `net.admin`.
- the query id and source port come from Crypto, so an off-path packet cannot
  be mistaken for the answer.
- only A records are cached; AAAA is resolved each time.
- the resolver does its own timing with `MkTimeMillis`; it never blocks the
  server loop past its deadline.

## Release target

0.9.2.

## Release evidence

`userland/dns_proofs` drives the real resolve handlers against an upstream
played on the host. `userland/net_proofs` runs the response parser, the name
reader and the answer selection. the `microkernel-input-e2e-ps2` image boots the
capsule against a real `net.udp`.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x0003c`, kernel mirror requests the
      same four caps.
- [ ] the five ops answer, and authz refuses flush and set-upstream to everyone.
- [ ] the query id and source port are drawn from Crypto and the answer is
      matched on source address, port, id and question.
- [ ] CNAME chains cap at `MAX_CHAIN` and the cache TTL never exceeds one day.
- [ ] `dns_proofs` and `net_proofs` pass.

## Explicit non-goals today

- no DNS over TLS or HTTPS, no DNSSEC.
- one upstream at a time, not changeable while nothing holds `net.admin`.
- no reverse lookups and no record types beyond A and AAAA.

## Verification

the magic, version and header layout are fixed in `src/protocol`; the op set in
`src/protocol/ops.rs`; the cap mask in `Capsule.mk` and
`src/userspace/capsule_net_dns/spawn.rs`. the `MkIpc*` calls are the only kernel
IPC surface the capsule touches (`mk_ipc_recv_from`, `mk_ipc_reply`,
`mk_ipc_call`, `mk_service_lookup`), alongside `mk_time_millis`, `mk_yield` and
`mk_exit`, and the query id comes from Crypto. `dns_proofs` and `net_proofs`
check the behavior end to end. see
[the network stack](../../docs/handbook/network/stack.md).
