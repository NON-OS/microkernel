# What each USB network capsule binds

The three USB network [capsules](../../overview/glossary-boot-root-record.md#capsule) each take one kind of device; none runs in 0.9.2, for the reasons on the [USB networking](usb-net.md) page. Each capsule's own `BINDING.md` gives its steps in full.

| Capsule | Service and port | Binds |
|---|---|---|
| `capsule_driver_cdc_ecm` | `driver.cdc_ecm0`, 4250 | a communications interface of subclass 0x06 with a Union and an Ethernet Networking descriptor |
| `capsule_driver_cdc_ncm` | `driver.cdc_ncm0`, 4252 | a communications interface of subclass 0x0D, protocol 0, with the NCM descriptor |
| `capsule_driver_rndis` | `driver.rndis0`, 4254 | three RNDIS control classes, listed below |

- CDC-ECM: `find_ecm` takes the first communications interface of subclass 0x06 at alternate 0 whose Union it masters and names a CDC Data interface, with a 13-byte Ethernet Networking descriptor and a data alternate 1 holding exactly one bulk IN and one bulk OUT (`userland/capsule_driver_cdc_ecm/src/ecm/function.rs`, `find_ecm`; `union.rs`, `data.rs`). Before configuring it holds every interface of the configuration with the host, and stops if another driver holds one (`claim.rs`). With the RTL8153 capsule quarantined, it binds 0bda:8153 and 17ef:721e in their ECM configuration like any other device. One Ethernet frame goes in each bulk transfer, with one padding byte when its length is a multiple of 64 or of the packet size (`userland/capsule_driver_cdc_ecm/src/ecm/frame.rs`, `padded_len`).
- CDC-NCM: `find_ncm` needs subclass 0x0D, protocol 0, and both the Ethernet Networking and the NCM functional descriptors (`userland/capsule_driver_cdc_ncm/src/ncm/function.rs`, `find_ncm`).
- RNDIS: the control interface must be CDC ACM with the vendor protocol (0x02, 0x02, 0xFF), the wireless controller class phones use for tethering (0xE0, 0x01, 0x03), or the miscellaneous RNDIS class (0xEF, 0x04, 0x01) (`userland/capsule_driver_rndis/src/rndis/function.rs`, `CONTROL_CLASSES`). A communications function that declares modem capabilities is a modem and is skipped (`userland/capsule_driver_rndis/src/rndis/function.rs`, `acm_capable`).
- ASIX AX88179 and Realtek RTL8153: quarantined (branches `quarantine/ax88179` and `quarantine/rtl8153`). Their register maps, values and bring-up were taken from another driver, and no public programming reference exists to rewrite them from (Decision 10).

## See also

- [USB networking](usb-net.md)
- [Ethernet drivers](README.md)
- [Realtek Ethernet](realtek.md)
- [Wi-Fi chips with no driver](../wifi/not-supported.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
