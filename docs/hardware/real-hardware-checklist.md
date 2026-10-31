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

## Battery, AC and lid: HP laptop (Gemini Lake)

capsule_acpi runs the laptop's own AML and reads the battery through the
embedded controller; it is built power-only (`userland/capsule_acpi/README.md`,
"Release build"), so it changes nothing else on the machine. It has never
run on this laptop: QEMU has no battery. The capsule's own `[ACPI]` lines
print on a development image only (`standard-dev`, `Debug` granted to the
capsule); on Standard, the bar and About are the test.

- [ ] What to boot: `standard-dev` from the same commit as the result you
      report, on battery, AC unplugged, lid open.
- [ ] The bar, within a minute of the desktop: a percentage beside a gauge
      ("57%"), not "Battery status unavailable" and not "No battery". Note
      the percentage, and what the firmware's own setup screen or another
      OS says it is (a few points apart is fine).
- [ ] About, System: the Power card says Battery `57%` (the same number),
      State `Discharging`, AC `Offline`.
- [ ] Plug the AC in. Within a minute the bar says "57% Charging" and
      About says State `Charging`, AC `Online`. A full battery on AC says
      the number alone and State `Not charging`. Unplug: within a minute
      the word Charging goes and About says `Discharging`, `Offline`.
      Note how many seconds each change took: under a few is the EC's
      event; up to a minute is the poll.
- [ ] Close the lid for five seconds, then open it: the screen goes dark
      while it is closed and lights again (within 2 s of opening). The
      machine stays on.
- [ ] Press the power button once, briefly: the desktop's power choice
      appears, as it did before this capsule. Nothing powers off by itself.
- [ ] In the Terminal: `log acpi > acpi.txt`. It should hold, in order:
      `[ACPI] build: power only`, `[ACPI] EC data=...`
      (ports and GPE), the `_REG failed in 0 scopes` and `_INI` lines, `inventory: N devices`,
      `GPEs armed ...`, `[ACPI] sci irq=...`, `first publish pass done`,
      and `power battery=yes charge=57 ac=offline` (again on each change),
      plus the kernel's `[ACPI] region ...` grants and its posture line.
      Then `log driver- exit > drivers.txt` as usual.
- [ ] Send back: `acpi.txt`, `drivers.txt`, photos of the bar on battery
      and on AC, and the About Power card. Say if any line reads `faulted`,
      `refused`, `SCI not bound` or `no usable`, and quote it. The lines
      carry no serial number: the battery's serial string is never logged.

## Intel Wi-Fi on a first boot: ASUS NUC 15 Pro, ThinkPad T14 (Intel)

Which list below applies is read from the card, not the model name: the
first `[iwlwifi] pci 0x0000xxxx` line, or `lspci -nn -d 8086:` under Linux.
7740, a840, 4d40 (BE201) and 272b (BE200): the BE200 and BE201 list.
51f0, 51f1, 54f0, 7a70, 7af0, 7f70 (AX211, or an AX201 module on an Alder
or Raptor Lake platform, `so-a0-gf-a0-86` or `so-a0-hr-b0-84`): the BE200
list with that image. 7e40, 2729: the Meteor Lake list. 02f0, 06f0, 34f0,
3df0, 4df0, 43f0, a0f0 (AX201 CNVi): the 22000-family list. Then, on every
one of them:

- [ ] Cold and warm: boot once from power-off (ten seconds unplugged) and
      once by rebooting straight from Windows or Linux with Wi-Fi in use.
      Both reach `radio up; passive scanning`. If only the warm boot
      fails, note its step and detail: step 3 detail 1 (`the NIC never
      reported ready`: the previous system or the management engine still
      holds it), or step 7 detail 0x30 (`never raised ALIVE`). Linux
      answers the second by removing and rescanning the PCI function, which
      this driver cannot; a cold power-off is the workaround.
- [ ] Probe: no step 4 refusal with detail 0x30000 (an RF id of type 0). A
      blank OTP is now named from the CRF id as Linux names it; 0x30000
      means a CRF type Linux does not map either. Compare with Linux's
      `Detected crf-id 0x..., cnv-id 0x..., wfpm id 0x...` line.
