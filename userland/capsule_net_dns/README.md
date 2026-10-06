# capsule_net_dns

## Role

`capsule_net_dns` is the DNS resolver of the split network stack. It builds a
query, sends it over `net.udp` to one upstream resolver, checks that the answer
is a reply to that query, and keeps a small cache of A records.

```text
net.sockets / client capsule
    |
    | NDNS requests over IPC
    v
net.dns -- query, answer check, cache --> net.udp --> upstream resolver
```

The desktop image does not carry it: there `net.core` registers the `net.dns`
name and answers A lookups through smoltcp. It is built into the
`microkernel-input-e2e-ps2` test image.

## Service and endpoints

- Handle `net.dns`, service endpoint `service:4450:net.dns`, reply endpoint
  `reply:4451:endpoint.net.dns.reply`.
- Kernel mirror `src/userspace/capsule_net_dns`.
- At setup it finds `net.udp` and asks `net.dhcp.client` for the lease; the
  leased DNS server replaces `DEFAULT_UPSTREAM`, 1.1.1.1, when there is one.

## Interface

Requests carry the 20 byte header shared by the stack capsules, with magic
`0x4E444E53` ("NDNS") and version 1.

| Op | Value | Who may call | Meaning |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | anyone | liveness |
| `OP_RESOLVE_A` | 2 | anyone | IPv4 address for a name |
| `OP_RESOLVE_AAAA` | 3 | anyone | IPv6 address for a name |
| `OP_FLUSH_CACHE` | 4 | `net.admin` | empty the cache |
| `OP_SET_UPSTREAM` | 5 | `net.admin` | change the upstream resolver |

No capsule registers `net.admin`, so the last two answer `E_PERM` to everyone.
A frame that fails header parsing is still answered, under the op and request
id it names, or under zeros when it is too short. Errnos: `E_TIMEOUT`,
`E_NXDOMAIN`, `E_SERVFAIL`, `E_NAME_INVALID`, `E_PERM` and the header errors.

## Lookups

- `exchange` sends from a random high port, resends every `RESEND_MS`
  (400 ms) and gives up after `DEADLINE_MS` (3 s). It reads a packet further
  only when it comes from the upstream address and port 53 and is a response
  with this query's id and question. Anything else is ignored and the wait goes
  on, so a stray or forged packet cannot end the lookup.
- `first_address` takes a record of the asked type owned by the asked name, or
  by the name a chain of up to `MAX_CHAIN`, eight, CNAMEs leads to. The TTL is
  the least along the chain, and a loop ends with no answer.
- Names are read with compression pointers that must point strictly backwards,
  labels of at most 63 bytes and names of at most 255.
- A records are cached for their TTL, never longer than one day, in
  `ENTRY_CAP`, 128, slots. AAAA answers are not cached.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0003c`: Network (`0x04`), IPC (`0x08`), Memory
(`0x10`) and Crypto (`0x20`). Network because the kernel registers a network
service only for a holder of it; Crypto draws each query id and source port.
No CoreExec, FileSystem, Debug or hardware bits.

## Privacy and persistence

A lookup names the host to the upstream resolver and to everyone on the path:
queries go in the clear. The cache lives in memory and nothing is logged or
written to storage. This capsule reads no network route; callers that must not
look a name up in the clear ask `nonos_route_link::direct_refusal` first.

## What it does not do

- No DNS over TLS or HTTPS, no DNSSEC.
- One upstream at a time, and it cannot be changed while nothing holds
  `net.admin`.
- No reverse lookups and no record types beyond A and AAAA.

## Build and verify

- `make nonos-mk-net-dns`, then `nonos-mk-net-dns-sign` and
  `nonos-mk-net-dns-verify`.
- `userland/dns_proofs` drives the real resolve handlers against an upstream
  played on the host. `userland/net_proofs` runs the response parser, the
  name reader and the answer selection.

See [the network stack](../../docs/handbook/network/stack.md).
