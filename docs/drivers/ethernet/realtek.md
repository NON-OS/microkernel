# Realtek Ethernet

NONOS has two Realtek PCI Ethernet driver [capsules](../../overview/glossary.md#capsule) in the image: one for the RTL8139 and one for the RTL8169 family, which covers the RTL8168, RTL8111, RTL810x and the 2.5 GbE RTL8125.

## Which driver takes which chip

| Driver | Chips | PCI ids, vendor 10ec | State |
|---|---|---|---|
| `driver.rtl8139_0` | RTL8139 | 8139 | Partial: in the image, host tests pass, [receive fault](README.md#the-receive-fault), no hardware report, no QEMU run target |
| `driver.rtl8169_0` | RTL8169, RTL8110, RTL8168, RTL8111, RTL810x, RTL8125 | 8169, 8167, 8168, 8161, 8162, 2502, 2600, 8136, 8125, 3000 | Partial: in the image, [receive fault](README.md#the-receive-fault), no hardware report; its flake check fails on a lint |
| none | RTL8126A, RTL8127A | 8126, 8127 | Not supported: left out of the id table |

The USB Realtek RTL8153 is a separate capsule, not in the image; see [USB networking](usb-net.md).
