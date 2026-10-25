# Coverage: Input, EC and battery

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

| Priority | Part | Years | How common | Ids | Named by the driver | In Standard | Owner | Gap |
|---|---|---|---|---|---|---|---|---|
| P1 | I2C-HID touchpads on Intel LPSS (Skylake to Meteor Lake) | 2017-2026 | most machines | `8086:9d60` `8086:a368` `8086:02e8` `8086:a0e8` `8086:51e8` `8086:7acc` `8086:7a4c` `8086:7e78` | 8 of 8 | yes | F, B | listed through Meteor Lake-P; works on the bench Gemini Lake |
| P1 | I2C-HID on Arrow Lake, Lunar Lake, Panther Lake LPSS | 2024-2026 | most machines | - | - | yes | F, B | not in the table; ids to be read from the parts' datasheets |
| P1 | I2C-HID on AMD (AMDI0010) | 2018-2026 | most machines | `acpi:AMDI0010` | 1 of 1 | yes | F | matched; never booted on an AMD laptop |
| P1 | Embedded controller, battery, AC, lid | 2017-2026 | most machines | `acpi:PNP0C09` `acpi:PNP0C0A` `acpi:ACPI0003` `acpi:PNP0C0D` | 4 of 4 | kernel | F | matched; battery and EC never reported from a real laptop on main |
| P3 | HID over SPI touchpads and keyboards | 2021-2026 | servers or niche | `acpi:PNP0C51` | 1 of 1 | no | B | driver exists, in no image |
| - | i8042 PS/2 keyboard and touchpad | 2017-2026 | most machines | `acpi:PNP0303` `acpi:PNP0F13` | 2 of 2 | yes | F | works on the bench laptop; _CID honoured |
