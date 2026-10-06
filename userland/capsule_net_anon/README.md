# capsule_net_anon

## Role

`capsule_net_anon` is the Anyone Protocol onion routing client (`net.anon`). Anyone is a fork
of Tor 0.4.8.11; what was checked identical and what differs is recorded in `PROTOCOL.md`. The
capsule fetches and verifies the directory from the seven Anyone authorities, keeps one TLS link
to one guard, builds three-hop circuits with ntor over CREATE2 and EXTEND2, and carries
RELAY_BEGIN streams to an exit. It sits beside `net.nym` as the second privacy transport and
reaches the network only through `net.tcp`.

```text
SOCKS5 caller (frames 0, 1, 2) API caller (ANO1 header)
 \ /
 +------ service:4484 --------+
 |
 v
+--------------------------- net.anon ----------------------------+
| directory: fetch, inflate, RSA quorum, microdesc match |
| path: guard / middle / exit draw by consensus weights |
| link: one TLS session to the guard, CERTS + Ed25519 chain |
| circuits: CREATE2 / EXTEND2 with ntor, AES-128-CTR, SHA-1 digest |
| streams: RELAY_BEGIN, DATA, END, authenticated v1 SENDME |
+-----------------------------------------------------------------+
 | MkIpcCall | MkIpcCall
 v v
 net.tcp crypto_pool (RSA verify)
 |
 +--> authority DirPort 9230 (HTTP) +--> guard ORPort (TLS)
```

## Microkernel contract

The capsule is a userland IPC service:

- `MkIpcRecvFrom` receives requests on `service:4484:net.anon` with a 200 ms timeout
 (`src/server/runner.rs:43`). The same call answers early callers while `net.tcp` is still
 missing (`src/early.rs:44`).
- `MkIpcReply` answers the kernel-attested sender pid (`src/server/respond.rs:32`,
 `src/server/socks/answer.rs:34`). The manifest reply endpoint is
 `reply:4485:endpoint.net.anon.reply`.
- `MkIpcCall` reaches `net.tcp` (`src/tcp_client/envelope.rs:35`) and `crypto_pool` for RSA
 signature checks (`src/directory/verify/frame.rs:47`).
- `MkServiceLookup` resolves `net.tcp` (`src/setup.rs`) and `crypto_pool`
 (`src/directory/verify/frame.rs:32`).
- The libc crypto calls `crypto_random`, `crypto_hash`, `crypto_hmac_sha256`,
 `crypto_hkdf_sha256`, `crypto_x25519_public` and `crypto_x25519_shared` supply entropy,
 SHA-256, HMAC, HKDF and X25519 (`src/crypto/`).
- `MkDebug` writes the `[ANON]` serial markers (`src/trace/`). `MkTimeMillis`, `MkUptimeMs`
 and `MkYield` pace the loops.
- Its wire magic is `0x414E4F31`. Its kernel mirror target is `src/userspace/capsule_net_anon`.

The kernel does not parse cells, hold circuit keys, choose relays, or verify the consensus.

## Interface contract

API operations (`src/protocol/ops.rs`, routed in `src/server/dispatch.rs`):

| Operation | Value | Meaning |
|---|---|---|
| `OP_HEALTHCHECK` | 1 | liveness, empty body |
| `OP_STATUS` | 2 | bootstrap stage, relay count, circuit count, open circuit count |
| `OP_SYNC_DIRECTORY` | 3 | answered `E_OK`; the idle path already fetches |
| `OP_BUILD_CIRCUIT` | 4 | answered `E_OK`; the idle path already builds |
| `OP_OPEN_STREAM` | 5 | port and host; returns a stream id |
| `OP_SEND` | 6 | stream id and bytes; returns bytes accepted |
| `OP_RECV` | 7 | stream id; returns bytes that have arrived, or the close reason |
| `OP_CLOSE_STREAM` | 8 | send RELAY_END and drop one stream |
| `OP_CLOSE_CIRCUIT` | 9 | send DESTROY and drop a circuit with all its streams |
| `OP_CIRCUIT_PATH` | 10 | IPv4 address and ORPort of each hop of an open circuit |

