# capsule_driver_netvsc

## Role

`capsule_driver_netvsc` drives Hyper-V's synthetic network adapter: the
network VSC a Hyper-V or Azure guest reaches its network through, over a
VMBus channel. It serves Ethernet frames to net.core on the NNET wire,
through `nonos_usbnet`'s frame service. The channel (rings, GPADLs, open and
close) is `nonos_vmbus`'s; this capsule speaks NVSP and RNDIS over it.

```text
net.core --NNET--> driver.netvsc0 -- nonos_usbnet::nnet --> Net (Nic)
   send buffer section --SEND_RNDIS_PACKET--> Hyper-V --transfer pages--> receive buffer
```

## Microkernel contract

- The VMBus core (`driver.vmbus0`) learns the adapter's channel from the
  host's offer and has the kernel make it a record (`MkHvChannel`): vendor
  0x1414, device 0x4E45, on the virtual bus, the relid in `irq_source`.
  Only `driver.netvsc0` may claim it. The record's class is NETWORK, whose
  ceiling is 64 pages a grant.
- `MkDeviceList` finds the record, every 100 ms for up to 15 s.
- `MkDeviceClaim` of the record; `MkDmaMap` three times, coherent and 64
  pages each: the rings (32 pages each way), the receive buffer, the send
  buffer. A channel record has no requester id, so its grants carry
  physical addresses, under the no-IOMMU posture or the development opt-in.
- `MkHvPost` on the record, connection 0: three GPADLs (rings, receive
  buffer, send buffer), OPENCHANNEL, and at the end CLOSECHANNEL and the
  teardowns. `MkHvRecv` on the record for this channel's answers alone.
  `MkHvSignal` on the record.
- `CryptoRandom` for the drawn station address and the probe's DHCP
  transaction id.
- No `MkIrqBind`: the driver polls the host's ring.
- `MkIpcRecv` serves `service:4306:driver.netvsc0`; replies to the caller.

## Interface contract

NNET, as `nonos_usbnet::nnet` serves it to every NIC driver: HEALTHCHECK
1, LINK_STATUS 2, MAC_ADDRESS 3, TX_PACKET 4, RX_PACKET 5, STATS 7. A frame
is 14 to 1514 bytes, no FCS.

## Authority

`IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Dma`
(`CAPSULE_REQUIRED_CAPS = 0x98038`); `Debug` optional (`0x100`). No `Mmio`,
`Pio`, `Irq` or `Network`.

## Privacy and persistence

The driver keeps no frame beyond the one it hands over, and no address
beyond the run: the station address is drawn each run and kept nowhere. Its
console lines name no address. The host-assigned address is used only when
the host will not carry the drawn one, and that is said (below).

## The station address

Every NIC driver serves a station address drawn for the run, never the
device's own. Hyper-V's switch drops, without a word, every frame from an
address other than the one it assigned the port, unless the VM's adapter
has MAC spoofing on. So before the link is ever reported up (net.core reads
the address once, when it binds), the driver settles it, as the lead ruled:

1. It offers the drawn address to the host (RNDIS
   OID_GEN_RNDIS_CONFIG_PARAMETER "NetworkAddress", as Linux's
   `rndis_filter_set_device_mac`). A host that refuses it leaves the
   host-assigned address at once.
2. Otherwise it sends a DHCP DISCOVER from the drawn address, three times
   two seconds apart, then waits two seconds more. A frame sent to the drawn
   address, or a DHCP reply for its transaction naming it, keeps the drawn
   address.
3. Nothing within that bounded wait, and the host-assigned address is used,
   and the console says:
   `[NET] netvsc: Hyper-V drops a chosen MAC (MAC spoofing off); using the host-assigned MAC`
