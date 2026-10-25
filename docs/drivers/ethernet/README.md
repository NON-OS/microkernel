# Ethernet drivers

NONOS carries wired traffic through one driver [capsule](../../overview/glossary-boot-root-record.md#capsule) per card family; this page lists every Ethernet driver in the tree, says which ones are in the image, and describes the receive fault that stopped the three PCI drivers in the image and what `net.core` now does in its place. [How a frame travels](frame-path.md) and [which image carries which driver, with the tests](images-and-tests.md) have their own pages.

## Drivers at a glance

The states are the [support matrix](../../hardware/MATRIX.md#how-to-read-it)'s:

- Works: the image carries the whole path for the card.
- Partial: part of the path is there; the row says what is missing.
- Not supported: no driver in the image.

| Family | PCI or USB ids | Capsule | In the image | State | Page |
|---|---|---|---|---|---|
| Intel 8254x (e1000) | 28 ids, vendor 8086 | `driver.e1000_0` | yes | Partial: no run shows a frame received, receive batches not yet booted, no QEMU run target | [intel.md](intel.md) |
| Intel 82574, 82583, I217, I218, I219 (e1000e) | 64 ids, vendor 8086 | `driver.e1000e_0` | no | Not supported: capsule not built into any image | [intel-e1000e.md](intel-e1000e.md) |
| Intel I225, I226 (igc) | 16 ids, vendor 8086 | `driver.igc_0` | no | Not supported: wired in the kernel, not yet signed, so in no image | [intel-igc.md](intel-igc.md) |
| Realtek RTL8139 | 10ec:8139 | `driver.rtl8139_0` | yes | Partial: no run shows a frame received, receive batches not yet booted, no QEMU run target | [realtek.md](realtek.md) |
| Realtek RTL8169, RTL8168, RTL8111, RTL810x, RTL8125 | 10 ids, vendor 10ec | `driver.rtl8169_0` | yes | Partial: no run shows a frame received, receive batches not yet booted, flake check fails on a lint | [realtek-rtl8169.md](realtek-rtl8169.md) |
| Realtek RTL8126A, RTL8127A | 10ec:8126, 10ec:8127 | none | no | Not supported: left out of the id table | [realtek-rtl8169.md](realtek-rtl8169.md) |
| virtio network device | 1af4:1000, 1af4:1041 | `driver.virtio_net0` | yes | Works: the QEMU run targets attach it | this page |
| USB CDC-ECM, CDC-NCM, RNDIS | USB class | three capsules | no | Not supported: built, embedded and spawned where a USB host is, but no owner certificate yet | [usb-net.md](usb-net.md) |

No Ethernet driver has a hardware report in 0.9.2. Read from the code on this commit, `net.core` takes received frames from the e1000, RTL8139 and RTL8169 drivers in batches with operation 6, past [the receive fault](#the-receive-fault), but only virtio-net, in a virtual machine, has a run that shows a frame received. For a machine that needs a network now, read [what to use instead](../wifi/not-supported.md#what-to-use-instead).

## The receive fault

In 0.9.2 the e1000, e1000e, igc, RTL8139 and RTL8169 drivers numbered operation 6 for a register snapshot and answered it with status 0 and twelve register words. `net.core` sends operation 6 with no body as its receive-batch probe on its first poll of a newly bound card, and took any status 0 reply as a yes. Read as a batch, a snapshot's frame count never matches its body, so each one was dropped with `[NET-CORE] rx batch unreadable, dropped` (`userland/capsule_net_core/src/device/rx_batch.rs:63-68`, `batch_frames`): a card read that way received nothing, and DHCP got no lease through it.

Fixed in code on this commit, from both sides; not yet booted:

- `net.core` takes only an empty batch, a status 0 reply whose body is a zero count word, as the probe's yes (`userland/capsule_net_core/src/device/batch_served.rs:29-31`, `served`). Any other answer leaves the card on operation 5, one frame per call, as the Wi-Fi drivers are read (`userland/capsule_net_core/src/device/receive.rs:22-32`, `receive`).
- The snapshot moved to operation 7 in the five PCI capsules and in the kernel's RTL8139 and RTL8169 clients (`userland/capsule_driver_e1000/src/protocol/ops.rs:32`, `OP_STATS`). The USB core keeps its counters at 7 too and answers operation 6 with `E_INVAL` (`userland/nonos_usbnet/src/nnet/wire.rs:41`, `OP_STATS`).
- e1000, RTL8139 and RTL8169 serve operation 6 as virtio-net does: an empty request answers an empty batch, and a numbered batch is kept until the caller asks for the next number, so a reply that comes late is not lost (`userland/capsule_driver_e1000/src/server/handlers/rx_batch.rs:41`, `handle`). e1000 takes every completed descriptor in one pass of the ring (`userland/capsule_driver_e1000/src/queue/rx_batch.rs:38`, `take_batch`); the Realtek drivers call their receive routine up to the batch limit (`userland/capsule_driver_rtl8139/src/rx/batch.rs:40`, `gather`). A receive error after some frames ends the batch and is answered with `E_IO` on the next number.

No run on this commit shows a frame received through the PCI drivers either way.

What a boot must show to say a card receives: the driver's own link line (`igb: link up 1000 full`, for one), then `net.dhcp.client`'s one line for the lease it bound, `[DHCP] lease 10.0.2.15/24 gw 10.0.2.2 from 10.0.2.2` over QEMU's user net (`userland/capsule_net_dhcp/src/dora/lease_line.rs`, `lease_line`). A lease binds only once the server's offer and its ack came in through the card, so the line is the receive path seen end to end. Each card is booted alone, with no virtio-net beside it, through `nix run .#qemu -- --lane nic=igb` (or `nic=e1000`, `nic=e1000e`, `nic=rtl8139`; QEMU has no RTL8169 model), plain and with `--lane iommu=intel`.

## Behaviour every PCI driver shares

- The kernel starts `driver_e1000`, `driver_rtl8139` and `driver_rtl8169` on every boot of an image that carries them (`src/userspace/init/spawn_plan/drivers_nic.rs:17-21`, `spawn`). A driver whose chip is absent exits at once with `EXIT_ABSENT` (2) and claims nothing (`userland/libc/src/bringup/policy.rs:37-41`, `EXIT_ABSENT`).
- A chip that is present but fails bring-up is tried 7 times, with a sleep starting at 100 ms and doubling up to 3.2 s, then the driver exits with `EXIT_GAVE_UP` (6) (`userland/libc/src/bringup/policy.rs:30-41`, `BRINGUP_ATTEMPTS`, `EXIT_GAVE_UP`).
- No PCI Ethernet driver binds an interrupt: they all poll, and none of their masks holds Irq (`userland/capsule_driver_e1000/src/setup/sequence.rs:24-27`, `IMS`).
- Each draws a new locally administered station address every boot instead of the address burned into the card, and fails closed without randomness (`userland/capsule_driver_e1000/src/init/station_address.rs:22-31`, `draw`).
- When the bound link goes down and comes back, `net.core` asks DHCP for the lease again (`userland/capsule_net_core/src/iface/relink.rs:40-47`, `Change::Returned`).

The USB drivers draw theirs too, once a run before the first bind, and hand the same address to a device bound again after a loss; see [USB networking](usb-net.md).

## virtio-net in a virtual machine

The virtio driver takes vendor 0x1AF4 with the transitional device 0x1000 or the modern device 0x1041 (`userland/capsule_driver_virtio_net/src/constants/pci.rs:22-24`, `VIRTIO_NET_MODERN`). A transitional device with its legacy I/O BAR is driven over port I/O; a modern-only one over the virtio 1.0 structures in a memory BAR (`userland/capsule_driver_virtio_net/src/transport/probe.rs:25-31`, `probe`). Its mask is 0x1B8038, which holds both Mmio and Pio, with Debug optional (`userland/capsule_driver_virtio_net/Capsule.mk:24-27`, `CAPSULE_REQUIRED_CAPS`). It draws its station address like the others and never takes the device's (`userland/capsule_driver_virtio_net/src/setup/station.rs:23-27`, `draw`).

The kernel starts it only when the boot PCI scan found a virtio network device (`src/userspace/init/spawn_plan/drivers_virtio_display.rs:35-41`, `spawn_net`). The QEMU run targets attach `virtio-net-pci` with user-mode networking, `nat` unless `QEMU_NET_MODE` says `off` or `hostfwd` (`mk/10-qemu.mk:124-136`, `QEMU_NET`). `nonos-mk-run` builds the full image and boots it that way (`mk/40-run.mk:107-118`, `QEMU_NET`):

```sh
make nonos-mk-run QEMU_NET_MODE=nat
```

Not tested in this release.

The flake check `proofs-virtio_net_proofs` passed with 20 tests on this commit.

## See also

- [How a frame travels](frame-path.md)
- [Images and tests](images-and-tests.md)
- [Intel Ethernet](intel.md), [e1000e](intel-e1000e.md) and [igc](intel-igc.md)
- [Realtek Ethernet](realtek.md) and [the RTL8169 family](realtek-rtl8169.md)
- [USB networking](usb-net.md)
- [Wi-Fi drivers](../wifi/README.md)
- [The driver model](../README.md)
- [The broker API](../broker-api.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
- [Wi-Fi and networking for users](../../using/wifi-and-networking.md)
