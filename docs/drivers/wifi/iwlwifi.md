# Intel Wi-Fi (iwlwifi)

The iwlwifi driver [capsule](../../overview/glossary-boot-root-record.md#capsule) finds Intel Wi-Fi cards from the 7260 to the AX210 family and the Wi-Fi 7 BE200 and BE201; it boots firmware, scans and joins on the SO platforms (AX211, and AX201 modules on those platforms) and on the BE200 and BE201, and those paths have run only against a modelled device.

## State in this release

No Intel Wi-Fi card has a hardware report for 0.9.2. The SO path is complete in code and passes host tests against a model of the device and its firmware (`iwlwifi_proofs`). Every other Intel card the driver takes ends at the stage `NoAirPath`, which the Settings panel shows as `card not supported yet; use Ethernet or USB Wi-Fi`; the status reply carries the exact reason as a step and a detail word. Until a hardware report exists, do not count on Wi-Fi from an Intel card in this release; the alternatives are listed under [Wi-Fi chips with no driver](not-supported.md#what-to-use-instead).

## Which cards do what

The driver takes an Intel PCI function of class 0x02, subclass 0x80, with a memory BAR0, whose device id is in `family_for_device` (`userland/capsule_driver_iwlwifi/src/pci_match.rs:47-59`, `is_supported_adapter`; `userland/capsule_driver_iwlwifi/src/firmware/family.rs:19-39`, `family_for_device`). The card names come from `name` (`userland/capsule_driver_iwlwifi/src/firmware/generation.rs:32-65`).

| Cards | PCI device ids (vendor 8086) | State |
|---|---|---|
| 7260, 3160, 7265, 3165, 3168 | 08b1 to 08b4, 095a, 095b, 3165, 3166, 24fb | Refused: no boot path, stage `NoAirPath` |
| 8260, 4165, 8265, 8275 | 24f3 to 24f6, 24fd | Refused: no boot path, stage `NoAirPath` |
| 9260, 9162, 9461, 9462, 9560 | 2526, 271b, 271c, 30dc, 31dc, 9df0, a370 | Refused: no boot path, stage `NoAirPath` |
| AX200 | 2723 | Refused: no boot path, stage `NoAirPath` |
| AX201 or 9560 on Qu and QuZ platforms | 02f0, 06f0, 34f0, 3df0, 43f0, 4df0, a0f0 | Refused: no boot path, stage `NoAirPath` |
| AX211, AX201 module on SO platforms | 51f0, 51f1, 54f0, 7a70, 7af0, 7f70 | Partial: boot, scan and join pass against the model |
| AX210 discrete (TY) | 2725 | Refused: its firmware file is not in the tree |
| Meteor Lake (MA) | 2729, 7e40 | Refused: its firmware file is not in the tree, and a MAC step other than B is refused before that |
| AX210 family ids with no transport values | a74f, 272f | Refused: not an SO platform |
| BE200 (Gl, discrete) | 272b | Partial: boot, scan and join pass against the model in the V2 CSR layout; host-proven, never booted on a BE200 |
| BE201 (Bz, the CNVi of Lunar Lake and Arrow Lake) | a840, 7740, 4d40 | Partial: as the BE200; host-proven, never booted on a BE201 |

Only the ids that carry transport values go past the first step, and only the SO, BE200 and BE201 ones among them have a bundled firmware; `identify` leaves every other card as setup left it and refuses it with `NotSoDevice` (`userland/capsule_driver_iwlwifi/src/server/radio/bring/identify.rs`, `identify`). The transport values are per PCI id (`userland/capsule_driver_iwlwifi/src/firmware/gen3/select/transports.rs`, `transport`).

## BE200 and BE201

The BZ family runs the same gen3 boot as the AX210 family in Linux v6.12's second CSR layout (`gen3::csr::CsrLayout::V2`): D0A through MAC_INIT and MAC_STATUS, NIC access on bits 21 and 20, the software reset and the bus-master stop in CSR_GP_CNTRL, the ROM started by CSR_FUNC_SCRATCH and ROM_START, the MSI-X software error on bit 5, the receive index through HBUS_TARG_WRPTR, four-byte used-ring entries and 4096-entry byte count tables. The transport values are cfg/bz.c's: the BE200 (`iwl_gl_trans_cfg`) is discrete with no LTR delay, the BE201 (`iwl_bz_trans_cfg`) integrated with a 2500 us LTR delay; both have the 12000 us low-latency crystal.

| Card | MAC, step | Radio | Firmware file | PNVM file |
|---|---|---|---|---|
| BE200, 8086:272b | Gl 0x47, C (CSR_HW_REV) | FM 0x112, C | `iwlwifi-gl-c0-fm-c0-92.ucode` (1913128 bytes) | `iwlwifi-gl-c0-fm-c0.pnvm` (295036 bytes) |
| BE201, 8086:a840, 7740, 4d40 | Bz 0x46, B (read from the CNVi, CNVI_AUX_MISC_CHIP) | FM 0x112, C | `iwlwifi-bz-b0-fm-c0-92.ucode` (1926176 bytes) | `iwlwifi-bz-b0-fm-c0.pnvm` (295356 bytes) |

The four files are linux-firmware's at commit bf992df7a95b5e19bb8f8546eb4d7c53ab2598d0, firmware API 92, in `nonos-bootloader/firmware/intel/`, pinned by sha256 in `scripts/baselines/upstream.sha256`. Licence: Redistributable, under LICENCE.iwlwifi_firmware (linux-firmware's WHENCE), the text in `nonos-bootloader/firmware/intel/LICENSE`. These images list PHY_CONTEXT_CMD version 5 and LINK_CONFIG_CMD version 2, which the driver encodes (for a legacy link they are the bytes of versions 4 and 1), and answer the regulatory update in its version 8 reply. Any other MAC step, radio step, a dual-radio (fm4) module or another radio is refused by name (step 4; a radio step is detail 0x7000s).

This is host-proven only, never booted on a BE200 or BE201: `iwlwifi_proofs` parses the real files, boots both images against a model that answers only V2 bits, and catches a mutant of each V2 rule (`gen3_bz_tests`). [The real-hardware checklist](../../hardware/real-hardware-checklist.md#intel-be200-and-be201) says what a first boot should show.

## Parts

- [Firmware choice and bring-up](iwlwifi/bring-up.md)
- [Station address, scanning, joining and authority](iwlwifi/running.md)
- [The status reply, the firmware files and the tests](iwlwifi/status.md)

## Not supported

- Hidden networks, open, TKIP and Enterprise networks.
- 6 GHz, HT, VHT and HE rates, rate scaling, QoS and aggregation. Power save is Linux's balanced scheme only.
- Interrupt-driven operation, roaming, access point and monitor mode.

## See also

- [Wi-Fi drivers](README.md)
- [Realtek RTL8821CE](rtl8821ce.md)
- [Wi-Fi chips with no driver](not-supported.md)
- [The broker API](../broker-api.md)
- [IOMMU](../../kernel/iommu.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
