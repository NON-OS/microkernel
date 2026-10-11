# capsule_driver_pcnet

## Role

`capsule_driver_pcnet` drives the AMD PCnet family (Am79C970A PCnet-PCI II
and Am79C973 PCnet-FAST III, PCI 1022:2000): VirtualBox's default network
adapter and QEMU's `pcnet`. It sends and receives raw Ethernet frames for the
stack capsules and contains no ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing
or firewall policy.

```text
net.core / net.l2
    |
    | raw frame IPC (NNET)
    v
driver.pcnet0 -- init block + descriptor DMA --> PCnet
```

## Microkernel contract

The capsule uses only Mk/broker interfaces:

- `MkDeviceList` finds the part: AMD, 1022:2000, an Ethernet controller with
  a memory BAR1 of at least 32 bytes.
- `MkDeviceClaim` owns the device claim; `MkPciConfigWrite` sets memory
  decode, Bus Master and Interrupt Disable.
- `MkMmioMap` maps BAR1, the 32-byte memory twin of the I/O register file,
  so the capsule needs no port I/O.
- `MkDmaMap` takes three grants below 4 GiB (`MK_DMA_MAP_DMA32`): one
  coherent page for the init block and both rings, a receive pool and a
  transmit pool. The pools ask for the direction bits (`FROM_DEVICE`,
  `TO_DEVICE`) and are asked again without them from a broker that does not
  know those bits yet.
- No `MkIrqBind`: the driver polls, never sets INEA, and masks every cause.
- `MkIpcRecv` serves `driver.pcnet0` on `service:4274:driver.pcnet0`.

Only `net.core` and `net.l2` may send to `driver.pcnet0`: the kernel holds the
endpoint to them (`src/services/registry/held_table.rs`).

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | alive, with both engines on (CSR0 RXON and TXON) | status word |
| `OP_LINK_STATUS` | BCR4 LEDOUT, the link | 1 byte |
| `OP_MAC_ADDRESS` | the station address drawn this boot | 6 bytes |
| `OP_TX_PACKET` | queue one Ethernet frame | status word |
| `OP_RX_PACKET` | one received frame | length plus frame bytes |
| `OP_RX_BATCH` | every received frame waiting, kept per batch number until the next one | frames |

## Authority

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xB8038`). `Debug` is optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Irq`, no `Pio`, no `Network`: that
belongs to the stack. Crypto draws the per-boot station address.

```text
allowed:   PCnet claim, BAR1 registers, DMA below 4 GiB, kernel randomness, raw-frame IPC
forbidden: IP stack, sockets, packet store, routing policy, kernel drivers, the APROM address
```

## Privacy and persistence

Frames live only in the DMA pools and IPC payloads. The APROM, which holds the
factory address, is never read: the init block carries an address drawn per
bring-up, and that address is the one the part filters on and sends from.

## Runtime lifecycle

Claim, map BAR1, take the three grants; stop the part, reset it (a read of
RESET), require CSR0 to read STOP alone, set SWSTYLE 2 in BCR20 while stopped;
read the chip id from CSR88/89; draw the station address; hand every receive
descriptor to the part; write the init block; mask every cause in CSR3, set
CSR4; point CSR1/CSR2 at the init block; INIT and wait for IDON; STRT and wait
for RXON and TXON; serve. Teardown stops and resets the part before the first
grant goes back, so no DMA lands in pages the broker hands on.

## Failure model

Every wait has a deadline. A part that never reads STOP after the reset, an id
without AMD's code, an init block never taken, or a part that never starts is
refused with a named reason; the part is stopped and reset and every grant and
the claim go back, last taken first. No entropy for the station address means
the part is never initialised. Bring-up is tried a bounded number of times
(`nonos_libc::bring_up`): exit 2 with no PCnet present, 6 when one never comes
up. A memory error or babble that stops the part shows as `OP_HEALTHCHECK`
failing with `-5`.

## Current implemented surface

- SWSTYLE 2 (32-bit, 16-byte descriptors), 32 receive and 32 transmit slots of
  1536 bytes.
- FCS cut from every received frame; errored, chained, runt and oversized
  frames dropped and their slots recycled.
- Short frames padded with zeros in the buffer to 60 bytes.
- Link, MAC, RX, RX batch and TX over IPC.

## Wire format

Requests use the `NNET` header, version `1`, the shared 20-byte driver
envelope, and a payload length that must match the message exactly. Replies
start with a 4-byte status word: `-22` bad request, `-90` size mismatch, `-11`
nothing received or no free transmit slot, `-5` a dropped frame or a stopped
part.

## State ownership

The capsule owns the control page, both pools, the register grant and the
station address. `net.core` and `net.l2` own Ethernet and ARP.

## Operating rules

- Word access only: a doubleword store to RDP would switch the part to DWIO.
- OWN is stored last on every descriptor, after a release fence.
- Never read the APROM.

## Release target

A signed raw-frame NIC service that takes a DHCP lease through VirtualBox's
default adapter and QEMU's `pcnet`, protocol-blind.

## Release evidence

`pcnet_proofs` holds the driver to a register-level model of the Am79C970A:
bring-up order, the init block, descriptor ownership, FCS handling, batch
bodies, every refusal path and its cleanup, and DMA confined to the grants.
A boot under QEMU `-device pcnet` and under VirtualBox is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_pcnet` keys enrolled).
- `pcnet_proofs` passes.
- QEMU `pcnet` and VirtualBox PCnet-FAST III take a DHCP lease.

## Explicit non-goals today

No ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing, firewall, multicast group
filtering, interrupts, or the ISA parts (Am79C960/961).

## Verification

- Build: `make -B nonos-mk-driver-pcnet`
- Proofs: `cd userland/pcnet_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
