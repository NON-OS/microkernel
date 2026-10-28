# Bring-up plans

One page per reference machine: what to build, what to set in the firmware, what to do in what order, which log lines prove each part works, and what to send back. Each assumes nothing but the machine, a USB stick of 8 GB or more, and a second computer that builds the image. The parts each machine carries, and what NONOS gives them today, are in [Mainstream hardware](../mainstream.md#what-the-reference-machines-get).

| Machine | Plan |
|---|---|
| Intel laptop: ThinkPad T14 Gen 5 Intel, or Gen 4 Intel | [intel-laptop.md](intel-laptop.md) |
| AMD laptop: ThinkPad T14 Gen 4 AMD | [amd-laptop.md](amd-laptop.md) |
| Desktop board: ASRock B650M Pro RS WiFi, or ROG STRIX B760-F | [desktop-board.md](desktop-board.md) |
| Mini PC: ASUS NUC 15 Pro | [nuc-15-pro.md](nuc-15-pro.md) |

## Common to every plan

2. Write it to the stick and check the write: [Write a USB stick](../../install/usb-stick.md).
3. Firmware setup: Secure Boot off (a development image is not signed for it), boot from USB on, and the setting each plan names. Write down every setting you changed.
4. Fill in [the checklist](../real-hardware-checklist.md) as you go, then follow the plan's own steps.
5. In the Terminal, `log` writes the boot log; `log <words>` keeps the lines holding any of them. Save each file to the stick with the name the plan gives.
6. Send back, per [the kit](../real-hardware-kit.md#sending-it-back): the commit, the checklist, the files, photos of anything on screen that went wrong. Never a serial number, MAC address, network name or password; no driver line holds one, so the log files are safe to send as written.
