# Ethernet drivers

NONOS carries wired traffic through one driver [capsule](../../overview/glossary.md#capsule) per card family; this page lists every Ethernet driver in the tree, says which ones are in the image, shows how a frame reaches `net.core`, and describes the receive fault the three PCI drivers carry in 0.9.2.

## Drivers at a glance

The states mean:

- Partial: in the image, with host tests, and no hardware report in this release. The row says what else is missing.
- Not supported: no driver in the image.

| Family | PCI or USB ids | Capsule | In the image | State | Page |
|---|---|---|---|---|---|
| Intel 8254x (e1000) | 28 ids, vendor 8086 | `driver.e1000_0` | yes | Partial: [receive fault](#the-receive-fault), no QEMU run target | [intel.md](intel.md) |
| Intel 82574, 82583, I217, I218, I219 (e1000e) | 64 ids, vendor 8086 | `driver.e1000e_0` | no | Not supported: capsule not built into any image | [intel.md](intel.md) |
| Intel I225, I226 (igc) | 16 ids, vendor 8086 | `driver.igc_0` | no | Not supported: capsule not built into any image | [intel.md](intel.md) |
| Realtek RTL8139 | 10ec:8139 | `driver.rtl8139_0` | yes | Partial: [receive fault](#the-receive-fault), no QEMU run target | [realtek.md](realtek.md) |
| Realtek RTL8169, RTL8168, RTL8111, RTL810x, RTL8125 | 10 ids, vendor 10ec | `driver.rtl8169_0` | yes | Partial: [receive fault](#the-receive-fault), flake check fails on a lint | [realtek.md](realtek.md) |
| Realtek RTL8126A, RTL8127A | 10ec:8126, 10ec:8127 | none | no | Not supported: left out of the id table | [realtek.md](realtek.md) |
| virtio network device | 1af4:1000, 1af4:1041 | `driver.virtio_net0` | yes | Partial: the QEMU run targets attach it | this page |
| USB CDC-ECM, CDC-NCM, RNDIS, ASIX AX88179, Realtek RTL8153 | USB class or USB ids | five capsules | no | Not supported: not built, and the USB host driver lacks the transfer they need | [usb-net.md](usb-net.md) |

No Ethernet driver has a hardware report in 0.9.2.
