# What each USB network capsule would bind

The five USB network [capsules](../../overview/glossary.md#capsule) each take one kind of device; none runs in 0.9.2, for the reasons on the [USB networking](usb-net.md) page. Each capsule's own `BINDING.md` gives its steps in full.

| Capsule | Service and port | Binds |
|---|---|---|
| `capsule_driver_cdc_ecm` | `driver.cdc_ecm0`, 4250 | a communications interface of subclass 0x06 with a Union and an Ethernet Networking descriptor |
| `capsule_driver_cdc_ncm` | `driver.cdc_ncm0`, 4252 | a communications interface of subclass 0x0D, protocol 0, with the NCM descriptor |
| `capsule_driver_rndis` | `driver.rndis0`, 4254 | three RNDIS control classes, listed below |
| `capsule_driver_ax88179` | `driver.ax88179_0`, 4256 | 13 USB vendor:product pairs |
| `capsule_driver_rtl8153` | `driver.rtl8153_0`, 4258 | 19 USB vendor:product pairs |

- CDC-ECM: `find_ecm` takes the first communications interface of subclass 0x06 that has a Union descriptor, an Ethernet Networking descriptor and a data interface with a bulk pair (`userland/capsule_driver_cdc_ecm/src/ecm/function.rs:26-51`, `find_ecm`). It leaves 0bda:8153 and 17ef:721e to the RTL8153 capsule, as Linux does (`userland/capsule_driver_cdc_ecm/src/ecm/vendor.rs:22`, `LEFT_TO_RTL8153`). One Ethernet frame goes in each bulk transfer, with one padding byte when its length is a multiple of 64 or of the packet size (`userland/capsule_driver_cdc_ecm/src/ecm/frame.rs:22-36`, `padded_len`).
- CDC-NCM: `find_ncm` needs subclass 0x0D, protocol 0, and both the Ethernet Networking and the NCM functional descriptors (`userland/capsule_driver_cdc_ncm/src/ncm/function.rs:26-62`, `find_ncm`).
- RNDIS: the control interface must be CDC ACM with the vendor protocol (0x02, 0x02, 0xFF), the wireless controller class phones use for tethering (0xE0, 0x01, 0x03), or the miscellaneous RNDIS class (0xEF, 0x04, 0x01) (`userland/capsule_driver_rndis/src/rndis/function.rs:27-31`, `CONTROL_CLASSES`). A communications function that declares modem capabilities is a modem and is skipped (`userland/capsule_driver_rndis/src/rndis/function.rs:57-64`, `acm_capable`).
- ASIX AX88179: the ids Linux's ax88179_178a driver binds, with 0b95:1790 for the AX88179 and 0b95:178a for the AX88178A; the other 11 pairs are adapters built on these chips (`userland/capsule_driver_ax88179/src/ax/products.rs:17-35`, `PRODUCTS`).
- Realtek RTL8153: 0bda:8153 for the RTL8153 and RTL8153B, and 18 pairs for adapters and docks built on them. For those, the chip's version register decides, and an adapter built on another chip is refused by name (`userland/capsule_driver_rtl8153/src/r8153/ids.rs:17-51`, `RTL8153_FAMILY`). The RTL8152, the RTL8156 family and two ids whose chip Linux does not name are left out.

## See also

- [USB networking](usb-net.md)
- [Ethernet drivers](README.md)
- [Realtek Ethernet](realtek.md)
