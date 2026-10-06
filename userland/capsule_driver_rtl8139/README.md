# capsule_driver_rtl8139

## Role

`capsule_driver_rtl8139` is the Realtek RTL8139 Fast Ethernet capsule. It is a
PIO NIC driver that moves raw Ethernet frames between the device and userland
network capsules.

```text
RTL8139 port BAR -- MkPio* --> driver.rtl8139_0
       |
       +-- MkDmaMap --------> packet buffers
       `-- polled: no interrupt is bound
```

## Microkernel contract

The capsule reaches the NIC through broker authority only:

- `MkDeviceList` locates the RTL8139 PCI function.
- `MkDeviceClaim` owns the NIC claim.
- `MkPioGrant`, `MkPioRead`, and `MkPioWrite` access the port BAR.
- `MkDmaMap` and `MkDmaUnmap` allocate packet buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.rtl8139_0` on
  `service:4212:driver.rtl8139_0`.
- `CryptoRandom` draws the station address each boot (`src/init/mac.rs`).

The kernel does not route packets or retain network policy. Only `net.core` and
`net.l2` may send to `driver.rtl8139_0`: the kernel holds the endpoint to them
(`src/services/registry/held.rs`), by name and by pid.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_LINK_STATUS` | link-up state | 1 byte |
| `OP_MAC_ADDRESS` | the station address drawn this boot | 6 bytes |
| `OP_TX_PACKET` | transmit one Ethernet frame | status word |
| `OP_RX_PACKET` | poll one received frame | length plus frame bytes |
| `OP_STATS` | non-mutating register and ring snapshot | 48 bytes |

## Authority

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Dma`
and `Pio` (`CAPSULE_REQUIRED_CAPS = 0x198038`). Crypto draws the per-boot station
address; the driver polls, so no `Irq`. It has no MMIO, socket,
filesystem, admin, debug, or routing authority.

```text
allowed:   port BAR PIO, DMA packet buffers, kernel randomness, raw-frame IPC
forbidden: MMIO, IP stack, socket table, packet capture storage, firewall
```

## Privacy and persistence

RX/TX payloads are ephemeral. The capsule does not persist frames, keep peer
history, store packet captures, or log payloads. Buffers are broker resources
and are revoked on exit.

## Runtime lifecycle

The capsule claims the NIC, turns on bus mastering, takes the PIO grant,
allocates packet buffers, initializes device registers, draws a station
address, and serves raw-frame IPC. It polls and binds no interrupt. Teardown
releases DMA, PIO, and claim grants.

## Failure model

With no supported card in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. A card that is present but fails
setup or programming gives back every grant, and the attempt is repeated on
the shared bounded schedule (`nonos_libc::bring_up`: seven tries, sleeping
between them); running out exits `EXIT_GAVE_UP` (6).

Setup failure rolls back previous grants. TX/RX errors are reported as NIC
faults. Empty RX is non-fatal. Port access remains broker-mediated for every
hardware read/write.

## Current implemented surface

- Builds as a signed driver capsule.
- Claims the NIC and owns the port and DMA broker path.
- Advertises `driver.rtl8139_0`.
- Keeps protocol handling above the driver boundary.
- Reports command, link, interrupt, RX, TX, and software cursor state without
  reading counters that clear on access.
- Is one of the wired cards `net.core` and `net.l2` look up by name.

## Wire format

Requests use the `NR89` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. MAC replies carry
6 bytes. Link replies carry 1 byte. RX replies carry length plus frame bytes.
TX requests carry one Ethernet frame bounded by `MAX_ETHERNET_FRAME`.
`OP_STATS` returns twelve little-endian `u32` fields:
`CMD`, `MSR`, `ISR`, `RCR`, `TCR`, `CAPR`, `TXSTATUS0..3`, `rx_offset`,
and `tx_cur`.

## State ownership

The capsule owns the PIO grant, DMA packet buffers, station address, link
state, and TX/RX device state. Network protocol capsules own every byte above
the Ethernet frame boundary.

## Operating rules

- Keep RTL8139 as PIO-only; do not add MMIO grants.
- Keep all port access broker-mediated.
- Never persist packet payloads.
- Report RX-empty, link-down, and TX fault explicitly.

## Release target

The finished RTL8139 capsule is a signed raw-frame NIC service with port-BAR
PIO access through the broker, RX/TX buffer management, interrupt recovery,
link/MAC reporting, side-effect-free telemetry, and frame delivery to `net.core`.
It must pass QEMU and hardware validation before being promoted beyond build-only.

## Release evidence

Release requires QEMU `rtl8139` round trip, broker PIO proof, RX/TX validation
through `net.core`, teardown grant proof, and one compatible hardware boot.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU RTL8139 TX/RX validation passes through `net.core`.
- PIO gate proves no inline port assembly.
- Teardown proof shows PIO/DMA/device claim revocation.
- Hardware boot records link, MAC, RX, and TX behavior.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- The port grant reaches the part; the reset bit self-clears.
- The station address in use is the drawn one and changes on every boot.
- A frame that crosses the end of the receive ring is read whole (from the slack after the ring, with RCR.WRAP set).
- After a FIFO overrun, receive restarts and keeps going.
- DHCP completes; TX and RX hold up under load.

## Explicit non-goals today

No ARP, IP, DHCP, DNS, TCP, UDP, sockets, routing, firewall, offload policy,
or packet capture facility belongs in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-rtl8139`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: RTL8139 must not map MMIO and must use broker PIO
  wrappers rather than inline assembly.
- Documentation check: this README is part of CI's driver-capsule contract.
- Handbook: [drivers](../../docs/handbook/drivers.md).