The same service port also accepts SOCKS5 (RFC 1928) carried as IPC frames
(`src/server/socks/`). A frame whose first byte is 0 (stream bytes), 1 (reset), 2 (numbered
stream bytes), 3 (reset of a named stream: u32 stream) or 4 (numbered bytes on a named
stream: u32 stream, u32 sequence, bytes) goes to the SOCKS front; an API request starts with the magic, whose first byte
is `0x31`, so the two never collide (`src/server/socks/frame.rs`). Host names are sent to the
exit to resolve, so no DNS lookup leaves the machine (`src/server/socks/mod.rs`). Up to 32
SOCKS conversations are held at once (`src/server/socks/front.rs`), each keyed on the
kernel-attested pid and the stream it named, frames 0 to 2 being stream 0
(`src/server/socks/who.rs`); one caller holds at most 8, and the conversations of a caller
that ended are ended with the stream reaper. A numbered frame repeated with no
bytes, or with the same bytes, gets the kept answer; one repeated with new bytes has them
carried, with the answer the caller missed in front (`src/server/socks/kept.rs`).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x0003c` and `CAPSULE_OPTIONAL_CAPS = 0x100` (`Capsule.mk`):

| Bit | Name | Why |
|---|---|---|
| 0x004 | Network | open sockets through `net.tcp` to relays and authorities |
| 0x008 | IPC | serve `net.anon`, call `net.tcp` and `crypto_pool` |
| 0x010 | Memory | the heap a 1.8 MB consensus is parsed in |
| 0x020 | Crypto | entropy, SHA-256, HMAC, HKDF and X25519 through the crypto syscalls |
| 0x100 | Debug (optional) | the `[ANON]` serial markers; only a `capsule-serial-debug` build grants it |

It is the same mask as `net.nym`. No FileSystem, Hardware, driver, MMIO, IRQ, DMA, PIO, Admin
or graphics bit is asked for: the capsule never touches a device, a file or a surface.

## Privacy and persistence

- Nothing is written to storage. The consensus, microdescriptors, authority certificates, guard
 choice, link, circuit keys and stream buffers live in the `Manager` heap
 (`src/manager/state/manager.rs`) and end with the process.
- The guard is kept across link losses within one boot, so a dropped connection cannot walk the
 client onto a different relay. It is replaced after three failures, where a link that breaks
 within 30 s of opening counts as one (`src/manager/guard.rs`, `src/path/guard_pick.rs`). Up to
 eight guards this boot gave up on are remembered, and a redraw avoids each of them and its
 /16. It is not kept across reboots.
- Host names travel inside RELAY_BEGIN to the exit; the capsule does no local DNS.
- `OP_CIRCUIT_PATH` returns relay addresses, never identity keys
 (`src/server/handlers/path.rs`).
- The serial log names authority addresses and the guard address
 (`link open to guard <addr>`, `src/manager/link_tick.rs:47`) and stream and circuit ids. It
 does not log destination hosts or stream payloads.
- A one-hop or two-hop circuit is not offered: `HOPS` is fixed at 3
 (`src/protocol/limits.rs`).

## Runtime lifecycle

1. `heap_init`, or exit with status 1 (`src/main.rs`).
2. `early::wait_for_setup` looks up `net.tcp` every 250 ms with no bound, answering any request
 that arrives meanwhile with `E_NO_TCP` rather than dropping it (`src/early.rs`).
3. `server::run` serves requests and runs the idle work at least every 200 ms
 (`src/server/idle.rs`), in this order: directory, link, retire, circuit build, pump, SENDME.
4. Bootstrap stages are `Cold`, `Anchored`, `Joining`, `Ready`. A consensus is refetched once
 `fresh_until` passes. Meanwhile the relays in hand keep serving, and the new set replaces them
 in one step once all its microdescriptors are in and it draws a path
 (`src/manager/refresh_rule.rs`). No relay is used after `valid_until`.
5. At most 3 circuits (one working, one building, one being torn down) and 32 streams, at most
 half of them per owner (`may_open`, `src/stream/owned.rs`). A circuit stops taking new
 streams after 600 s and is retired after 2 failed streams (`src/protocol/limits.rs`).
6. Every turn also ends the streams of API callers that have ended (`src/server/reap.rs`), and
 every 5 s or on a change of stage posts a route report to the attest service
 (`src/server/report/`). The report names no relay; the board takes it only from `net.anon`
 holding Network.
7. A stream's SENDME is withheld once its unread buffer passes one stream increment, and a
 circuit's once its streams pass one circuit increment, bounding heap use by a slow reader.

## Failure model

Every refusal has its own errno (`src/protocol/errno.rs`):

| Errno | Value | Cause |
|---|---|---|
| `E_BAD_MAGIC` / `E_BAD_VERSION` / `E_BAD_OP` / `E_BAD_LEN` | 1 to 4 | malformed request |
| `E_NO_TCP` | 5 | `net.tcp` not yet registered |
| `E_NO_DIRECTORY` | 6 | bootstrap not `Ready` |
| `E_NO_PATH` | 7 | consensus read but no relay fills a position |
| `E_NO_LINK` | 8 | no TLS link to a guard |
| `E_TABLE_FULL` | 9 | stream table full |
| `E_NO_CIRCUIT` / `E_NO_STREAM` | 10, 11 | unknown or not open |
| `E_RX_EMPTY` | 14 | nothing has arrived yet |
| `E_STREAM_CLOSED` | 15 | stream ended; body is reason, retry-another-exit flag, clean flag |
| `E_DIRECTORY_STALE` | 18 | consensus past `valid-until` |
| `E_WOULD_BLOCK` | 20 | send window closed; wait for a SENDME |

A frame that does not parse still gets a reply (`src/server/runner.rs:64`). A consensus with
fewer than a majority of authority signatures (`REQUIRED_SIGNATURES`, 4 of 7) is rejected.
A lost link drops every circuit on it; the next idle turn reconnects to the same guard.

## Current implemented surface

As recorded in `PROTOCOL.md`, "What this capsule implements":

- ntor only, over CREATE2 and EXTEND2. No TAP.
- Link protocol 4 or 5, with the CERTS cell and Ed25519 certificate chain checked
 (`src/link/`).
- Three-hop circuits over guard, middle and exit, drawn by consensus bandwidth weights, with a
 whole /16 excluded per path, not only a relay identity (`src/path/`).
- RELAY_BEGIN streams with authenticated v1 SENDMEs (`src/stream/`, `src/circuit/window.rs`).
- Directory client for the microdescriptor consensus from the seven authorities over DirPort
 9230, with zlib inflate, authority certificate anchoring to hardcoded v3 identities, RSA
 signature quorum, and microdescriptors kept only when their digest matches the consensus
 (`src/directory/`, `src/manager/dir_*.rs`).
- SOCKS5 front on the service port (`src/server/socks/`).

## Wire format

API requests and replies share a 20-byte little-endian header (`src/server/parse_req.rs`,
`src/server/respond.rs`):

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | magic `0x414E4F31` |
| 4 | 2 | version `1` |
| 6 | 2 | op |
| 8 | 2 | errno (reply) |
| 10 | 2 | reserved, zero |
| 12 | 4 | request id, echoed |
| 16 | 4 | payload length, at most 32 KiB |

Bodies, all integers little-endian:

- `OP_OPEN_STREAM`: port `u16`, then host bytes. Reply: stream id `u16`.
- `OP_SEND`: stream id `u16`, then bytes. Reply: bytes accepted `u32`.
- `OP_RECV`: stream id `u16`. Reply: bytes, or on close reason `u8`, retry-another-exit `u8`,
 clean `u8`.
- `OP_CLOSE_STREAM`: stream id `u16`. `OP_CLOSE_CIRCUIT`: circuit id `u32`.
- `OP_STATUS` reply: stage `u8`, relays `u32`, circuits `u32`, open circuits `u32`.
- `OP_CIRCUIT_PATH`: circuit id `u32`. Reply: circuit id `u32`, hop count `u8`, then per hop
 IPv4 `[u8; 4]` and ORPort `u16`.

On the relay side the capsule speaks Tor link cells: 514-byte network cells with a 509-byte
payload and an 11-byte relay header, as listed in `PROTOCOL.md`.

## State ownership

`net.anon` owns everything in `Manager` (`src/manager/state/manager.rs`): bootstrap stage and
retry time, anchored authority certificates, consensus entries, matched microdescriptors,
usable relays and weights, freshness and validity times, the single guard link, the circuit and
stream tables, id counters and cursors, and the chosen guard. `net.tcp` owns the TCP
connections. `crypto_pool` owns RSA verification. The kernel owns none of it.

Each stream belongs to the pid that opened it (`src/stream/owned.rs`). `OP_SEND`, `OP_RECV` and
`OP_CLOSE_STREAM` answer only that pid, and another caller naming the id gets `E_NO_STREAM`.
`OP_CLOSE_CIRCUIT` is refused while the circuit carries a stream another caller opened. Streams
the SOCKS front opens belong to the front, which keeps its own map per caller, so no API request
reaches them. `anon_ntor_proofs/src/tests/stream_owner_tests.rs` holds this.

## Operating rules

- Reach the network only through `net.tcp`; never hold a device or a direct socket.
- Three hops, always.
- Use a consensus only with a majority of authority signatures and only inside its validity
 window.
- One link to one guard, shared by all circuits.
- Answer every request, including malformed and early ones.
- Do not log destinations or payloads, and do not persist any state.

## Release target

The finished capsule bootstraps from the authorities, keeps a verified consensus fresh, builds
and rotates three-hop circuits, carries SOCKS5 and API streams with correct flow control, and
fails each request with the specific errno for its cause, with no state surviving the process
and no routing logic in the kernel.

## Release evidence

- `anon_ntor_proofs`: 126 tests pass on main (
 ). It includes the capsule's own `ntor`, `path/draw.rs`,
 `link/step.rs`, SHA-1, SHA-256 and `manager` sources by `#[path]`
 (`userland/anon_ntor_proofs/src/lib.rs`).
