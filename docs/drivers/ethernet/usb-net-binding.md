# What each USB network capsule binds

The four USB network [capsules](../../overview/glossary-boot-root-record.md#capsule) each take one kind of device; none runs in 0.9.2, for the reasons on the [USB networking](usb-net.md) page. Each capsule's own `BINDING.md` gives its steps in full.

| Capsule | Service and port | Binds |
|---|---|---|
| `capsule_driver_cdc_ecm` | `driver.cdc_ecm0`, 4250 | a communications interface of subclass 0x06 with a Union and an Ethernet Networking descriptor |
| `capsule_driver_cdc_ncm` | `driver.cdc_ncm0`, 4252 | a communications interface of subclass 0x0D, protocol 0, with the NCM descriptor |
| `capsule_driver_rndis` | `driver.rndis0`, 4254 | three RNDIS control classes, listed below |
| `capsule_driver_rtl8153` | `driver.rtl8153_0`, 4258 | sixteen RTL8153-family ids in their vendor configuration, an RTL8153 or RTL8153B chip inside |

- CDC-ECM: `find_ecm` takes the first communications interface of subclass 0x06 at alternate 0 whose Union it masters and names a CDC Data interface, with a 13-byte Ethernet Networking descriptor and a data alternate 1 holding exactly one bulk IN and one bulk OUT (`userland/capsule_driver_cdc_ecm/src/ecm/function.rs`, `find_ecm`; `union.rs`, `data.rs`). Before configuring it holds every interface of the configuration with the host, and stops if another driver holds one (`claim.rs`). It binds an RTL8153 adapter's ECM configuration like any other device's, when it reaches the port before the RTL8153 capsule does; either then serves the adapter. One Ethernet frame goes in each bulk transfer, with one padding byte when its length is a multiple of 64 or of the packet size (`userland/capsule_driver_cdc_ecm/src/ecm/frame.rs`, `padded_len`).
- CDC-NCM: `find_ncm` needs subclass 0x0D, protocol 0, and both the Ethernet Networking and the NCM functional descriptors (`userland/capsule_driver_cdc_ncm/src/ncm/function.rs`, `find_ncm`).
- RNDIS: the control interface must be CDC ACM with the vendor protocol (0x02, 0x02, 0xFF), the wireless controller class phones use for tethering (0xE0, 0x01, 0x03), or the miscellaneous RNDIS class (0xEF, 0x04, 0x01) (`userland/capsule_driver_rndis/src/rndis/function.rs`, `CONTROL_CLASSES`). A communications function that declares modem capabilities is a modem and is skipped (`userland/capsule_driver_rndis/src/rndis/function.rs`, `acm_capable`).
- RTL8153: the device's ids must be one of sixteen in Linux v6.12's r8152 table (`userland/capsule_driver_rtl8153/src/rtl/adapters.rs`, `ADAPTERS`), its first configuration whose first interface is of the vendor class with a bulk pair is taken (`config.rs`, `vendor_config`), and the chip's version register must name an RTL8153 or RTL8153B, else it is refused by name before configuring (`model.rs`, `identify`). The bring-up is tables of register steps over vendor control transfers, every wait bounded (`wake/`, `start.rs`, `bounds.rs`). Each bulk IN is walked by its 24-byte descriptors, each length bounded against the bytes that came (`rx.rs`, `next`); each frame out goes behind an 8-byte descriptor (`tx.rs`, `encode`). Full steps: `userland/capsule_driver_rtl8153/BINDING.md`.
- ASIX AX88179: quarantined (branch `quarantine/ax88179`). Its register map, values and bring-up were taken from another driver, and no public programming reference exists to rewrite them from (Decision 10). The earlier RTL8153 capsule was quarantined for the same reason (branch `quarantine/rtl8153`); the one in the tree now was written anew, reading Linux v6.12 for behaviour only.

## See also

- [USB networking](usb-net.md)
- [Ethernet drivers](README.md)
- [Realtek Ethernet](realtek.md)
- [Wi-Fi chips with no driver](../wifi/not-supported.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
