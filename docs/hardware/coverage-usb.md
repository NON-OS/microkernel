# Coverage: USB

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

|---|---|---|---|---|---|---|---|---|
| P0 | USB hubs (internal laptop hubs, front-panel hubs, docks, keyboards) | 2017-2026 | most machines | `usb:class` `09` | no driver | no | B | out of Standard until sealed: nothing behind a hub enumerates |
| P1 | xHCI, any vendor (Intel PCH, AMD, ASMedia, Renesas, Fresco) | 2017-2026 | most machines | `class:0c/03/30` | by class | yes | B | class match; no per-vendor quirk table (only Thunderbolt ordering) |
| P2 | USB Attached SCSI sticks and SSDs | 2017-2026 | common | `usb:class` `08/06/62` | no driver | no feature | B | UAS crate exists; usb_msc is BOT only, which most UAS devices also offer |
| P2 | USB Ethernet (CDC ECM/NCM, RNDIS) and USB audio class | 2017-2026 | common | `usb:class` `02/06` `02/0d` `01/01` | no driver | no | B | drivers exist, in no image; held endpoint takes only HID and storage |
