# Hardware support matrix: Audio; Wi-Fi

Part of [Hardware support matrix](MATRIX.md).

## Audio

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Audio | HD Audio controller, class 04/03 any vendor; Intel class 04/01 | `capsule_driver_hda` | Works | proof crate `hda_proofs` (116); QEMU `intel-hda`; real hardware (Intel HD Audio) | 0.9.2 |
| Audio | Realtek ALC codecs, 10ec vendor, with Linux's EAPD coefficients and ALC256 setup | `capsule_driver_hda` | Works | proof crate `hda_proofs` with ALC236 and ALC269 models | 0.9.2 |
| Audio | Other HD Audio codecs with an analog output | `capsule_driver_hda` | Works | proof crate `hda_proofs` with QEMU's duplex codec model; QEMU `hda-duplex` | 0.9.2 |
| Audio | HDMI and DisplayPort audio; graphics controllers 1002, 10de, 8086:490d, 4f90, 4f91, 4f92, e2f7 | `capsule_driver_hda` | Refused: NONOS plays through speakers, headphones and line out only; Settings and the player say so | proof crate `hda_proofs` | 0.9.2 |
| Audio | Intel SST engines 8086:9c36, 9cb6, 0f28, 22a8, 119a, and Intel DSP-only laptops | `capsule_driver_hda` | Refused: needs Intel SOF, which NONOS does not have; Settings and the player say so | proof crate `hda_proofs` | 0.9.2 |
| Audio | AMD audio coprocessor (ACP), 1022 class 04/80 | `capsule_driver_hda` | Refused: needs an ACP driver, which NONOS does not have; Settings and the player say so | proof crate `hda_proofs` | 0.9.2 |
| Audio | Microphones and recording | none | Not supported: no input stream is opened | none | 0.9.2 |

## Wi-Fi

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Wi-Fi | Realtek RTL8821CE 10ec:c821 | `capsule_driver_rtl8821ce` | Works | proof crate `rtl8821ce_proofs` (171); real hardware (scan, join, DHCP, DNS, browser traffic) | 0.9.2 |
| Wi-Fi | Intel AX210 family on SO platforms 8086:51f0, 51f1, 54f0, 7a70, 7af0, 7f70 | `capsule_driver_iwlwifi` | Partial: firmware start, scan and WPA2 and WPA3 join run against a modelled device; not run on the air in this release | proof crate `iwlwifi_proofs` (205) | 0.9.2 |
| Wi-Fi | Intel 8086:2725, 7e40, 2729 | `capsule_driver_iwlwifi` | Refused: their firmware image is not in the tree | proof crate `iwlwifi_proofs` | 0.9.2 |
| Wi-Fi | Intel 8086:08b1 to 08b4, 095a, 095b, 3165, 3166, 24fb, 24f3 to 24f6, 24fd, 2526, 9df0, a370, 31dc, 30dc, 271b, 271c, 2723, 34f0, 3df0, 4df0, 02f0, 06f0, 43f0, a0f0, 272f, a74f | `capsule_driver_iwlwifi` | Refused: no boot path in this driver. The driver's line naming the card is not printed, since it holds no Debug capability; Settings shows `card not supported yet` | proof crate `iwlwifi_proofs` | 0.9.2 |
| Wi-Fi | Intel BE200 and BE201 8086:272b, a840 | none | Not supported: outside the iwlwifi id table, so the driver leaves them, and Settings says the chip has no NONOS driver | none | 0.9.2 |
| Wi-Fi | Every other Wi-Fi chip | none | Not supported: no driver | none | 0.9.2 |

For a card NONOS cannot run, Settings suggests Ethernet or USB Wi-Fi (`no_driver`, `userland/capsule_settings/src/settings/ui/live_wifi.rs:115-129`; `NoAirPath`, `userland/nonos_wifi_client/src/driver/stage.rs:60`). In 0.9.2 no USB Wi-Fi adapter has a driver, and of the Ethernet drivers in the image only virtio-net, the virtual machine's card, gets received frames through; the table below says why.
