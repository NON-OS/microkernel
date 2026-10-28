# Desktop board: ASRock B650M Pro RS WiFi (or ROG STRIX B760-F)

ASRock B650M Pro RS WiFi (AM5): Realtek RTL8125BG 2.5 GbE (10ec:8125), an 802.11ax module in the M.2 Key-E slot (often MT7922), HD Audio 1022:15e3 with a Realtek ALC897, three M.2 NVMe and four SATA ports on the PROM21 chipset (1022:43f6), USB on the CPU and PROM21 xHCIs. ROG STRIX B760-F (LGA1700): Intel 2.5 GbE (I226-V, unverified), ALC4080 (possibly a USB codec), RST SATA. Parts: [ASRock manual][asrock], [ROG specifications][rog]. Build and stick: [common steps](README.md#common-to-every-plan).

## Firmware setup

- Secure Boot off (or CSM off and Secure Boot in setup mode); boot from USB on.
- AMD-Vi or VT-d on; step 6 turns it off.
- SATA mode AHCI (the default on both).

## Steps

1. Before NONOS, from a Linux live stick: `lspci -nn > lspci.txt`, `lsusb > lsusb.txt`.
2. Boot with a cable in the board's port; a USB keyboard and mouse. Desktop time.
3. Network: `log rtl8169 igc net-core dhcp > net.txt`; load a page on Nym, Anyone and Direct.
4. Storage: an NVMe disk and a SATA disk attached; the installer lists both. `log nvme ahci BLOCK > disks.txt`.
5. Sound: the test sound from the rear jack; `log hda audio usb-audio > audio.txt`.
6. Reboot with the IOMMU off: `log IOMMU > iommu-off.txt`.
7. USB: a stick in a CPU port and a chipset port, and through a hub: `log xhci usb > usb.txt`.

## What passes

- `net.txt`: on the B650M, `[DRIVER-RTL8169] capsule spawned`, the RTL8125 version line, the bind line and a `[DHCP] lease`; on the B760-F, the same from `igc` (`igc: up 125c link=up`). The pages load.
- `disks.txt`: `[DRIVER-NVME]` and `[DRIVER-AHCI]` each serve a disk; both are in the installer's list.
- A sound heard from the rear jack. On the B760-F, if the ALC4080 is a USB device, `usb-audio` claims it instead.
- `usb.txt`: every xHCI the board has is bound (the CPU's 15b6/15b7/15b8 and PROM21's 43f7 on the B650M).

[asrock]: https://download.asrock.com/Manual/B650M%20Pro%20RS%20WiFi.pdf
[rog]: https://rog.asus.com/motherboards/rog-strix/rog-strix-b760-f-gaming-wifi-model/spec/
