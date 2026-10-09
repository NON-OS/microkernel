# capsule_net_ntp

## Role

`capsule_net_ntp` is the SNTP time-sync client, the handle `net.ntp.client`.
when Direct is the network the person chose, it asks one fixed time server for
the time over `net.udp`, checks the reply, and hands the server's transmit time
to the kernel with `MkTimeAdjust`. it repeats every 15 minutes. it serves no
requests of its own.

```text
net.ntp.client  (no served endpoint)
    |
    | nonos_route_link::direct_refusal()  -- refused --> keep RTC, look again in 30 s
    |
    | net.udp over IPC: OP_BIND 50123, OP_SEND, OP_RECV (MkIpcCall)
    v
net.udp -> net.ip -> net.l2 -> NIC driver -> 162.159.200.1:123
    |
    | MkTimeAdjust(unix ms)
    v
kernel wall clock offset (src/syscall/microkernel/time.rs)
```

it is built into the `microkernel-net-ntp` profile and the `nonos-ntp-smoketest`
feature, and is not in the desktop image. the capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md).

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS := 0x40001C`: Network (`0x04`), IPC (`0x08`), Memory
  (`0x10`) and TimeSet (`0x400000`). `CAPSULE_OPTIONAL_CAPS := 0x100`, Debug,
  granted only by a `capsule-serial-debug` build; the kernel mirror
  (`src/userspace/capsule_net_ntp/spawn.rs`) requests `Capability::Network |
  Capability::IPC | Capability::Memory | serial_debug_cap() |
  Capability::TimeSet`.
- service `service:4482:net.ntp.client`, reply
  `reply:4483:endpoint.net.ntp.client.reply`, declared in `Capsule.mk`. the
  capsule never reads that inbox: a request sent there is not answered. the
  feature is `nonos-capsule-net-ntp`, namespace `systems.nonos.net.ntp.client`.
- it is a client only. it reaches `net.udp` with `MkIpcCall` (`mk_ipc_call`)
  after one `MkServiceLookup` (`mk_service_lookup`), reads the clock with
  `MkTimeMillis` (`mk_time_millis`), applies the result with `MkTimeAdjust`
  (`mk_time_adjust`), writes `[NTP]` markers with `MkDebug` (`mk_debug`) on a
  serial-debug build, waits with `MkYield` (`mk_yield`), and leaves with
  `MkExit` (`mk_exit`).

## Interface contract

the capsule exposes no served endpoint. it drives `net.udp` as a client
(`src/udp_client/`) over the shared 20 byte stack header, magic `0x4E554450`
("NUDP"), version 1:

| Op | Value | Use |
|---|---|---|
| `OP_BIND` | 2 | local port 50123; `E_PORT_IN_USE` is taken as success |
| `OP_SEND` | 4 | send the SNTP request |
| `OP_RECV` | 5 | poll for the reply |

## Authority

`CAPSULE_REQUIRED_CAPS := 0x40001C` and `CAPSULE_OPTIONAL_CAPS := 0x100`:

| Bit | Name | Why |
|---|---|---|
| 0x000004 | Network | `net.udp` requires it of callers |
| 0x000008 | IPC | `MkServiceLookup`, `MkIpcCall`, the policy store read |
| 0x000010 | Memory | the heap for request and reply buffers |
| 0x000100 | Debug (optional) | the `[NTP]` serial markers; a `capsule-serial-debug` build only |
| 0x400000 | TimeSet | `MkTimeAdjust` |

no CoreExec, Crypto, FileSystem or hardware bit. without Crypto the nonce is the
clock rather than a random value. TimeSet is the one authority here that changes
system state: whoever controls an accepted reply sets the machine's wall clock.
the kernel installs the mask from the verified manifest at spawn and the capsule
cannot widen it.

## Privacy and persistence

- under Direct the request goes out in the clear. the server and every observer
  on the path see the machine's address every 15 minutes, and the request's
  transmit timestamp carries the local clock.
- nothing is persisted. the serial line `[NTP] SYNC ok unix_ms=<ms>` prints the
  time applied; `[NTP] time sync skipped: <reason>` says why nothing was asked.

## Runtime lifecycle

spawned by the verified path. each turn of the loop asks
`nonos_route_link::direct_refusal()`, which reads the policy store's default
network. while the answer is a refusal, `step` (`src/decide.rs`) logs the reason
once, the RTC time is kept, and the capsule looks again every `ROUTE_CHECK_MS`,
30 s (`sleep_ms`, built on `MkYield`). an unknown or unreadable default counts as
the mixnet and refuses. only once Direct is the default does it look up
`net.udp`, bind its port and run the exchange, then sleep `RESYNC_INTERVAL_MS`,
15 minutes, before the next turn. on a machine set to an anonymous network
nothing of this capsule's touches the network. the `microkernel-net-ntp` profile
carries no policy store, so there the default cannot be read and the capsule
never asks a server.

## Failure model

- `sync_once` (`src/sync.rs`) sends the request, resends every `RESEND_MS`,
  400 ms, and polls `OP_RECV` until a datagram from 162.159.200.1 port 123 passes
  the checks or `EXCHANGE_DEADLINE_MS`, 3000 ms, pass. it then gives up for this
  turn and keeps the RTC time.
- a datagram from any other source, or one that fails the checks, is ignored, so
  one forged packet cannot cancel the exchange.
- a transmit time before 1970 is refused instead of wrapping the clock; the
  kernel then refuses a time outside 2025-01-01 to 2100-01-01.
- under Nym or Anyone the exchange never runs and the RTC time is kept.

## Current implemented surface

the route check, the single-server SNTP exchange, the reply validation and the
`MkTimeAdjust` apply. one compiled server, no fallback and no backoff beyond the
fixed 15 minute interval. no served IPC handler.

## Wire format

two layers. the IPC leg to `net.udp` is the shared 20 byte little-endian stack
header (`src/udp_client/header.rs`):

```text
offset  size  field
0       4     magic = 0x4E554450 ("NUDP")
4       2     version = 1
6       2     op
8       2     errno (reply only; zero in a request)
10      2     reserved
12      4     request_id
16      4     payload_len
20      ...   payload
```

the payload is the 48 byte SNTP packet (`src/sntp.rs`). `build_request` writes a
version 4 client request whose transmit time is the local clock in milliseconds;
that value is the 8 byte nonce the reply must echo. `parse_reply` takes a reply
only when it is 48 bytes, mode 4, version 3 or 4, stratum 1 to 15, leap indicator
not 3, and its originate timestamp equals the nonce. the transmit seconds are
read across the 2036 era rollover.

## State ownership

stateless across turns beyond the request buffer and the last nonce held for the
pending exchange. no table, no files, no sessions. the one piece of system state
it writes is the kernel wall clock offset, through `MkTimeAdjust`, which the
kernel owns.

## Operating rules

- ask `direct_refusal()` every turn; touch the network only once Direct is the
  default.
- bind local port 50123; treat `E_PORT_IN_USE` as success.
- accept a reply only from 162.159.200.1 port 123 that passes every check and
  echoes the nonce.
- refuse a transmit time before 1970; leave the out-of-range guard to the kernel.
- apply the accepted time with `MkTimeAdjust` and sleep 15 minutes.

## Release target

0.9.2.

## Release evidence

`userland/net_proofs` includes `src/sntp.rs` and checks the request and the
reply rules. `userland/route_link_proofs` includes `src/decide.rs` and holds it,
with `direct_refused`, for every default. the `nonos-ntp-smoketest` feature adds
`capsule-serial-debug` so the `[NTP]` markers can be read.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x40001C`.
- [ ] the capsule stays silent on an anonymous-network default (`route_link_proofs`).
- [ ] the reply rules in `src/sntp.rs` hold (`net_proofs`).
- [ ] the kernel mirror `src/userspace/capsule_net_ntp` requests the same mask
      plus optional Debug.

## Explicit non-goals today

- no round-trip delay compensation, slewing, leap second handling, NTS or
  authenticated NTP, IPv6 or server mode.
- no fallback server and no backoff beyond the fixed interval.
- no time over an anonymity network: under Nym or Anyone it keeps the RTC time.

## Verification

`Capsule.mk` carries the mask and endpoints. the IPC header is fixed by
`src/udp_client/header.rs`, the SNTP packet and its checks by `src/sntp.rs`, the
timing by `src/main.rs` (`ROUTE_CHECK_MS`, `RESYNC_INTERVAL_MS`) and
`src/sync.rs` (`RESEND_MS`, `EXCHANGE_DEADLINE_MS`), the server and local port by
`src/sync.rs` and `src/state.rs`. `net_proofs` and `route_link_proofs` check the
behavior end to end.
