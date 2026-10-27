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

## Intel BE200 and BE201

The iwlwifi driver's BZ-family path (BE200 8086:272b; BE201 8086:a840, 7740,
4d40) is host-proven only: it has never booted on either card. A first boot
is its test. The `[iwlwifi]` lines print on a development image (`Debug`
granted to the capsule); on another image read the same facts from the
Settings Wi-Fi panel's status (stage, step, detail).

- [ ] What to boot: a `standard-dev` image at or after the BZ-family commit,
      on a machine with a BE200 card or a Lunar Lake or Arrow Lake laptop
      with a BE201. Wi-Fi on in the firmware settings; the RF-kill switch on.
- [ ] Before: if the machine runs Linux, save `lspci -nn -d 8086:` and
      `dmesg | grep -i iwlwifi` (the `Detected crf-id ..., cnv-id ...` and
      `loaded firmware version` lines).
- [ ] Identity: `[iwlwifi] pci 0x0000272b` (or `a840`, `7740`, `4d40`), then
      `[iwlwifi] firmware iwlwifi-gl-c0-fm-c0-92.ucode` (BE200) or
      `iwlwifi-bz-b0-fm-c0-92.ucode` (BE201). A `radio not up: ... (step 4,
      detail 0x...)` line instead names the refusal: 0x5000s a MAC step,
      0x7000s a radio step, 0x2xxxx a MAC type, 0x3xxxx a radio type.
- [ ] ALIVE: `[iwlwifi] firmware alive: status 0x0000cafe umac M.N sku 0x...`.
      If instead `radio not up: the firmware never raised ALIVE (step 7,
      detail 0x00000030)`, keep the `secure boot status: cpu1 ... cpu2 ...`
      line before it. Step 3 (`the MAC clock never came up`) means the V2
      wake bits were not answered.
- [ ] PNVM: `platform NVM section 0x... given` or `no platform NVM section
      for this SKU: firmware defaults`.
- [ ] NVM: `[iwlwifi] up: N channels, tx ant A, regulatory 0x....`, then
      `[iwlwifi] radio up; passive scanning`. Neither `joining off: the
      firmware runs another layout of command ...` nor `no randomness for a
      station address` should follow.
- [ ] Scan: `[iwlwifi] first scan complete: N networks from M beacons`
      within about ten seconds, and the Settings panel lists the networks in
      range.
- [ ] Join (WPA2, then WPA3 if an access point offers it): `[iwlwifi] join:
      port open; sent 4 recv N eapol 2 state 4 refused 0` (WPA3: `sent 5`);
      the panel reads Joined.
- [ ] DHCP: `[NET-CORE] bind: interface up on driver.iwlwifi0`, then a
      lease (`[NET-CORE] lease-status state=...`); a page loads in the
      browser.
- [ ] Send back: the boot log (`log > boot-log.txt`), `log driver- exit >
      drivers.txt`, the Linux lines above if any, which card and laptop
      model, and the first `[iwlwifi]` line that differs from this list.
      The logs never carry a MAC address, SSID or BSSID from this driver.

## MediaTek MT7921 and MT7922

The mt7921e driver (14c3:7961 MT7921E/RZ608, 14c3:0616 and 14c3:7922
MT7922/RZ616, 14c3:0608, 14c3:7920, 0b48:7922) is host-proven only: it has
never run on a card. A first boot is its test. The `[mt7921]` lines print on
a development image (`Debug` granted to the capsule); on another image the
Settings Wi-Fi panel shows the stage. A ThinkPad T14 Gen 4 AMD carries the
MT7922.

- [ ] What to boot: a `standard-dev` image at or after the mt7921e wiring
      commit, written with `make nonos-mk-usb-img` so the firmware package is
      on the stick: `EFI/nonos/firmware/mediatek/` holds six `WIFI_*` files
      and `LICENCE.mediatek` (check with `mdir` or by mounting the stick).
      Wi-Fi on in the firmware settings.
- [ ] Before: if the machine runs Linux, save `lspci -nn -d 14c3:` and
      `dmesg | grep -i mt79` (the `HW/SW Version` and `WM Firmware Version`
      lines).
- [ ] Power and identity: `[mt7921] stage power: driver owns the card`, then
      `[mt7921] chip 0x00007961 rev 0x000000NN` (or `0x00007922`, or
      `0x00007920`). `stopped at stage 2` or `stage 3` here: keep the line.
- [ ] Package: `[mt7921] package: WIFI_MT7922_patch_mcu_1_1_hdr.bin verified`
      and `... WIFI_RAM_CODE_MT7922_1.bin verified` (the MT7961 or 1a names
      on the other chips). `missing` is stage 10: the stick has no package;
      `does not match its sha256` is stage 11: the files are another
      release's. Neither touches the card past its ownership.
- [ ] Rings and firmware: `[mt7921] stage rings: WFDMA on, six rings
      armed`, `[mt7921] stage firmware: patch and RAM code running`,
      `[mt7921] stage mac: up, bands 1,1`. On a failure, `firmware failed at
      step N` (1 a bad patch, 2 the semaphore, 3 a piece's announcement,
      4 its bytes, 5 the patch finish, 6 a bad RAM image, 7 the start,
      8 never ready, 9 the MCU's ownership) or `mac init failed at step N` (1 capabilities to 5 RX
      path), then `attempt N failed; resetting`; after three, `card quiet:
      WFDMA off, bus master off` and `stopped at stage 4`.
- [ ] Interface: `[mt7921] stage interface: station added`, `[mt7921] stage
      ready: scanning`, `[mt7921] serving driver.mt7921e0`. `stopped at
      stage 7` means the kernel gave no randomness.