- [ ] Integrated AX211, AX201 on SO, Meteor Lake: `firmware alive` comes.
      A step 7 detail 0x30 on these parts points at the image loader, which
      the driver now waits for back to back for up to 100 ms without
      sleeping, as Linux does; keep the `secure boot status` line.
- [ ] Regulatory: `up: N channels, tx ant A, regulatory 0x....`: 0x5a5a
      ("ZZ", the world domain) when the firmware knows no country yet, else
      the country's two letters (0x5553 "US"). In ZZ the 5 GHz channels are
      passive: join a 5 GHz network as well as a 2.4 GHz one. A 2.4 GHz join
      that works with a 5 GHz one that does not points here.
- [ ] Transmit power: no SAR, geographic SAR, PPAG, TAS or LARI table is
      sent (this driver reads no BIOS table; Linux sends none of them on a
      machine without the tables, and the firmware then transmits at its
      NVM and PNVM limits). Note the signal of the joined network beside
      what Linux shows at the same spot.
- [ ] Panel: on any failure, a photo of the Settings Wi-Fi row, which reads
      `Intel Wi-Fi: stopped at N: <what> (0x<detail>)`; the step and detail
      table is `docs/drivers/wifi/iwlwifi/status.md`. Step 8 detail
      0x01ggcc0w names the command (group gg, command cc) and how it ended
      (w: 1 timed out, 2 the firmware's error, 3 too large, 4 busy).

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

## Intel AX210 (discrete) and Meteor Lake

The iwlwifi driver's TY and MA paths (the discrete AX210 8086:2725; the
Meteor Lake CNVi 8086:7e40 and 2729 with an AX211-class module) are
host-proven only: neither has booted. They run the AX211 path with each
MAC's own image and PNVM file; the AX210 is discrete (its LTR in the CSR,
no image-loader spin), a dual-radio Meteor Lake module (GF4) puts its 5 GHz
PHY on the second LMAC. Run the BE200 list above with these changes.

- [ ] What to boot: a `standard-dev` image at or after the TY and MA commit,
      on a desktop with an AX210 card (an M.2 card, not the AX211 CNVi) or a
      Meteor Lake (Core Ultra 1st gen) laptop with Intel Wi-Fi.
- [ ] Before: as for the BE200 (`lspci -nn -d 8086:`, `dmesg | grep -i
      iwlwifi`; on Meteor Lake the `loaded firmware version` line names
      ma-b0-gf-a0 or ma-b0-gf4-a0).
- [ ] Identity: `[iwlwifi] pci 0x00002725` then `[iwlwifi] firmware
      iwlwifi-ty-a0-gf-a0-86.ucode`; or `pci 0x00007e40` (or `2729`) then
      `iwlwifi-ma-b0-gf-a0-86.ucode` or `-gf4-a0-86.ucode`, the one Linux
      loaded. A step 4 refusal: detail 0x5xxxx an MA MAC not at step B,
      0x7xxxx an MA radio not at step A, 0x3xxxx an HR or other radio.
- [ ] ALIVE: `firmware alive: status 0x0000cafe ...`, then `platform NVM
      section 0x... given` (or `no platform NVM section for this SKU`). On
      the AX210 a step 7 detail 0x30 points at the discrete LTR; keep the
      `secure boot status` line.
- [ ] NVM, scan, join (WPA2 and WPA3) and DHCP as for the BE200. On a GF4
      module, join a 5 GHz network as well as a 2.4 GHz one: that is the
      second-LMAC PHY context.
- [ ] Send back: as for the BE200, saying AX210 card or Meteor Lake laptop.

## Intel AX200 and AX201 (22000 family)

The iwlwifi driver's 22000-family path (the discrete AX200 8086:2723; the
AX201 CNVi on Ice, Comet, Tiger and Jasper Lake platforms, 8086:02f0,
06f0, 34f0, 3df0, 4df0, 43f0 and a0f0) is
host-proven only: it has never booted on any of them. It boots through the
version 1 context info, not the AX210's, and its rings, packets, receive
descriptor and transmit command are this family's; a first boot is the
test of each.

