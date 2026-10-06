# capsule_driver_e1000

## Role

`capsule_driver_e1000` is the Intel 8254x Ethernet driver capsule. It owns the
PCI NIC, MMIO registers, RX/TX DMA rings, and raw Ethernet frame movement.
It deliberately stops at the Ethernet frame boundary.

```text
net.core / net.l2
    |
    | raw Ethernet IPC
    v
driver.e1000_0 -- broker MMIO + DMA rings --> Intel e1000 NIC
    |
    `-- polled: no interrupt is bound
```

## Microkernel contract

Hardware access is mediated by Mk and broker syscalls:

- `MkDeviceList` locates the Intel NIC record.
- `MkDeviceClaim` owns the NIC claim and claim epoch.
- `MkMmioMap` maps BAR0.
- `MkDmaMap` and `MkDmaUnmap` allocate RX/TX descriptors and packet buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.e1000_0` on
  `service:4210:driver.e1000_0`.
- `CryptoRandom` draws the station address each boot.

No interrupt is bound. The driver polls its rings and never sets IMS
(`src/setup/sequence.rs`): an INTx line it held and never serviced would stay
masked and starve any device sharing it.

Only `net.core` and `net.l2` may send to `driver.e1000_0`. The kernel holds the
endpoint to them (`src/services/registry/held.rs`), by name and by pid.

The kernel never parses Ethernet, ARP, IP, TCP, UDP, DNS, DHCP, or socket
state. It only enforces capabilities and revokes grants.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_LINK_STATUS` | link-up state | 1 byte |
| `OP_MAC_ADDRESS` | the station address drawn this boot | 6 bytes |
| `OP_TX_PACKET` | transmit one Ethernet frame | status word |
| `OP_RX_PACKET` | poll one received frame | length plus frame bytes |
| `OP_STATS` | live register and ring cursor snapshot | 48-byte state record |

## Authority

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xB8038`). `Crypto` is there for
`CryptoRandom`, which draws the station address. It has no `Irq`, socket,
routing, firewall, filesystem, admin, or debug authority.

```text
allowed:   NIC claim, BAR0 MMIO, RX/TX DMA, kernel randomness, raw-frame IPC
forbidden: IP policy, socket policy, packet capture store, kernel drivers
```

## Privacy and persistence

Frames are transient. Packet data lives in DMA buffers and IPC payloads only
for the duration of RX/TX handling. The capsule does not persist captures,
record peers, keep application identity, or log payloads by default.

## Runtime lifecycle

The capsule claims the NIC, maps BAR0, allocates RX/TX DMA rings, resets and
programs the device, draws a locally administered station address with
`CryptoRandom` (the EEPROM address is not used), and serves raw-frame IPC. It
polls the rings and enables no interrupt. Teardown disables the device path and returns all broker grants.

## Failure model

With no e1000 in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. A card that is present but fails
setup or programming gives back every grant, and the attempt is repeated on
the shared bounded schedule (`nonos_libc::bring_up`: seven tries, sleeping
between them); running out exits `EXIT_GAVE_UP` (6).

Setup failure rolls back grants in reverse order. Runtime TX failure returns a
NIC fault without retrying inside the kernel. RX empty is non-fatal. Link-down
is reported to callers rather than hidden.

## Current implemented surface

- Claims an e1000 PCI NIC through the broker.
- Maps BAR0 and resets/programs device registers.
- Allocates RX/TX descriptor rings and packet buffers through `MkDmaMap`.
- Draws a new locally administered station address every boot
  (`src/init/station_address.rs`, through `nonos_mac`).
- Serves link, MAC, RX packet, and TX packet operations over IPC.
- Serves a side-effect-free register/ring state snapshot over IPC.
- Rolls broker grants back during setup failure and process teardown.

## Wire format

Requests use the `NE10` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies start with a 4-byte status word. MAC replies carry
6 bytes. Link replies carry 1 byte. RX replies carry a 4-byte frame length
followed by the Ethernet frame. TX requests carry one Ethernet frame bounded
by `MAX_ETHERNET_FRAME`. Stats replies carry twelve little-endian `u32`
values:

```text
STATUS, RCTL, TCTL, RDH, RDT, TDH, TDT,
rx_head, tx_tail, rx_desc_count, tx_desc_count, reserved
```

## State ownership

The capsule owns descriptor rings, packet buffers, MMIO register state, link
snapshot and station address. `net.core` and `net.l2` own protocol
interpretation. The kernel owns only grant records.

## Operating rules

- Keep the service frame-oriented; never add ARP/IP/TCP/UDP branches here.
- Refill RX buffers before exposing receive service as release-grade.
- Treat link-down and RX-empty as normal return states, not kernel events.
- Never retain packet payloads after the reply or TX completion path ends.

## Release target

The finished e1000 capsule is a signed, embedded, spawned raw-frame NIC service
with QEMU and hardware validation coverage. It owns link bring-up, interrupt
recovery, RX/TX ring refill, side-effect-free register telemetry, and frame
delivery to `net.core`. It never grows ARP, IP, sockets, firewall, or capture
policy.

## Release evidence

Release requires QEMU `e1000` frame round trip, link-down behavior, teardown
DMA revocation proof, and one compatible hardware boot with RX/TX counters
moving through `net.core`.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU e1000 TX/RX validation passes through `net.core`.
- Link state changes are visible over IPC.
- DMA teardown proof shows descriptor and packet buffers are revoked.
- Hardware boot records MAC, link, RX, and TX without kernel packet parsing.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- The 8254x completes CTRL.RST within 50 ms and the EEPROM reload within 20 ms.
- Link comes up (link status reports up) with no interrupt line bound.
- The station address in use is the drawn, locally administered one, not the EEPROM address, and it changes on every boot.
- DHCP completes through the network stack; TX and RX hold up under sustained load with no ring stall.

## Explicit non-goals today

No ARP, IP, TCP, UDP, DHCP, DNS, routing, sockets, firewall, packet capture,
traffic analytics, or RSS/multi-queue policy lives in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-e1000`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: e1000 must remain free of kernel driver imports and use
  broker MMIO/DMA only.
- Handbook: [drivers](../../docs/handbook/drivers.md).
- Documentation check: this README is required by CI and must cover authority,
  privacy, current surface, release evidence, non-goals, and verification.
