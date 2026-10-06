# SD cards and eMMC

What NONOS does with soldered eMMC storage and with SD card readers in this release.

## In short

| Hardware | Matched by | Driver | State in 0.9.2 |
|---|---|---|---|
| eMMC on an Intel SD host controller | PCI class 08h, subclass 05h, 13 Intel device ids | `driver.ahci0` | Served when no SATA disk comes up |
| eMMC on another SD host controller | PCI class 08h, subclass 05h, slot type embedded | `driver.ahci0` | Served the same way, but only when the capsule was started for another controller |
| SD card or SDIO slot on an Intel SD host controller | 14 Intel device ids | none | Refused by id |
| SD card slot on another SD host controller | slot type not embedded | none | Skipped |
| Realtek PCIe card reader RTS5227 or RTS522A | 10ec:5227, 10ec:522a | `driver.rtsx0` | Written, not in the image |
| Other Realtek PCIe card readers | 12 more Realtek ids | none | Not supported |
| USB card reader | USB mass storage, Bulk-Only | `driver.usb_msc0` | Served |

So an SD card in a built-in PCIe or SDHCI slot cannot be read in this release. An SD card in a USB card reader is served by the USB mass-storage driver.
