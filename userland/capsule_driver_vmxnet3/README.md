# capsule_driver_vmxnet3

## Role

`capsule_driver_vmxnet3` drives VMware's paravirtual network adapter
(vmxnet3, PCI 15ad:07b0) in VMware Workstation, Fusion and ESXi, and QEMU's
`vmxnet3`. It sends and receives raw Ethernet frames for the stack capsules
and contains no ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing or firewall
policy.

```text
net.core / net.l2
    |
    | raw frame IPC (NNET)
    v
driver.vmxnet3_0 -- shared area + queue DMA --> vmxnet3 (revision 1)
```

## Microkernel contract

- `MkDeviceList` finds the device: VMware, 15ad:07b0, with memory BAR0 and
  BAR1 of at least 4 KiB each.
- `MkDeviceClaim` owns the claim; `MkPciConfigWrite` sets memory decode,
  Bus Master and Interrupt Disable.
- `MkMmioMap` maps BAR0 (interrupt masks, producer doorbells) and BAR1
  (revision, shared-area address, commands, MAC).
- `MkDmaMap` takes three grants: a coherent control grant (DriverShared, the
  queue table and every ring), a receive pool asked `FROM_DEVICE` and a
  transmit pool asked `TO_DEVICE`, asked again without the direction bits
  from a broker that does not know them yet. The device takes 64-bit
  addresses, so no grant is held below 4 GiB.
- No `MkIrqBind`: the driver polls; intrCtrl disables every interrupt and
  interrupt 0 is masked.
- `MkIpcRecv` serves `driver.vmxnet3_0` on `service:4276:driver.vmxnet3_0`.

Only `net.core` and `net.l2` may send to `driver.vmxnet3_0`: the kernel
holds the endpoint to them (`src/services/registry/held_table.rs`).

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | alive, and GET_QUEUE_STATUS shows neither queue stopped and no queue error in ECR | status word |
| `OP_LINK_STATUS` | GET_LINK bit 0 | 1 byte |
| `OP_MAC_ADDRESS` | the station address drawn this boot | 6 bytes |
| `OP_TX_PACKET` | queue one Ethernet frame | status word |
| `OP_RX_PACKET` | one received frame | length plus frame bytes |
| `OP_RX_BATCH` | every received frame waiting, kept per batch number until the next one | frames |

## Authority

`IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio` and `Dma`
(`CAPSULE_REQUIRED_CAPS = 0xB8038`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Irq`, `Pio` or `Network`.

```text
allowed:   vmxnet3 claim, BAR0/BAR1 registers, DMA grants, kernel randomness, raw-frame IPC
forbidden: IP stack, sockets, packet store, routing policy, kernel drivers, the host's MAC
```

## Privacy and persistence

Frames live only in the pools and IPC payloads. The hypervisor's address is
never asked for (no GET_PERM_MAC): MACL/MACH carry an address drawn per
bring-up, which is the one the device filters on.

## Runtime lifecycle

Claim, map BAR0 and BAR1, take the three grants; select revision 1 and UPT 1
(VRRS, UVRS); RESET_DEV; draw the station address; clear both queues and hand
31 of 32 ring 0 descriptors over (ring 1 stays empty: no frame outgrows a
2 KiB head buffer); write DriverShared and the queue table; set MACL/MACH and
DSAL/DSAH; ACTIVATE_DEV, whose answer in CMD must be 0; mask interrupt 0;
write RXPROD; UPDATE_RX_MODE, UPDATE_VLAN_FILTERS, UPDATE_MAC_FILTERS; serve.
Teardown is QUIESCE_DEV, RESET_DEV and a cleared shared address before the
first grant goes back.

## Failure model

A host without revision 1, no entropy, or a refused activation fails the
attempt with a named reason; the device is quiesced and reset, the shared
address cleared, and every grant and the claim go back, last taken first.
Bring-up is tried a bounded number of times (`nonos_libc::bring_up`): exit 2
with no vmxnet3 present, 6 when one never comes up. No wait loops: every
command answers synchronously.

## Current implemented surface

- Revision 1, one transmit and one receive queue, 32 slots each, 2 KiB
  buffers, generation-bit ownership on every ring.
- Unicast to the drawn address and broadcast; VLAN 0 (untagged) only.
- No checksum offload, LRO, VLAN stripping or RSS: frames arrive as they were
  on the wire.

## Wire format

`NNET` header, version `1`, 20-byte envelope, payload length exactly matching
the message. Replies start with a status word: `-22` bad request, `-90` size
mismatch, `-11` nothing received or no free transmit slot, `-5` a stopped
queue.

## State ownership

The capsule owns the control grant, both pools, the register grants and the
station address. `net.core` and `net.l2` own Ethernet and ARP.

## Operating rules

- The generation bit is written last on every descriptor, after a release
  fence; a completion is read only after its generation bit matches.
- A receive completion for any slot but the next expected is ignored.
- Never ask the device for its permanent address.

## Release target

A signed raw-frame NIC service that takes a DHCP lease through VMware's
vmxnet3 and QEMU's `vmxnet3`.

## Release evidence

`vmxnet3_proofs` holds the driver to a model of the device: the shared area
checked byte by byte against vmxnet3_defs.h, ACTIVATE_DEV refusing a bad
layout, both queues across many generation flips, dropped and stray
completions, the refusal paths and their cleanup, and DMA confined to the
grants. A boot under QEMU `-device vmxnet3` and in VMware is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_vmxnet3` keys enrolled).
- `vmxnet3_proofs` passes.
- QEMU `vmxnet3` and VMware take a DHCP lease.

## Explicit non-goals today

No ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing, firewall, multicast, VLAN
tagging, offloads, multiple queues or interrupts.

## Verification

- Build: `make -B nonos-mk-driver-vmxnet3`
- Proofs: `cd userland/vmxnet3_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
