# Intel Wi-Fi (iwlwifi)

The iwlwifi driver [capsule](../../overview/glossary.md#capsule) finds Intel Wi-Fi cards from the 7260 to the AX210 family; in 0.9.2 it boots firmware, scans and joins only on the SO platforms (AX211, and AX201 modules on those platforms), and that path has run only against a modelled device.

## State in this release

No Intel Wi-Fi card has a hardware report for 0.9.2. The SO path is complete in code and passes host tests against a model of the device and its firmware (`iwlwifi_proofs`). Every other Intel card the driver takes ends at the stage `NoAirPath`, which the Settings panel shows as `card not supported yet; use Ethernet or USB Wi-Fi`; the status reply carries the exact reason as a step and a detail word. Until a hardware report exists, do not count on Wi-Fi from an Intel card in this release; the alternatives are listed under [Wi-Fi chips with no driver](not-supported.md#what-to-use-instead).
