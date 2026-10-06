# capsule_net_ntp

## Role

`capsule_net_ntp` is the SNTP client, `net.ntp.client`. When Direct is the
network the person chose, it asks one fixed time server for the time over
`net.udp`, checks the reply, and hands the server's transmit time to the kernel
with `MkTimeAdjust`. It repeats every 15 minutes. It serves no requests of its
own.

```text
net.ntp.client  (no served operations)
    |
    | nonos_route_link::direct_refusal()  -- refused --> keep the RTC time, look again in 30 s
    |
    | MkIpcCall: OP_BIND 50123, OP_SEND, OP_RECV   (net.udp)
    v
net.udp -> net.ip -> net.l2 -> NIC driver -> 162.159.200.1:123
    |
    | MkTimeAdjust(unix ms)
    v
kernel wall clock offset (src/syscall/microkernel/time.rs)
```

It is built into the `microkernel-net-ntp` profile and the
`nonos-ntp-smoketest` feature, and is not in the desktop image.

## Why it waits for Direct

A time server sees the machine that asks it, and SNTP over UDP cannot cross the
Nym mixnet or the Anyone network. So each turn of the loop asks
`nonos_route_link::direct_refusal()`, which reads the policy store's default
network. While the answer is a refusal, `step` (`src/decide.rs`) logs the
reason once, the RTC time is kept, and the capsule looks again every
`ROUTE_CHECK_MS`, 30 s. An unknown or unreadable default counts as the mixnet
and refuses. Only once Direct is the default does it look up `net.udp` and bind
its port, so on a machine set to an anonymous network nothing of this capsule's
touches the network.

The `microkernel-net-ntp` profile carries no policy store, so there the default
cannot be read and the capsule never asks a server.

## Service and endpoints

- Handle `net.ntp.client`. The manifest names `service:4482:net.ntp.client`
  and `reply:4483:endpoint.net.ntp.client.reply`, but the capsule never reads
  that inbox: a request sent there is not answered.
- Kernel mirror `src/userspace/capsule_net_ntp`, whose `spawn.rs` requests
  Network, IPC, Memory and TimeSet, plus Debug from a serial-debug build.

It calls `net.udp` (`src/udp_client/`): `OP_BIND` 2 for local port 50123
(`E_PORT_IN_USE` is taken as success), `OP_SEND` 4 and `OP_RECV` 5.

## The exchange

`sync_once` (`src/sync.rs`):

- builds a version 4 client request whose transmit time is the local clock in
  milliseconds; that value is the nonce the reply must echo;
- sends it, resends every 400 ms, and polls `OP_RECV` until a datagram from
  162.159.200.1 port 123 passes the checks or 3000 ms pass;
- ignores a datagram from any other source, and one that fails the checks, so
  one forged packet cannot cancel the exchange.

`parse_reply` (`src/sntp.rs`) takes a reply only when it is 48 bytes, mode 4,
version 3 or 4, stratum 1 to 15, leap indicator not 3, and its originate
timestamp equals the nonce. The transmit seconds are read across the 2036 era
rollover; a time before 1970 is refused instead of wrapping the clock. The
kernel then refuses a time outside 2025-01-01 to 2100-01-01.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x40001C` and `CAPSULE_OPTIONAL_CAPS := 0x100`:

| Bit | Name | Why |
|---|---|---|
| 0x000004 | Network | `net.udp` requires it of callers |
| 0x000008 | IPC | `MkServiceLookup`, `MkIpcCall`, the policy store read |
| 0x000010 | Memory | the heap for request and reply buffers |
| 0x000100 | Debug (optional) | the `[NTP]` serial markers; only a `capsule-serial-debug` build grants it |
| 0x400000 | TimeSet | `MkTimeAdjust` |

No CoreExec, Crypto, FileSystem or hardware bit. Without Crypto the nonce is
the clock rather than a random value. TimeSet is the one authority here that
changes system state: whoever controls an accepted reply sets the machine's
wall clock.

## Privacy and persistence

- Under Direct the request goes out in the clear. The server and every
  observer on the path see the machine's address every 15 minutes, and the
  request's transmit timestamp carries the local clock.
- Nothing is persisted. The serial line `[NTP] SYNC ok unix_ms=<ms>` prints the
  time applied; `[NTP] time sync skipped: <reason>` says why nothing was asked.

## What it does not do

- One compiled server, no fallback and no backoff beyond the fixed interval.
- No round-trip delay compensation, slewing, leap second handling, NTS or
  authenticated NTP, IPv6 or server mode.
- No time over an anonymity network: under Nym or Anyone it keeps the RTC
  time.

## Build and verify

- `make nonos-mk-net-ntp`, then `nonos-mk-net-ntp-sign` and
  `nonos-mk-net-ntp-verify`.
- Kernel profile: `microkernel-net-ntp` in the root `Cargo.toml`; the smoke run
  `nonos-ntp-smoketest` adds `capsule-serial-debug` so the `[NTP]` markers can
  be read.
- `userland/net_proofs` includes `src/sntp.rs` and checks the request and the
  reply rules. `userland/route_link_proofs` includes `src/decide.rs` and holds it, with `direct_refused`, for every default.

See [the network stack](../../docs/handbook/network/stack.md) and
[network routes](../../docs/handbook/network/socks5-and-routes.md).
