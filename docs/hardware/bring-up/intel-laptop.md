# Intel laptop: ThinkPad T14 Gen 5 Intel (or Gen 4 Intel)

Gen 5: Meteor Lake (Core Ultra 100U/H); Intel I219-LM or -V Ethernet; Intel AX211 on the Meteor Lake CNVi (8086:7e40) or a discrete BE200 (8086:272b); Intel HD Audio 8086:7e28 with a Realtek ALC3287 codec, array microphone on the DSP; touchpad over I2C-HID on the LPSS I2C 7e78-7e7b; one M.2 NVMe; two Thunderbolt 4 ports. Gen 4: Raptor Lake-P, AX211 on 51f1 or a Qualcomm WCN6855, LPSS I2C 51e8-51eb. Parts: Lenovo PSREF ([Gen 5][g5], [Gen 4][g4]). Build and stick: [common steps](README.md#common-to-every-plan).

## Firmware setup

- Secure Boot off; boot from USB on (F12 boot menu).
- Config, Storage: note whether the controller mode is AHCI/NVMe or RAID/RST. Boot once as it ships (step 5).
- Security, Virtualization: Intel VT-d on; step 7 turns it off.

## Steps

1. Before NONOS, from a Linux live stick: `lspci -nn > lspci.txt`, `lsusb > lsusb.txt`.
2. Boot the stick with a cable in the Ethernet port (Gen 5 has one; Gen 4 too). Desktop time, roughly.
3. Network: `log e1000e net-core dhcp > e1000e.txt`; load a page on Nym, Anyone and Direct in the browser's network panel.
4. Wi-Fi: Settings, Wi-Fi; `log iwlwifi > wifi.txt`. AX211 on 51f1 (Gen 4) should list and join; on 7e40 (Gen 5) the driver refuses by name until the Meteor Lake firmware is bundled; a BE200 until the Gl port lands.
5. Storage: the installer's disk list. With RAID/RST on: `log VMD BLOCK > vmd.txt`, then switch to AHCI/NVMe and boot again.
6. Input: type in the Terminal; move the pointer with the touchpad and the TrackPoint; click. `log i2c ps2 > input.txt`.
7. Reboot with VT-d off: `log IOMMU > iommu-off.txt`, and a photo of About's DMA row.
8. Sound: the test sound in Settings, speakers and headphones; `log hda audio > audio.txt`. The internal microphone is expected silent (no SOF driver).
9. USB: a stick in each Thunderbolt port, and through a hub or dock: `log xhci usb > usb.txt`.

## What passes

- `e1000e.txt`: `[DRIVER-E1000E] capsule spawned`, the driver's link-up line, `[NET-CORE] bind: interface up on driver.e1000e_0`, a `[DHCP] lease` line; the pages load.
- `wifi.txt`: on 51f1, a scan list, a join and a lease; on 7e40 or 272b, the refusal names the device.
- `vmd.txt`: with RAID on, `[VMD] 7d0b` (Gen 5) or `a77f` (Gen 4) `... bringing up the drives behind it`, then the NVMe function; or the installer says to switch the storage mode. Never an empty disk list without a reason.
- `input.txt`: `[DRIVER-I2C-PCI] capsule spawned` with the 7e78-7e7b (or 51e8-51eb) controller and its clock; the touchpad moves the pointer.
- `audio.txt` and a sound heard from the speakers.

[g5]: https://psref.lenovo.com/syspool/Sys/PDF/ThinkPad/ThinkPad_T14_Gen_5_Intel/ThinkPad_T14_Gen_5_Intel_Spec.pdf
[g4]: https://psref.lenovo.com/syspool/Sys/PDF/ThinkPad/ThinkPad_T14_Gen_4_Intel/ThinkPad_T14_Gen_4_Intel_Spec.pdf