4. Which of these it was is in the NNET STATS reply, word 9, for Settings'
   network page: 1 the drawn address, 2 the host-assigned one for want of
   an answer, 3 the host-assigned one the host's refusal left
   (`nonos_usbnet::nnet::STATION_*`; 0 is every other driver's).

The fallback hides nothing from the host, which assigned that address. It
does show the address to the network the VM is on. **To keep the drawn
address on Hyper-V, turn MAC spoofing on for the VM's network adapter**
(Hyper-V Manager: the adapter's Advanced Features, "Enable MAC address
spoofing"; PowerShell: `Set-VMNetworkAdapter -VMName <vm>
-MacAddressSpoofing On`). A network with no DHCP server also falls back, as
nothing answers there either.

The DISCOVER is the one frame the driver sends of its own: option 53 alone,
to the broadcast address, `sname` and `file` empty, within the egress
rules for DHCP (`docs/security/privacy.md`).

## Current implemented surface

- One adapter, one channel, frames of 14 to 1514 bytes both ways, the
  host's link state, and the station address settled at bring-up.

## Runtime lifecycle

Claim, the three grants, both rings' headers set empty, the rings' GPADL
and OPENCHANNEL (`nonos_vmbus::open`). Then NVSP in the order netvsp takes
it: INIT for 6.1 down to 1; from version 2, SEND_NDIS_CONFIG (MTU 1514)
before the receive buffer; SEND_NDIS_VERSION (6.30 from protocol 5, else
6.1); the receive buffer's GPADL and SEND_RECEIVE_BUFFER (id 0xCAFE), whose
one section must lie within the buffer; the send buffer's GPADL and
SEND_SEND_BUFFER (id 0), whose section size the host chooses. Then RNDIS on
the control channel: INITIALIZE, the host-assigned address
(OID_802_3_PERMANENT_ADDRESS), the packet filter (directed, all multicast,
broadcast; without it netvsp drops every frame), and the link
(OID_GEN_MEDIA_CONNECT_STATUS). Then the station address, above. Then the
NNET service, with the channel serviced between requests.

A frame goes out as an RNDIS data message in a free send buffer section,
named in SEND_RNDIS_PACKET on the data channel; the section is the host's
until its completion. Frames come in as transfer-pages packets naming ranges
of the receive buffer, each an RNDIS message; the driver reads each range,
keeps the frames, notes link indications, and completes the packet so the
host may use the ranges again. A completion the ring has no room for is
owed and sent first next time.

## Failure model

Every bring-up step names its failure and gives back what was taken, the
claim last. A range past the receive buffer, or a message that does not fit
its own lengths, is passed over and the packet still completed. A host that
holds every section makes a send wait one second, then fail as busy. A ring
the host broke stops the driver: it closes the channel, takes its GPADLs
back, gives back the grants and the claim, and exits.

## State ownership

The capsule owns the channel while it is open, its rings and both buffers,
and the station address it settled. The VMBus core owns the bus; the kernel
owns the channel table and takes a channel's pages back from the host when a
driver leaves without. net.core owns what is sent and received, and when.

## Operating rules

- Never report the link up before the station address is settled.
- Never serve the host-assigned address without saying so on the console.
- A send buffer section is the host's from SEND_RNDIS_PACKET until its
  completion; a receive range is the driver's until it completes the packet.
- Never send a frame of its own but the probe's DISCOVERs.

## Wire format

NVSP as OpenVMM's `netvsp_protocol` (protocol.rs) and Linux's
`hyperv_net.h` lay it out (`src/constants/nvsp.rs`); RNDIS as
`rndisprot.rs` and Linux's `rndis.h` (`src/constants/rndis.rs`); transfer
pages as QEMU's `vmbus_pkt_xferpg` and OpenVMM's `TransferPageHeader`
(`nonos_vmbus::packet::xfer_pages`).

## Release target

Network on a Hyper-V Generation 2 guest (and Azure's): a DHCP lease and
traffic through net.core, on the drawn address where MAC spoofing is on and
on the host-assigned one, said so, where it is off.

## Release evidence

`netvsc_proofs` runs the shipping source against a Hyper-V network host
model: `nonos_vmbus`'s host model for the GPADLs, the open and the close,
and a network VSP written from OpenVMM's netvsp (the host side Microsoft
publishes), each behaviour cited to its line; past it, a switch that drops a
spoofed source unless MAC spoofing is on, and a DHCP server. It covers the
start in netvsp's order and every version fallback with its NDIS version,
frames both ways, every receive range given back, a range past the buffer,
sections held by the host, the link indications, a broken ring, the
messages byte by byte, the DISCOVER against the egress rules, and the
station address: kept where the host carries it, refused at once, and
fallen back only after the whole bounded wait. QEMU has no network VSP, so
a boot is on Hyper-V, and the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_netvsc` keys); the
  capsule is in no release profile until then.
- `netvsc_proofs` and `nonos_vmbus` tests pass.
- A Hyper-V Generation 2 guest gets a DHCP lease through it, with MAC
  spoofing on (the drawn address) and off (the fallback, and its line).

## Explicit non-goals today

No sub-channels or receive-side scaling, no checksum or segmentation
offload, no VLAN, no SR-IOV data path switch, no interrupts.

## Verification

- Build: `make -B nonos-mk-driver-netvsc`
- Proofs: `cd userland/netvsc_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
