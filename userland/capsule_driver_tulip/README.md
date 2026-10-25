# capsule_driver_tulip

## Role

`capsule_driver_tulip` drives the DEC 21140 and 21143 ("tulip", PCI 1011:0009
and 1011:0019): Hyper-V's legacy network adapter, which generation 1 virtual
machines use before their synthetic one is up, and QEMU's `tulip`. It sends
and receives raw Ethernet frames for the stack capsules and contains no ARP,
IP, TCP, UDP, DHCP, DNS, sockets, routing or firewall policy.

```text
net.core / net.l2
    |
    | raw frame IPC (NNET)
    v
driver.tulip0 -- CSR window + descriptor DMA --> 21140 / 21143
```

## Microkernel contract

The capsule uses only Mk/broker interfaces:

- `MkDeviceList` finds the part: DEC, 1011:0009 or 1011:0019, an Ethernet
  controller with a memory BAR1 of at least 128 bytes.
- `MkDeviceClaim` owns the device claim; `MkPciConfigWrite` sets memory
  decode, Bus Master and Interrupt Disable.
- `MkMmioMap` maps BAR1, the memory twin of the CSR file, so the capsule
  needs no port I/O.
- `MkDmaMap` takes three grants below 4 GiB (`MK_DMA_MAP_DMA32`): one
  coherent page for both rings, a receive pool and a transmit pool, the
  pools asking for the direction bits and asked again without them from a
  broker that does not know those bits yet.
- No `MkIrqBind`: the driver polls with every interrupt masked in CSR7.
- `MkIpcRecv` serves `driver.tulip0` on `service:4286:driver.tulip0`.

Only `net.core` and `net.l2` may send to `driver.tulip0`: the kernel holds the
endpoint to them (`src/services/registry/held_table.rs`).

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | alive, both processes running, no fatal bus error | status word |
| `OP_LINK_STATUS` | up while both processes run (no PHY to ask) | 1 byte |
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
allowed:   tulip claim, BAR1 CSRs, DMA below 4 GiB, kernel randomness, raw-frame IPC
forbidden: IP stack, sockets, packet store, routing policy, the serial ROM's address
```

## Privacy and persistence

Frames live only in the DMA pools and IPC payloads. The serial ROM, which
holds the factory address, is never read: the setup frame loads an address
drawn per bring-up into the perfect filter with broadcast, and the stack
sends from it. No multicast and no promiscuous mode.

## Runtime lifecycle

Claim, command bits, map BAR1, take the three grants; stop both processes,
software reset (CSR0.SWR), require SWR to clear and both processes to read
stopped; bus mode with back-to-back descriptors; mask every interrupt in
CSR7 and clear CSR5; draw the station address; hand every receive
descriptor to the part (end-of-ring on the last); point CSR3 and CSR4 at the
rings; post the setup frame and start the transmit process alone; once the
part has taken the frame, start the receive process too; serve. Each
recycled receive slot is followed by a receive poll demand, which wakes a
receive process that suspended for want of a buffer. Teardown stops and
resets the part before the first grant goes back.

## Failure model

Every wait has a deadline. A part that never leaves reset, a setup frame never
taken, or a part that never starts is refused with a named reason; the part
is stopped and reset and every grant and the claim go back, last taken first.
No entropy for the station address means the part is never started. Bring-up
is tried a bounded number of times (`nonos_libc::start_driver`). A fatal bus
error shows as `OP_HEALTHCHECK` failing with `-5`.

## Current implemented surface

- 16-byte descriptors in ring mode, 32 receive and 32 transmit slots of 1536
  bytes, buffer 1 only.
- FCS cut from every received frame; errored, split, runt and oversized
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

- Longword access only, CSR `n` at `n * 8`.
- OWN is stored last on every descriptor, after a release fence.
- CSR6 is written whole: out of reset QEMU's part has promiscuous mode set.
- Never read the serial ROM.

## Release target

A signed raw-frame NIC service that takes a DHCP lease through Hyper-V's
legacy network adapter and QEMU's `tulip`, protocol-blind.

## Release evidence

`tulip_proofs` holds the driver to a register-level model of the 21x4x:
the reset, the processes, the setup frame and the filter it loads, descriptor
ownership and end-of-ring, a receive process suspended and woken, FCS
handling, batch bodies, every refusal path and its cleanup, and DMA confined
to the grants. A boot under Hyper-V and QEMU `-device tulip` is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_tulip` keys enrolled).
- `tulip_proofs` passes.
- Hyper-V's legacy network adapter and QEMU `tulip` take a DHCP lease.

## Explicit non-goals today

No ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing, firewall, multicast group
filtering, interrupts, media selection, or the 21040/21041.

## Verification

- Build: `make -B nonos-mk-driver-tulip`
- Proofs: `cd userland/tulip_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
