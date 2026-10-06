# capsule_net_dhcp

## Role

`capsule_net_dhcp` is the DHCPv4 client of the split network stack. It runs
DISCOVER, OFFER, REQUEST and ACK over raw Ethernet frames on `net.l2`, installs
the lease into `net.ip`, tells `net.l2` the new address, and answers lease
status to anyone who asks.

```text
net.dhcp.client -- raw DHCP frames --> net.l2 --> NIC driver
    |
    `-- accepted lease (address, prefix, gateway) --> net.ip
```

It reads the link directly because there is no address yet for `net.ip` and
`net.udp` to receive on. The desktop image does not carry it: there `net.core`
runs DHCP inside smoltcp and registers the `net.dhcp.client` name itself. It
is built into the `microkernel-net-dhcp` and `microkernel-net-ntp` profiles and
the `microkernel-input-e2e-ps2` test image.

## Service and endpoints

- Handle `net.dhcp.client`, service endpoint `service:4440:net.dhcp.client`,
  reply endpoint `reply:4441:endpoint.net.dhcp.client.reply`.
- Kernel mirror `src/userspace/capsule_net_dhcp`.
- At start it waits until `net.l2` and `net.ip` are registered, tries to take
  a lease up to sixteen times, then serves inbox 0.

## Interface

Requests carry the 20 byte header shared by the stack capsules, with magic
`0x4E444843` ("NDHC") and version 1.

| Op | Value | Who may call | Meaning |
|---|---|---|---|
| `OP_HEALTHCHECK` | 1 | anyone | liveness |
| `OP_LEASE_REQUEST` | 2 | `app.settings` | acquire a lease again |
| `OP_LEASE_STATUS` | 3 | anyone | the current lease |
| `OP_LEASE_RELEASE` | 4 | `app.settings` | give the lease back |
| `OP_LEASE_RENEW` | 5 | `app.settings` | REQUEST the bound address again |

Request, renew and release change the machine's address, so only the Settings
app may send them; any other sender gets `E_PERM` (8). `LEASE_ADMINS` and
`may_change_lease` in `src/server/lease_admin.rs` hold the rule. A frame that
fails header parsing is still answered, under the op and request id it names,
or under zeros when it is too short.

## Frames

`dhcp_payload` (`src/frame/extract.rs`) takes a reply only when its framing
passes the checks `net.ip` would make: the IPv4 header checksum, a total length
that covers the header and fits the frame, no fragment, and the UDP checksum
when the sender computed one. `wait_for` reads at most `MAX_POLL_ITERATIONS`
frames for the matching transaction id, which is drawn from `crypto_random`.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0003c`: Network (`0x04`), IPC (`0x08`), Memory
(`0x10`) and Crypto (`0x20`). Network because the kernel registers a network
service only for a holder of it; Crypto draws each transaction id. No CoreExec,
FileSystem, Debug or hardware bits. `net.l2` lets only this capsule and
`net.ip` move frames, and only this capsule set the address.

## Privacy and persistence

The lease (address, prefix, gateway, DNS server, lease time, server
identifier) lives in memory. Nothing is written to storage.

## What it does not do

- No expiry or renewal timer. The lease runs out unless Settings sends
  `OP_LEASE_RENEW`, and no profile that carries this capsule carries Settings.
- No DHCPv6 and no DHCP server.
- One interface, the one `net.l2` bound.

## Build and verify

- `make nonos-mk-net-dhcp`, then `nonos-mk-net-dhcp-sign` and
  `nonos-mk-net-dhcp-verify`.
- `userland/dhcp_proofs` runs the request header decode and the reply a
  refused frame gets. `userland/net_proofs` runs the DHCP parser, the frame
  checks and `may_change_lease`.

See [the network stack](../../docs/handbook/network/stack.md).
