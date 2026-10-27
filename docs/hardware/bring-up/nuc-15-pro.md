# ASUS NUC 15 Pro

Arrow Lake-H (Core Ultra 200H), Intel I226-V or I226-LM 2.5 GbE, Intel Wi-Fi 7 BE201 or BE202 on the Arrow Lake CNVi (8086:7740), Intel HD Audio 8086:7728, M.2 NVMe, Thunderbolt 4. Parts: [ASUS's specifications][nuc15]. Build and stick: [common steps](README.md#common-to-every-plan).

## Firmware setup

- Secure Boot off; boot from USB first.
- Storage: if the setup offers RST, VMD or "RAID", note which it is, then boot once as it ships before changing it (step 5).
- VT-d on (the default); the second boot of step 7 turns it off.

## Steps

1. Lead a cable from the I226 port to a router that gives DHCP. Boot the stick. Note how long the desktop takes.
2. Before NONOS, from a Linux live stick: `lspci -nn > lspci.txt` and `lsusb > lsusb.txt` (NONOS's own device report is not built yet). Check the Wi-Fi is 8086:7740 and the NIC 8086:125c or 125b.
3. Network: `log igc net-core dhcp > igc-1.txt`. Then load any HTTPS page in the browser once on each network: `Nym mixnet`, `Anyone network`, `Direct, not anonymised`, chosen in the browser's network panel ([Privacy networks](../../using/privacy-network.md)).
4. Pull the cable for ten seconds, put it back: `log igc net-core dhcp > igc-2.txt`.
5. Storage: the installer's disk list (do not install). If firmware had VMD on: `log VMD BLOCK > vmd.txt`, then turn VMD off, boot again, and save the disk list again.
6. Wi-Fi: Settings, Wi-Fi. `log iwlwifi > wifi.txt`. Until the Bz port lands the driver refuses this card by name; the line says why.
7. Reboot with VT-d off in the firmware: `log IOMMU > iommu-off.txt`, and a photo of About's DMA row.
8. Sound: play the test sound in Settings through HDMI or the headphone jack; `log hda audio > audio.txt`.
9. USB: a stick and a keyboard in a Thunderbolt port, then through a hub: `log xhci usb > usb.txt`.

## What passes

- `igc-1.txt`: `[DRIVER-IGC] capsule spawned`; `igc: up 125c link=up` (or 125b for the LM); `igc: link up 2500 full` or the speed the router offers; `[NET-CORE] bind: interface up on driver.igc_0`; a `[DHCP] lease <address>` line. The pages in step 3 load, through Nym and through Anyone too.
- `igc-2.txt`: `igc: link down`, then `link up`, then a new lease.
- No `[EXIT] driver.igc_0` line anywhere; no line holds a MAC address.
- `vmd.txt`, with VMD on: `[VMD] 7d0b ...: RST/VMD is on; bringing up the drives behind it` and a function of class 010802; with it off, `[VMD] no Intel VMD on segment 0`. Either way the installer lists the NVMe disk, or says to switch the storage mode.
- `wifi.txt`: a line naming 8086:7740 and why it is not run (until the Bz port lands).
- `audio.txt`: `[DRIVER-HDA] capsule spawned` and the codec found; a sound heard.
- `usb.txt`: the xHCI driver bound both the PCH and the TCSS controllers; the stick lists in Files.

[nuc15]: https://www.asus.com/displays-desktops/nucs/nuc-mini-pcs/asus-nuc-15-pro/techspec/
