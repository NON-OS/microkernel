# capsule_driver_rtl8169

## Role

`capsule_driver_rtl8169` is the Realtek RTL8168/RTL8169 gigabit Ethernet
capsule. It owns MMIO device registers, DMA rings, and raw Ethernet frame
movement. It polls; no interrupt is bound.

```text
net.core / net.l2
    |
    | raw frame IPC
    v
driver.rtl8169_0 -- MMIO / DMA broker grants --> RTL8169 NIC
```

## Microkernel contract

The capsule uses the broker for all hardware access:

- `MkDeviceList` locates the Realtek gigabit NIC.
- `MkDeviceClaim` owns the NIC claim.
- `MkMmioMap` maps the register BAR.
- `MkDmaMap` and `MkDmaUnmap` allocate RX/TX descriptors and packet buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.rtl8169_0` on
  `service:4214:driver.rtl8169_0`.
- `CryptoRandom` draws the station address each boot (`src/init/mac.rs`).

The kernel mediates grants and teardown only. Network policy is userland. Only
`net.core` and `net.l2` may send to `driver.rtl8169_0`: the kernel holds the
endpoint to them (`src/services/registry/held.rs`), by name and by pid.

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

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xB8038`). Crypto draws the per-boot station
address; the driver polls, so no `Irq`. It has no PIO, socket,
filesystem, admin, debug, or routing authority.

```text
allowed:   NIC claim, MMIO, DMA rings, kernel randomness, raw-frame IPC
forbidden: PIO, IP stack, sockets, persistent packet store, firewall
```

## Privacy and persistence

Network frames are transient. The capsule holds payloads in DMA buffers and
IPC buffers only while servicing RX/TX. It does not persist traffic, keep peer
identity, or implement analytics.

## Runtime lifecycle

The capsule claims the NIC, turns on bus mastering, maps MMIO, allocates RX/TX
DMA rings, programs the device, draws a station address, and serves raw-frame
IPC. It binds no interrupt. Teardown disables the device path and releases
DMA, MMIO, and claim grants.

## Failure model

With no supported card in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. A card that is present but fails
setup or programming gives back every grant, and the attempt is repeated on
the shared bounded schedule (`nonos_libc::bring_up`: seven tries, sleeping
between them); running out exits `EXIT_GAVE_UP` (6).

Setup failure rolls back previous grants. Runtime TX/RX errors are reported to
callers as device faults. Empty RX and link-down are ordinary reported states,
not kernel events.

## Current implemented surface

- Builds as a signed driver capsule.
- Uses MMIO/DMA broker primitives.
- Advertises `driver.rtl8169_0`.
- Keeps packet protocol state above the NIC driver boundary.
- Reports command, PHY, interrupt, config, ring-size, and software cursor state
  without reading counters that clear on access.
- Has a kernel mirror (`src/hardware/rtl8169_capsule`) and is one of the
  wired cards `net.core` and `net.l2` look up by name.

## Wire format

Requests use the `NR69` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. MAC replies carry
6 bytes. Link replies carry 1 byte. RX replies carry length plus frame bytes.
TX requests carry one Ethernet frame bounded by `MAX_ETHERNET_FRAME`.
`OP_STATS` returns twelve little-endian `u32` fields:
`CMD`, `PHY_STATUS`, `ISR`, `IMR`, `RX_CONFIG`, `TX_CONFIG`, `RMS`, `rx_cur`,
`tx_cur`, `rx_desc_count`, `tx_desc_count`, and `reserved`.

## State ownership

The capsule owns MMIO mapping, DMA descriptor rings, packet buffers,
station address, link state, and TX/RX device state. `net.core`, `net.l2` and
higher capsules own all protocol interpretation.

## Operating rules

- Keep RTL8169 MMIO-only; do not request PIO grants.
- Keep payload retention bounded to RX/TX service paths.
- Report link-down and RX-empty as ordinary states.
- Do not add IP, socket, or firewall logic to the driver.

## Release target

The finished RTL8169 capsule is a signed gigabit raw-frame service with MMIO
register ownership, DMA descriptor lifecycle, interrupt recovery, link/MAC
reporting, side-effect-free telemetry, and frame delivery to `net.core`.
Promotion requires QEMU validation and compatible hardware proof.

## Release evidence

Release requires RTL8169-compatible hardware proof, emulator validation where
available, `net.core` frame round trip, teardown grant proof, and link-change
behavior.

## Release checklist

- Signed manifest and kernel mirror present.
- Compatible hardware boot proves MAC/link/TX/RX.
- `net.core` frame round trip passes.
- Teardown proof shows MMIO/DMA/device claim revocation.
- Static gate confirms no PIO use.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- The reset bit self-clears; TE and RE take before RxConfig and TxConfig on the real 8168/8169 variant.
- The station address in use is the drawn one and changes on every boot.
- ISR latches with IMR left at zero (no line is bound).
- DHCP completes; TX and RX hold up under load.

## Explicit non-goals today

No ARP, IP, DHCP, DNS, TCP, UDP, sockets, routing, firewall, capture store,
or hardware-offload policy is implemented here.

## Verification

- Build: `make -B nonos-mk-driver-rtl8169`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: RTL8169 must stay MMIO-only and must not request PIO
  grants.
- Documentation check: this README is required by the driver docs gate.
- Handbook: [drivers](../../docs/handbook/drivers.md).
