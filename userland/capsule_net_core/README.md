# capsule_net_core

## Role

`capsule_net_core` is the integrated network core capsule. It binds the selected
network device path to smoltcp-backed interface state and exposes compact IPC
operations for DHCP status, DNS resolution, UDP sockets, and TCP sockets.

```text
browser / apps
    |
    | MkIpc socket operations
    v
net.core -- smoltcp iface/sockets --> net device client --> selected NIC capsule
    |
    `-- bounded replies to callers
```

## Microkernel contract

The capsule is an IPC network service. The kernel routes messages and enforces
capabilities; it does not hold TCP state, DNS cache state, DHCP lease state, or
application socket buffers.

- `MkIpcRecv` receives network service requests.
- `MkIpcSend` returns bounded replies.
- The service endpoint is `service:4480:net.core`, the reply endpoint
  `reply:4481:endpoint.net.core.reply`, and the kernel mirror
  `src/userspace/capsule_net_core`.
- At start it registers `net.tcp` (4476), `net.udp` (4472),
  `net.dhcp.client` (4474), `net.dns` (4478) and `net.ip` (4479) on its own
  ports (`src/register.rs`), so callers that look up those names reach it.
  The kernel lets it claim them because it holds RegisterService.
- It is the network stack of the desktop image (`microkernel-desktop-base`)
  and of the `microkernel-net-core`, `microkernel-net-nym` and
  `microkernel-net-sockets` profiles.

## Interface contract

The capsule routes by the magic in each request header. Op 1 is a health
check under any magic.

| Family | Magic | Ops |
|---|---|---|
| TCP | `0x4E544350` | `OP_CONNECT` 3, `OP_SEND` 5, `OP_RECV` 6, `OP_CLOSE` 7, `OP_STATE` 9, `OP_POLL` 10 |
| UDP | `0x4E554450` | `OP_BIND` 2, `OP_UNBIND` 3, `OP_SEND` 4, `OP_RECV` 5 |
| DNS | `0x4E444E53` | `OP_RESOLVE_A` 2 |
| DHCP | `0x4E444843` | `OP_LEASE_STATUS` 3 |
| IP | `0x4E495034` | `OP_SEND_PACKET` 4, `OP_POLL_PACKET` 5, ICMP only |

An unknown magic gets `E_BAD_MAGIC` and an unknown op `E_BAD_OP`. A request that
fails to parse is answered by `refuse`, which echoes the magic, op and request
id it could read, or answers under zeros when the frame is too short.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x0047d`: CoreExec (`0x001`), Network (`0x004`), IPC
(`0x008`), Memory (`0x010`), Crypto (`0x020`, the interface's random seed),
FileSystem (`0x040`) and RegisterService (`0x400`, the five names above).
The capsule owns network stack state only. It does not own MMIO, IRQ, DMA, PIO,
device enumeration, raw PCI config, or admin authority. It holds `FileSystem`
only so autojoin can read the saved Wi-Fi networks through vfs
(`nonos_wifi_client::load`), since vfs serves only a holder of FileSystem.
`Debug` is optional (`0x100`): only a `capsule-serial-debug` build grants it.
Hardware access remains below the selected driver capsule and broker grants.

## Privacy and persistence

The capsule observes IP addresses, DNS names requested by callers, socket peer
addresses, transient packet payloads, and DHCP lease data. It keeps this state
in memory only and does not write packet captures, DNS history, socket history,
or lease history to persistent storage.

## Runtime lifecycle

At start, the capsule registers its names and serves at once, before any
interface is bound. Once a second `reevaluate` binds the stack to the best NIC
driver whose link is up, Wi-Fi cards first, then wired ones, and rebuilds it
when a better link appears. The same tick runs Wi-Fi autojoin: while no
interface is bound, it joins the first saved network in range, only when the
policy store says this boot keeps state, the radio switch is not off and the
driver can join. smoltcp runs DHCP; the lease sets the address, the default
route and the DNS server. A DNS lookup pumps the interface inside its handler
for up to three seconds, and no other caller is served meanwhile.

## Failure model

No device, link down, missing lease, DNS failure, malformed request, unsupported
operation, socket exhaustion, connection refusal, timeout, receive-empty, and
device TX/RX failure are explicit protocol results. They must not panic the
capsule or leak ownership of a caller socket handle.

## Current implemented surface

- Device RX/TX client modules are present.
- smoltcp interface construction and polling are present.
- DHCP, DNS, UDP, and TCP handler modules are present.
- Request parsing and bounded response helpers are present.
- Unknown operations reply with a protocol error.

## State ownership

The capsule owns the smoltcp interface, socket tables, DHCP state, DNS resolver
state, and the selected network device client. Driver capsules own hardware
rings and broker grants. Applications own their own navigation or socket call
intent, not network-global state.

## Operating rules

- Never move TCP, DHCP, DNS, or socket state into the kernel.
- Keep all request and response buffers bounded.
- Return explicit protocol errors for malformed or unsupported operations.
- Keep packet captures and persistent network telemetry out of this capsule.
- A connection or bound port answers only the client that opened it (`src/handles/table.rs`,
  `src/udp_ports/table.rs`).
- Hold one client to half the connections and half the bound ports (`PER_OWNER`), so no
  client, net.sockets included, can refuse every other program a connection or a port.
- Let go of the connections and ports of a client that ended without closing them, as its own
  close and unbind would have: looked for every two seconds, and at once when a connect or bind
  finds no room (`src/server/reap.rs`).
- A rebuilt stack forgets every client's connections and ports with the old socket set
  (`src/state/store.rs`): their handles name sockets in a set that no longer exists, and the
  client's next call is told the socket is gone.

## Release evidence

Release evidence for this capsule is a signed and attested `net.core`, static
proof that it has no hardware authority, DHCP lease evidence, DNS A-resolution
evidence, TCP connect/send/recv/close evidence, UDP bind/send/recv evidence,
and a browser fetch that reaches the GUI through the NØNOS network path.

## Wire format

Every request and reply starts with the 20 byte little endian header the stack
capsules share: magic, version 1, op, errno (in a reply), reserved, request id,
body length. A receive body may be up to `RECV_PAYLOAD_MAX`, 32 KiB. Socket
operations carry caller-owned handles; the capsule owns the backing socket
state.

## Release target

The capsule is built and signed with `make nonos-mk-net-core`,
`nonos-mk-net-core-sign` and `nonos-mk-net-core-verify`, and booted alone with
the `microkernel-net-core` kernel profile. It must boot, acquire or report DHCP
state, resolve DNS, open TCP, move bytes, close sockets, and keep all hardware
access below brokered driver capsules.

## Release checklist

- Root build includes `userland/capsule_net_core/Capsule.mk`.
- `microkernel-net-core` embeds and spawns `net_core`.
- DHCP status returns explicit lease or no-lease state.
- DNS A resolution returns bounded replies.
- TCP connect/send/recv/state/close works through IPC.
- Unknown operations return protocol errors.
- Static gate confirms no kernel packet stack.

## Explicit non-goals today

No raw NIC ownership, packet capture store, persistent DNS cache, browser
history, firewall policy, Tor/Nym routing policy, TLS verification, or GUI
rendering belongs in `net.core`. Those remain in driver capsules, browser
capsule, Nym capsule, or user-facing apps.

Not done: no listening TCP (`OP_LISTEN` and `OP_ACCEPT` get `E_BAD_OP`), no
AAAA lookups, no IPv6, and one thread for every caller.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Capsule build: `make nonos-mk-net-core`
- Kernel profile: `microkernel-net-core` in the root `Cargo.toml`
- Host proofs: `userland/net_proofs` (`core_table_tests.rs` runs the connection and port tables:
  ownership, the per-client share, the take-out of ended clients' entries, and the forget on a
  rebuilt stack).
- Host proofs: `userland/net_core_proofs` (the request header decode, the
  refusal and the reply).
- Runtime proof: DHCP/DNS/TCP socket transaction followed by a browser fetch
  rendered in the GUI through `net.core`.

See [the network stack](../../docs/handbook/network/stack.md).
