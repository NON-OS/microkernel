# AMD laptop: ThinkPad T14 Gen 4 AMD

Ryzen 7040 (Phoenix); Realtek RTL8111EPV Ethernet (10ec:8168); MediaTek RZ616 (MT7922, 14c3:0616) or Qualcomm WCN6855 Wi-Fi; HD Audio 1022:15e3 with a Realtek ALC3287 codec, the microphone on the ACP (1022:15e2); touchpad over I2C-HID on the AMD DesignWare I2C (ACPI AMDI0010); one M.2 NVMe; USB4. Parts: [Lenovo PSREF][g4a]. Build and stick: [common steps](README.md#common-to-every-plan).

## Firmware setup

- Secure Boot off; boot from USB on.
- Security, Virtualization: AMD-Vi (IOMMU) on; step 7 turns it off.

## Steps

1. Before NONOS, from a Linux live stick: `lspci -nn > lspci.txt`, `lsusb > lsusb.txt`.
2. Boot the stick with a cable in the Ethernet port. Desktop time, roughly.
3. Network: `log rtl8169 net-core dhcp > rtl.txt`; load a page on Nym, Anyone and Direct. This is the first real RTL8111 run: it proves or disproves [the receive fault](../../drivers/ethernet/README.md#the-receive-fault) fix on silicon.
4. Wi-Fi: Settings, Wi-Fi; `log wifi > wifi.txt`. No MediaTek or Qualcomm driver exists: expect no Wi-Fi; the file records what the boot said about it.
5. Storage: the installer's disk list.
6. Input: type; move the pointer with the touchpad and TrackPoint; `log i2c ps2 > input.txt`.
7. Reboot with AMD-Vi off: `log IOMMU > iommu-off.txt`, and About's DMA row.
8. Sound: the test sound, speakers and headphones; `log hda audio > audio.txt`. The microphone is expected silent (no ACP driver).
9. USB: a stick in the USB4 port and through a hub: `log xhci usb > usb.txt`.

## What passes

- `rtl.txt`: `[DRIVER-RTL8169] capsule spawned`, the chip version line, `[NET-CORE] bind: interface up on driver.rtl8169_0`, a `[DHCP] lease` line; the pages load. No `[NET-CORE] rx batch unreadable` line.
- `input.txt`: `[DRIVER-I2C-PCI] capsule spawned` and its line naming the ACPI controller it bound; the touchpad moves the pointer.
- `audio.txt` and a sound heard.

[g4a]: https://psref.lenovo.com/syspool/Sys/PDF/ThinkPad/ThinkPad_T14_Gen_4_AMD/ThinkPad_T14_Gen_4_AMD_Spec.pdf
