# Ethernet drivers

NONOS carries wired traffic through one driver [capsule](../../overview/glossary.md#capsule) per card family; this page lists every Ethernet driver in the tree, says which ones are in the image, and describes the receive fault that stopped the three PCI drivers in the image and what `net.core` now does in its place. [How a frame travels](frame-path.md) and [which image carries which driver, with the tests](images-and-tests.md) have their own pages.

## Drivers at a glance

The states are the [support matrix](../../hardware/MATRIX.md#how-to-read-it)'s:

- Works: the image carries the whole path for the card.
- Partial: part of the path is there; the row says what is missing.
- Not supported: no driver in the image.

| Family | PCI or USB ids | Capsule | In the image | State | Page |
|---|---|---|---|---|---|
| Intel 8254x (e1000) | 28 ids, vendor 8086 | `driver.e1000_0` | yes | Partial: no run shows a frame received, no QEMU run target | [intel.md](intel.md) |
| Intel 82574, 82583, I217, I218, I219 (e1000e) | 64 ids, vendor 8086 | `driver.e1000e_0` | no | Not supported: capsule not built into any image | [intel-e1000e.md](intel-e1000e.md) |
| Intel I225, I226 (igc) | 16 ids, vendor 8086 | `driver.igc_0` | no | Not supported: capsule not built into any image | [intel-igc.md](intel-igc.md) |
| Realtek RTL8139 | 10ec:8139 | `driver.rtl8139_0` | yes | Partial: no run shows a frame received, no QEMU run target | [realtek.md](realtek.md) |
| Realtek RTL8169, RTL8168, RTL8111, RTL810x, RTL8125 | 10 ids, vendor 10ec | `driver.rtl8169_0` | yes | Partial: no run shows a frame received, flake check fails on a lint | [realtek-rtl8169.md](realtek-rtl8169.md) |
| Realtek RTL8126A, RTL8127A | 10ec:8126, 10ec:8127 | none | no | Not supported: left out of the id table | [realtek-rtl8169.md](realtek-rtl8169.md) |
| virtio network device | 1af4:1000, 1af4:1041 | `driver.virtio_net0` | yes | Works: the QEMU run targets attach it | this page |
| USB CDC-ECM, CDC-NCM, RNDIS, ASIX AX88179, Realtek RTL8153 | USB class or USB ids | five capsules | no | Not supported: not built, and the USB host driver lacks the transfer they need | [usb-net.md](usb-net.md) |

No Ethernet driver has a hardware report in 0.9.2. Read from the code on this commit, `net.core` takes received frames from the e1000, RTL8139 and RTL8169 drivers with operation 5, past [the receive fault](#the-receive-fault), but only virtio-net, in a virtual machine, has a run that shows a frame received. For a machine that needs a network now, read [what to use instead](../wifi/not-supported.md#what-to-use-instead).

## The receive fault

The e1000, e1000e, igc, RTL8139 and RTL8169 drivers number operation 6 for a register snapshot and answer it with status 0 and twelve register words (`userland/capsule_driver_e1000/src/server/handlers/stats.rs:25-33`, `write_status`). `net.core` sends operation 6 with no body as its receive-batch probe on its first poll of a newly bound card. Read as a batch, a snapshot's frame count never matches its body, so each one is dropped with `[NET-CORE] rx batch unreadable, dropped` (`userland/capsule_net_core/src/device/rx_batch.rs:63-68`, `batch_frames`): a card read that way receives nothing, and DHCP gets no lease through it. That was the receive fault, in the code until this commit.

`net.core` now takes only an empty batch, a status 0 reply whose body is a zero count word, as the probe's yes (`userland/capsule_net_core/src/device/batch_served.rs:29-31`, `served`). A register snapshot reads as no, and the card is read one frame at a time with operation 5, as the Wi-Fi drivers are (`userland/capsule_net_core/src/device/receive.rs:22-32`, `receive`). The virtio driver serves real batches (`userland/capsule_driver_virtio_net/src/protocol/ops.rs:25`, `OP_RX_BATCH`). The USB core keeps its counters at 0x0100 and answers operation 6 with `E_INVAL` (`userland/nonos_usbnet/src/nnet/wire.rs:36-39`, `OP_STATS`). No run on this commit shows a frame received through the PCI drivers either way.

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