- `anon_link_proofs`: red on main in (rc 101), recorded as
 for want of `vectors/certs_cell.bin`. That vector is in the tree now, with `tls_leaf.der` and
 `source.txt`; no committed log shows the crate passing on it.
- No booted run and no live network log is committed yet; Nym and Anyone are listed as not run. The live measurements in `PROTOCOL.md` are notes, not a committed log.

## Release checklist

- `anon_link_proofs` green on its committed vector, in a committed log.
- `anon_ntor_proofs` stays green.
- A booted run logs consensus quorum, `link open to guard`, `circuit open` and a stream that
 returns data, committed in a log.
- A pcap of that run shows no traffic other than to authorities and the guard.
- Static gate passes for this README and the capsule manifest.

## Explicit non-goals today

Not implemented, as listed in `PROTOCOL.md`: the `.anyone` naming layer, onion services,
conflux, congestion control, ntor v3 and TAP. Also not here: IPv6 relay addresses, a persisted
guard or consensus, bridges or pluggable transports, and a fallback directory mirror set (the
fork ships none).

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build: `make -B nonos-mk-net-anon`
- Kernel profile: `cargo check --no-default-features --features microkernel-desktop-base`
- Route report: `cd userland/route_proof_proofs && cargo test --release`
- Host proofs: `cd userland/anon_ntor_proofs && cargo test --release` and
 `cd userland/anon_link_proofs && cargo test --release`
- Runtime proof: boot with networking, watch the `[ANON]` serial markers reach `circuit open`,
 then fetch a page through the SOCKS front.

See [the Anyone onion routing capsule](../../docs/handbook/network/anyone.md).
