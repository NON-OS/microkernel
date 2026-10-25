# NONOS documentation: Drivers

Part of [NONOS documentation](README.md).

### Drivers

- [Drivers](drivers/README.md): how a device is found, how its driver starts and
  is confined, and every driver capsule.
- [Broker API](drivers/broker-api.md): the calls a driver makes to find, claim,
  map and release a device.
- [Writing a driver](drivers/writing-a-driver.md): a driver capsule from its
  manifest to its proof crate, built on the virtio-rng driver.
- [Platform](drivers/platform.md): processor cores, the IOMMU, the TPM, the ACPI
  buttons and GPIO.
- [Display](drivers/display.md): the UEFI GOP framebuffer, virtio-gpu, and what
  NONOS cannot drive.
- [Audio: Intel HD Audio](drivers/audio.md): the HD Audio driver, the audio
  server, the volume keys, and where it cannot play.
- [Input drivers](drivers/input/README.md): how keys, mouse movement and touches
  reach an app.
- [PS/2 keyboard and mouse](drivers/input/ps2.md): the i8042 driver for
  keyboards, mice and touchpads on the aux port.
- [I2C-HID touchpads](drivers/input/i2c-hid.md): the I2C controllers NONOS
  finds, binding the pad, and its gestures.
- [Storage drivers](drivers/storage/README.md): which disks NONOS reads and
  writes, and how a disk becomes the store and the data volume.
- [NVMe](drivers/storage/nvme.md): the NVMe driver, its limits, and how it was
  verified.
- [AHCI SATA and Intel RST](drivers/storage/ahci-and-rst.md): the SATA driver,
  RST mode, and what it refuses.
- [Intel VMD](drivers/storage/vmd.md): drives hidden behind VMD, what is
  untested, and the firmware setting to change.
- [SD cards and eMMC](drivers/storage/sd-and-emmc.md): soldered eMMC storage and
  SD card readers in this release.
- [USB mass storage](drivers/storage/usb-mass-storage.md): USB sticks and disks,
  and what is not supported.
- [USB and the xHCI host controller](drivers/usb/README.md): the xHCI driver and
  the class drivers above it.
- [USB keyboards and mice](drivers/usb/hid.md): what the USB HID driver binds
  and where its events go.
- [USB hubs](drivers/usb/hubs.md): a hub comes up, and the devices behind it are
  not reached yet.
- [Wi-Fi drivers](drivers/wifi/README.md): the two Wi-Fi drivers, a scan and a
  join from Settings to the radio, and what is refused.
- [Realtek RTL8821CE](drivers/wifi/rtl8821ce.md): the 2.4 GHz driver that scans,
  joins WPA2 and WPA3 networks and carries traffic.
- [Intel Wi-Fi (iwlwifi)](drivers/wifi/iwlwifi.md): the Intel cards it finds,
  and the few it can join on, so far only against a modelled device.
- [Wi-Fi chips with no driver](drivers/wifi/not-supported.md): how to tell
  whether your chip has a driver, and what to use instead.
- [Ethernet drivers](drivers/ethernet/README.md): every Ethernet driver, which
  are in the image, and the receive fault of the PCI drivers.
- [Intel Ethernet](drivers/ethernet/intel.md): e1000 in the image, and e1000e
  and igc written but not built.
- [Realtek Ethernet](drivers/ethernet/realtek.md): the RTL8139 and RTL8169
  family drivers.
- [USB networking](drivers/ethernet/usb-net.md): five USB network drivers, and
  why none of them runs in 0.9.2.
