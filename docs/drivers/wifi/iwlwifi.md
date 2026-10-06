# Intel Wi-Fi (iwlwifi)

The iwlwifi driver [capsule](../../overview/glossary.md#capsule) finds Intel Wi-Fi cards from the 7260 to the AX210 family; in 0.9.2 it boots firmware, scans and joins only on the SO platforms (AX211, and AX201 modules on those platforms), and that path has run only against a modelled device.

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
| BE200, BE201 | 272b, a840 | Not supported: not in the id table, the driver never takes them |

Only the AX210 family ids that carry transport values go past the first step, and only the SO ones among them have a bundled firmware; `bring_up` leaves every other card as setup left it and refuses it with `NotSoDevice` (`userland/capsule_driver_iwlwifi/src/server/radio/bring.rs:97-101`, `NotSoDevice`). The transport values are per PCI id (`userland/capsule_driver_iwlwifi/src/firmware/gen3/select.rs:119-130`, `transport`). The BE200 and BE201 ids are named but sit outside the id table (`userland/capsule_driver_iwlwifi/src/firmware/generation.rs:59-60`, `family_for_device`).
