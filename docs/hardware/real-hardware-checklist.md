# The real-hardware checklist

One copy per machine and boot. Part of [the real-hardware kit](real-hardware-kit.md).

## Before booting

- [ ] Machine number from the kit, CPU family and memory size (no brand serials).
- [ ] Firmware settings: VT-d or AMD-Vi on or off; Secure Boot on or off; SATA mode
      (AHCI, RAID/RST or VMD); CSM or legacy boot off.
- [ ] The image's commit and profile (`standard-dev` or `hardened-dev`).
- [ ] If the machine also runs Linux: `lspci -nn` and `lsusb` saved to a file.

## The boot

- [ ] The boot menu appears. Photo.
- [ ] The splash: how far it gets, whether it says devices are unconfined. Photo.
- [ ] The desktop appears, and how long after the boot menu, roughly.
- [ ] Keyboard types; the touchpad or mouse moves the pointer; buttons click.
- [ ] Screen at its native resolution; brightness keys change it (laptops).
- [ ] Sound from the speakers and the headphone jack; volume keys work.
- [ ] Wired network: a cable plugged in gets an address (Settings, Network).
- [ ] Wi-Fi: networks are listed, one joins, a page loads in the browser.
- [ ] Storage: the installer lists the internal disk (do not install unless that is the test).
- [ ] USB: a stick and a keyboard work in a port on the machine and in a hub.
- [ ] Battery level and charging state shown; the lid closes the screen (laptops).
- [ ] About: the DMA row, and what it says. Photo.

## Afterwards

- [ ] In the Terminal: `log > boot-log.txt`, then `log driver- exit > drivers.txt`.
- [ ] The hardware report, when the image has `nonos-hwreport`: `hwreport` writes it to the
      stick; copy that file.
- [ ] Anything that failed, in a sentence each: what you did, what you expected, what happened.