- [ ] Scan: within about ten seconds the Settings panel lists the networks
      in range, 2.4 and 5 GHz. If it lists none, the status reply's
      diagnostic words (chip, channel grants, grants missed, bad frames,
      refused commands) say whether the firmware ever granted a channel;
      keep the log.
- [ ] Join (WPA2, then WPA3 if an access point offers it): the panel reads
      Joined. WPA3 through this driver is unproven even on the host; say
      which security the access point ran.
- [ ] DHCP: `[NET-CORE] bind: interface up on driver.mt7921e0`, then a
      lease (`[NET-CORE] lease-status state=...`); a page loads in the
      browser. Leave it a few minutes: a `card stopped answering: radio off`
      line ends the session and is the thing to report.
- [ ] Send back: the boot log (`log > boot-log.txt`), `log driver- exit >
      drivers.txt`, the Linux lines above if any, the laptop model and
      card, and the first `[mt7921]` line that differs from this list. The
      driver's lines never carry a MAC address, SSID or BSSID.

## USB Ethernet: ASIX AX88179

The AX88179 driver (`driver.ax88179_0`) is host-proven only: QEMU has no
AX88179 and it has never run on one. A first boot with a dongle is its
test. The `[usbnet ax88179]` lines print on a development image (`Debug`
granted to the capsule); on another image, Settings' Network page says
whether the wired interface came up.

- [ ] What to boot: a development image with `microkernel-driver-ax88179`
      added (it is not in Standard until Decision 10 is ruled),
      with an AX88179 or AX88178A dongle (ASIX 0b95:1790 or 0b95:178a, or a
      rebrand listed in `userland/capsule_driver_ax88179/BINDING.md`) and a
      cable to a network with DHCP. Try it in a USB 3 port, then a USB 2
      port, then behind a hub.
- [ ] Before: if the machine runs Linux, save `lsusb -v -d 0b95:` (the
      vendor:product, bcdDevice and the configurations offered). An
      AX88179A or B may offer a CDC NCM configuration: say which.
- [ ] Bind: `[usbnet ax88179] port N: bound`, then `[usbnet ax88179] port N
      up, drawn station address programmed`. A line `port N 0b95:1790: <step>
      errno E` instead names the bring-up step the chip refused; keep it.
      `not this driver's device` for the dongle means its vendor:product is
      not listed or it showed no vendor interface.
- [ ] Link: with the cable in, `[NET-CORE] bind: interface up on
      driver.ax88179_0` within about ten seconds, then a lease
      (`[NET-CORE] lease-status state=...`); a page loads in the browser.
      Note the speed the switch or router shows for the port (10, 100 or
      1000).
- [ ] Unplug the cable, wait ten seconds, plug it in: the lease comes back.
      Unplug the dongle: `[usbnet ax88179] device stopped answering, given
      back`; plug it in again: bound and up again.
- [ ] A download of a few hundred megabytes in the browser completes;
      note how long it took.
- [ ] Send back: the boot log (`log > boot-log.txt`), `log usbnet >
      usbnet.txt`, the `lsusb` output if any, the dongle's make and model,
      and the first `[usbnet ax88179]` line that differs from this list.
      The logs never carry a MAC address or a serial number from this
      driver.

## Realtek RTL8153 USB Ethernet adapter

The RTL8153 driver (`capsule_driver_rtl8153`) is host-proven only: it has
never driven a real adapter. A first plug-in is its test. The `[usbnet
rtl8153]` lines print on a development image (`Debug` granted to the
capsule). Use a machine whose built-in Ethernet NONOS does not drive, or
leave its cable out, so the lease can only come through the adapter.

- [ ] What to plug: an RTL8153 or RTL8153B adapter (Realtek 0bda:8153, or a
      Lenovo, Microsoft, TP-Link, Linksys, Samsung or Nvidia rebrand; the
      ids are in `userland/capsule_driver_rtl8153/BINDING.md`), with a
      cable to a network that serves DHCP. If the machine also runs Linux,
      save `lsusb` and `dmesg | grep r8152` (its `Detected version` line).
- [ ] Boot a development image with `microkernel-driver-rtl8153` added (it
      is not in Standard until Decision 10 is ruled), then plug
      the adapter into a USB 3 port on the machine (not a hub) once the
      desktop is up; then try it again plugged in before boot.
- [ ] In the Terminal: `log rtl8153 net-core`. Expected, in order:
      `[usbnet rtl8153] started, looking for a device`,
      `[usbnet rtl8153] port N: bound`,
      `[usbnet rtl8153] port N up, drawn station address programmed`,
      `[NET-CORE] bind: interface up on driver.rtl8153_0`, then the lease
      line (`[NET-CORE] lease <address>/<prefix> gw <gateway>`), and a page
      loads in the browser.
- [ ] A refusal names its step: `chip is an RTL8156, not this driver's
      errno -19` (another chip under a listed id), `chip never finished
      loading its settings errno -110`, `PHY never came on errno -110`,
      `MAC reset never finished errno -110`, `buffer link list never ready
      errno -110`. Keep that line and the vendor:product it names.
- [ ] Unplug the cable, then plug it back: the link reads down, then up,
      and the lease comes back. Unplug the adapter: `device stopped
      answering, given back`; plug it in again: bound again.
- [ ] Send back: `log rtl8153 net-core > rtl8153.txt`, the boot log
      (`log > boot-log.txt`), the Linux lines above if any, the adapter's
      make and model. No line from this driver carries a MAC address or a
      serial number.
