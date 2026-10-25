# Coverage: USB

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

| Priority | Part | Years | How common | Ids | Named by the driver | In Standard | Owner | Gap |
|---|---|---|---|---|---|---|---|---|
| P1 | xHCI, any vendor (Intel PCH, AMD, ASMedia, Renesas, Fresco) | 2017-2026 | most machines | `class:0c/03/30` | by class | yes | B | class match; no per-vendor quirk table (only Thunderbolt ordering) |
| P1 | USB hubs (internal laptop hubs, front-panel hubs, docks, keyboards) | 2017-2026 | most machines | `usb:09` | by class | yes | B | in Standard since 25 October; never booted behind a real hub |
| P2 | USB microphones and headsets (audio class capture) | 2017-2026 | most machines | `usb:01/02` | by class | no | B | driver exists, in no image: nothing is claimed for it in 0.9.3 |
| P2 | USB Attached SCSI sticks and SSDs | 2017-2026 | common | `usb:08/06/62` | by class | no feature | B | UAS crate exists; usb_msc is BOT only, which most UAS devices also offer |
| P2 | USB Ethernet (CDC ECM/NCM, RNDIS) and USB audio class | 2017-2026 | common | `usb:02/06` `usb:02/0d` `usb:01/01` | by class | no | B | drivers exist, in no image; held endpoint takes only HID and storage |