- [ ] What to boot: a `standard-dev` image at or after the 22000-family
      commit, on a laptop with an AX201 (Ice, Comet, Tiger or Jasper Lake
      with Intel Wi-Fi 6) or a machine with an AX200 card.
- [ ] Before: if the machine runs Linux, save `lspci -nn -d 8086:` and
      `dmesg | grep -i iwlwifi` (its `loaded firmware version` line names
      Qu-b0-hr-b0, Qu-c0-hr-b0, QuZ-a0-hr-b0 or cc-a0; its subsystem id is in
      `lspci -nnv`).
- [ ] Identity: `[iwlwifi] pci 0x000002f0` (or the platform's id) then
      `[iwlwifi] firmware iwlwifi-Qu-b0-hr-b0-77.ucode` (or `-Qu-c0-`,
      `-QuZ-a0-`), the one Linux loaded; on the AX200 `pci 0x00002723` and
      `iwlwifi-cc-a0-77.ucode`. A step 4 refusal with detail 0x30105 is a
      9560-class JF module on that id (not run, expected); 0x5xxxx a Qu MAC
      of another step; 0x4xxxx a CDB radio.
- [ ] Start: no `radio not up: the device's persistence bit is set and
      write-protected` (step 3 detail 4, or step 7 detail 0x14); if it
      appears, a cold power-off, not a reboot, should clear it. Step 3
      detail 3 (step 7 detail 0x13) means a peripheral write did not wake
      the MAC.
- [ ] ALIVE: `firmware alive: status 0x0000cafe umac 77.N sku 0x...`, no
      PNVM line (an HR radio has none). If it never comes (step 7, detail
      0x30), keep the `secure boot status` line before it: it says whether
      the ROM took the context info.
- [ ] NVM: `[iwlwifi] up: N channels, tx ant A, regulatory 0x....`, then
      `radio up; passive scanning`, without `joining off: ...` or `scan
      request version other than 15 or 17`.
- [ ] Scan: `first scan complete: N networks from M beacons` within about
      ten seconds, with networks on both bands (the version 15 request puts
      each channel's band in another byte than the AX210's does; a scan
      that sees 2.4 GHz only, or none, points there).
- [ ] Join (WPA2, then WPA3): `join: port open; sent 4 ...` (WPA3: `sent
      5`), then DHCP and a page load as for the BE200. Keep a download
      running for a minute: the transmit rings wrap every 256 frames, and a
      stall after a few hundred frames points at the wrap.
- [ ] Send back: as for the BE200, with which laptop or card, the Linux
      firmware name and the subsystem id from `lspci -nnv`.

## Intel cards this driver does not run

- [ ] On a machine with a 7000, 8000 or 9000-family card (7265, 8265, 9260,
      9560 on Cannon Lake or Whiskey Lake): `[iwlwifi] pci 8086:<id> <what>:
      present, not run by this driver; given up`, no claim, then `[EXIT]
      <service> status 6: device present, bring-up failed and was given up`;
      not status 2. Send back the line and `lspci -nn -d 8086:`.

## MediaTek MT7921 and MT7922

The mt7921e driver (14c3:7961 MT7921E/RZ608, 14c3:0616 and 14c3:7922
MT7922/RZ616, 14c3:0608, 14c3:7920, 0b48:7922) is host-proven only: it has
never run on a card. A first boot is its test. The `[mt7921]` lines print on
a development image (`Debug` granted to the capsule); on another image the
Settings Wi-Fi panel shows the stage. A ThinkPad T14 Gen 4 AMD carries the
MT7922.

- [ ] What to boot: the sealed `standard` image (or a `standard-dev` from
      `tools/nonos-dev-image`) at or after the commit whose seal places the
      firmware package, or `make nonos-mk-usb-img`. Either way the stick's
      `EFI/nonos/firmware/mediatek/` holds six `WIFI_*` files and
      `LICENCE.mediatek` (check with `mdir` or by mounting the stick). A stick
      without them boots silently past them and the panel reads "The firmware
      package is missing from the boot medium". Boot it live: an installed
      disk does not carry the package yet. Wi-Fi on in the firmware settings.
- [ ] Before: if the machine runs Linux, save `lspci -nn -d 14c3:` and
      `dmesg | grep -i mt79` (the `HW/SW Version` and `WM Firmware Version`
      lines). Booting straight from Windows or Linux (a warm reboot) is
      fine and worth trying once: the driver resets a card left running.
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
      8 never ready, 9 the MCU's ownership) or `mac init failed at step N`
      (1 capabilities, 2 efuse, 3 MAC enable, 4 channel domain, 5 RX path,
      6 transmit power limits), then `attempt N failed; resetting`; after
      three, `card quiet: WFDMA off, bus master off` and `stopped at stage 4`.
- [ ] Interface: `[mt7921] stage interface: station added`, `[mt7921] stage
      ready: scanning`, `[mt7921] serving driver.mt7921e0`. `stopped at
      stage 7` means the kernel gave no randomness.
- [ ] Scan: within about ten seconds the Settings panel lists the networks
      in range, 2.4 and 5 GHz. If it lists none, the status reply's
      diagnostic words (chip, channel grants, grants missed, bad frames,
      refused commands) say whether the firmware ever granted a channel;
      keep the log.
- [ ] Join WPA3 first if the router offers it (WPA3 or WPA2/WPA3 mixed: the
      station picks SAE), then WPA2: the panel reads Joined. Both are proven
      on the host only; say which security the access point ran, and whether
      it is 2.4 or 5 GHz.
- [ ] Keys: as the join finishes, `[mt7921] pairwise key in the chip` and
      `[mt7921] group key in the chip`. `pairwise key refused by the
      firmware` means the station fell back to CCMP in software: report it,
      and whether DHCP still works (that says whether the chip passes up a
      protected frame it holds no key for). Joined with both keys in but no
      lease: the status reply's bad-frames word rising means the chip
      reports decrypt errors; keep the log. Leave the link up past the
      access point's group rekey (often an hour): a second `group key in the
      chip`, and traffic still flowing, is the rekey working.
- [ ] DHCP: `[NET-CORE] bind: interface up on driver.mt7921e0`, then a
      lease (`[NET-CORE] lease-status state=...`); a page loads in the
      browser. Leave it a few minutes, on battery too: a line `[mt7921]
      ownership: taken back from the firmware, time N` means the laptop's
      power management took the card and the driver took it back (expected
      rarely, keep it); `ownership: the firmware holds the card and would
      not give it back` means nothing is sent until it does, and `card
      stopped answering: radio off` ends the session: either is the thing
      to report.
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
      added (it is not in Standard until Decision 10 is decided),
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
      is not in Standard until Decision 10 is decided), then plug
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

## Realtek RTL8822CE Wi-Fi

The RTL8822CE driver (`driver.rtl8822ce0`) is host-proven only: QEMU has
no RTL8822CE and it has never run on one. A first boot with the card is
its test. The `[rtl8822ce]` lines print on a development image (`Debug`
granted to the capsule); on another image read the same facts from the
Settings Wi-Fi panel's status (stage, step, detail). It does no IQK or
DPK yet, so a weaker signal than under Linux is expected, not a fault.

- [ ] What to boot: a `standard-dev` image at or after the RTL8822CE
      commit, on a laptop with an RTL8822CE (10ec:c822, or 10ec:c82f).
      Wi-Fi on in the firmware settings; the RF-kill switch on. Leave any
      wired cable out so a lease can only come over Wi-Fi.
- [ ] Before: if the machine runs Linux, save `lspci -nn -d 10ec:` and
      `dmesg | grep -i rtw88` (the firmware version line, and any `failed
      to` line).
- [ ] Inventory: `log HW` shows `[HW] 10ec:c822 02/80 driven by
      driver_rtl8822ce` (or `c82f`).
- [ ] Bring-up, in the Terminal `log rtl8822ce`. Expected, in order:
      `[rtl8822ce] chip powered and answering; bus master on`,
      `[rtl8822ce] firmware v9.9`,
      `[rtl8822ce] firmware loaded and ready`,
      `[rtl8822ce] trx engines enabled`,
      `[rtl8822ce] mac initialised`,
      `[rtl8822ce] phy tables loaded, two rf paths`,
      `[rtl8822ce] antenna granted to wi-fi`,
      `[rtl8822ce] phy configured`,
      `[rtl8822ce] serving net_core`.
- [ ] A refusal names its stage; keep the line: `power-on sequence failed;
      chip given back`, `no response after power-on; chip given back`,
      `pcie clock delay: dbi write timed out`, `efuse unreadable on a
      freshly powered mac`, `firmware download failed` (with an `fw:` line
      before it), `rf front-end type has no table in this driver, radio not configured`, `coex
      window did not answer; antenna not granted`. `dac calibration did
      not read back; radio left as calibrated` is not fatal: note it and
      go on. `phy tables loaded, one rf path` on this 2x2 card is a
      finding: note it.
- [ ] Scan: within about ten seconds the Settings panel lists the networks
      in range, adapter `Realtek RTL8822CE`. Note the signal the panel
      shows for your own access point at one metre and at five, and what a
      phone shows at the same spots.
- [ ] Join (WPA2, then WPA3 if an access point offers it): the panel reads
      Joined; `[NET-CORE] bind: interface up on driver.rtl8822ce0`, then a
      lease (`[NET-CORE] lease-status state=...`); a page loads in the
      browser. A download of a hundred megabytes completes; note how long.
- [ ] Send back: `log rtl8822ce net-core > rtl8822ce.txt`, the boot log
      (`log > boot-log.txt`), `log HW > devices.txt`, the Linux lines
      above if any, the laptop model, the signal notes, and the first
      `[rtl8822ce]` line that differs from this list. No line from this
      driver carries a MAC address, an SSID or a serial number.

## Realtek RTL8852BE Wi-Fi

The RTL8852BE driver (`driver.rtl8852be0`) is host-proven only: QEMU has
no RTL8852BE and it has never run on one. A first boot with the card is
its test. The `[rtl8852be]` lines print on a development image (`Debug`
granted to the capsule); on another image read the same facts from the
Settings Wi-Fi panel's status (the stage). It
runs the card as a 20 MHz station with 802.11a/b/g rates and no TSSI or
DPK, so slower links and a weaker signal than under Linux are expected,
not a fault.

- [ ] What to boot: a `standard-dev` image at or after the RTL8852BE
      commit, on a laptop with an RTL8852BE (10ec:b852, or 10ec:b85b).
      Wi-Fi on in the firmware settings; the RF-kill switch on. Leave any
      wired cable out so a lease can only come over Wi-Fi.
- [ ] Before: if the machine runs Linux, save `lspci -nn -vv -d 10ec:`
      (the memory BAR's size, expected 1M) and `dmesg | grep -i rtw89`
      (the `Firmware version 0.29.29.15` line, the `chip rfe_type` line,
      and any `failed to` line).
- [ ] Inventory: `log HW` shows `[HW] 10ec:b852 02/80 driven by
      driver_rtl8852be` (or `b85b`).
- [ ] Bring-up, in the Terminal `log rtl8852be`. Expected, in order:
      `[rtl8852be] firmware up, efuse and phy capabilities read`,
      `[rtl8852be] station interface ready; tx power open loop, no TSSI or DPK`,
      `[rtl8852be] serving`. The panel's stage reads Ready.
- [ ] A refusal names its step; keep the line and the panel's stage:
      `claim or map refused, step N errno M`, `bus master refused`,
      `ring mapping failed` (stage 5), `probe stopped` (stage 2 power, 4
      firmware or mailbox, 6 efuse, 12 a PCIe or MAC step),
      `one-antenna board: only two-antenna boards are driven` (stage 8: note
      the laptop model),
      `start stopped` (stage 12: a MAC, PHY or calibration step), `no
      entropy for an address` (stage 7), `station interface set-up
      failed` (stage 12). `firmware did not take every rf register page`
      and `mac: BA CAM reset did not finish` are not fatal: note them and
      go on.
- [ ] Scan: within about ten seconds the Settings panel lists the networks
      in range on 2.4 and 5 GHz, adapter `Realtek RTL8852BE`. The signal
      shown is not a measurement on this driver (RSSI is not read yet).
- [ ] Join (WPA2, then WPA3 if an access point offers it, once on a
      2.4 GHz and once on a 5 GHz network): the panel reads Joined;
      `[NET-CORE] bind: interface up on driver.rtl8852be0`, then a lease
      (`[NET-CORE] lease-status state=...`); a page loads in the browser.
      A download of a hundred megabytes completes; note how long, and the
      distance to the access point.
- [ ] Keys: as the join finishes, `[rtl8852be] pairwise key in the chip`
      and `[rtl8852be] group key in the chip`. `pairwise key refused by the
      firmware` means the DMA engine never took the command and the station
      runs CCMP in software: report it, and whether a lease still comes
      (whether the chip passes up a protected frame it holds no key for).
      Joined with both keys in but no lease: keep the log and the status
      reply's words (the third counts frames dropped for a bad CRC or ICV).
      Stay joined past the access point's group rekey (often an hour): a
      second `group key in the chip` and traffic still flowing.
- [ ] Leave: disconnect from the panel; the link reads down and the scan
      resumes. Join again: it comes back.
- [ ] Send back: `log rtl8852be net-core > rtl8852be.txt`, the boot log
      (`log > boot-log.txt`), `log HW > devices.txt`, the Linux lines
      above if any, the laptop model, the download notes, and the first
      `[rtl8852be]` line that differs from this list. No line from this
      driver carries a MAC address, an SSID or a serial number.

## Bluetooth on the HP laptop: the RTL8821CE's Realtek controller

The RTL8821CE combo card carries Wi-Fi on PCIe (10ec:c821,
`capsule_driver_rtl8821ce`) and Bluetooth on the laptop's internal USB,
behind xHCI (`capsule_driver_bt_usb`). The Bluetooth half needs Realtek's
patch before it is used: the capsule loads `rtl8821c_fw.bin` and
`rtl8821c_config.bin` (linux-firmware's, embedded) with `nonos_bt_rtl`,
then brings it up and pairs Bluetooth LE keyboards and mice. It is
host-proven on a modelled RTL8821C only; this boot is its first on real
hardware. The `bt-usb:` lines print on a development image
(`standard-dev`; `Debug` granted). Hardened and Air-Gapped carry no
Bluetooth at all.

- [ ] Before booting, under Linux if the HP has it: save `lsusb` (the
      Bluetooth line: 0bda:b00a, 0bda:c821 or 13d3:3529), and
      `dmesg | grep -i 'bluetooth: hci0: RTL'` (its `rom_version status=0
      version=N`, `loading rtl_bt/rtl8821c_fw.bin` and `fw version 0x...`
      lines). The ROM version decides which patch is sent.
- [ ] Power the laptop fully off first (not a reboot from Linux): a patch
      Linux loaded survives a warm reboot, and the capsule then rightly
      says `Realtek controller runs its own firmware` instead of loading.
      Try that warm case too, second, and note which line it gave.
- [ ] Boot a `standard-dev` image. In the Terminal: `log bt-usb`. Expected,
      in order: `bt-usb: started, looking for a controller`, then within a
      few seconds `bt-usb: Realtek patch loaded: ROM 1, firmware
      1975054488, 139 fragments` (ROM 0: 78 fragments), then
      `bt-usb: controller up`.
- [ ] A refusal names its step: `controller not usable: ...` (binding; for
      `ACL endpoints refused` keep the whole log), `Realtek patch not
      loaded: no patch for this ROM version` (keep the Linux `rom_version`
      line), `... controller failed or refused the setup` (a fragment
      refused), `controller given back: ...`. Keep that line.
- [ ] Settings, Bluetooth: Controller reads `Controller ready`, Status
      `Looking for devices`, Scanning `On`.
- [ ] A BLE keyboard (one that pairs by passkey): put it in pairing mode,
      press P on the page. The Asks row shows `Type NNNNNN on it, then
      its Enter`; type those six digits on the keyboard and Enter. The log
      shows `bt-usb: connected`, `paired`, `bond kept`, `HID ready:
      keyboard`, and no digits. Type in the Terminal: the letters appear.
      The Paired devices row reads `Paired with a passkey or number
      checked`, `Connected, battery N%`.
- [ ] A BLE mouse: P again, then pairing mode on the mouse. It pairs with
      nothing asked (`Paired without a check: Just Works`); the pointer
      moves and the buttons click. A keyboard that paired by Just Works
      would not type: the log says `keys not posted: paired with no
      passkey typed`.
- [ ] Switch the keyboard off and on: `link lost`, then `encrypted with the
      kept bond` and it types again without pairing. F forgets it.
- [ ] With Wi-Fi joined, pair and type again: the card's Wi-Fi and
      Bluetooth share one antenna, and NONOS sets no coexistence; note any
      drop in either.
- [ ] Send back: `log bt-usb > bt.txt`, the boot log (`log > boot-log.txt`),
      the Linux lines above, the keyboard's and mouse's make and model.
      No `bt-usb:` line carries a Bluetooth address, a device name, a key
      or a passkey; report any that does.

## Bluetooth on the Intel and MediaTek laptops: T14 Intel, NUC 15 Pro, T14 AMD

The same capsule as the HP's (`capsule_driver_bt_usb`), with the vendor's own
setup before bring-up (`userland/capsule_driver_bt_usb/README.md`, "Intel
controllers" and "MediaTek MT7921 and MT7922"). Host-proven only; each of
these boots is the first on real hardware. Pairing, the page and what to send
back are the HP section's above; only the setup lines differ. Use a
`standard-dev` image so the `bt-usb:` lines print.

- [ ] Before booting, under Linux if the machine has it: save `lsusb` (the
      8087:... or 0e8d:... Bluetooth line) and `dmesg | grep -i
      'bluetooth: hci0'` (Intel: the `Found device firmware: intel/ibt-...`
      line; MediaTek: `HW/SW Version` and the `BT_RAM_CODE_...` name).
- [ ] Power fully off first, as on the HP: firmware Linux loaded survives a
      warm reboot.
- [ ] T14 Intel (AX201 or AX211) and NUC 15 Pro (BE201 on Arrow Lake CNVi):
      `log bt-usb` shows `bt-usb: intel: sending ibt-...` (AX201
      `ibt-19-0-4`, AX211 `ibt-1040-0041`; the BE201 sends
      `ibt-0190-0291-iml` first, then `ibt-0190-0291-usb`), then `intel:
      booted, DDC from ibt-...` and `bt-usb: controller up`. A controller
      already running the carried build says `intel: firmware already
      running: ibt-...`; one running another says `running firmware is not
      ..., sent back to load it` and is found again. `firmware not carried:
      ...` names a file this release lacks: keep that line and the Linux
      one. On the BE201, a refusal after the loader stage may be the DSBR
      word, which NONOS sends as zero (Linux reads it from a UEFI
      variable): keep the whole log.
- [ ] T14 AMD (MT7922): boot live from a stick built with the firmware
      package (the Wi-Fi section above; an installed disk has no package
      yet). `log bt-usb` shows `bt-usb: MediaTek MT7922 patched: N sections
      sent, M held`, then `controller up`. `MediaTek controller not set up:
      ...` names the step; `the firmware package is missing from the boot
      medium` means the stick has no `EFI/nonos/firmware/mediatek/`.
- [ ] Then the HP section's pairing steps: a passkey keyboard, a Just Works
      mouse, switch the keyboard off and on. On a combo card note any Wi-Fi
      or Bluetooth drop while both run: NONOS sets no coexistence.
- [ ] Send back: `log bt-usb > bt.txt`, `log > boot-log.txt`, the Linux
      lines, and the keyboard's and mouse's make and model.
