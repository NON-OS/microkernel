# P-05 booted at c39b8edd9: the power button works

This run is the fix for the failure recorded beside it, in `../p05-551520c30/`.

The image is standard-dev at platform/review-pwrbtn c39b8edd9 (a
fast-forward from 551520c30), built with throwaway keys; `head.txt` has its
sha256. The machine and the D-13 setup keys are the same as that run's.

## Result

1. During boot the power button armed once interrupt routing was up
   (`serial.log` 142 and 143; `routing-lines.txt` has the lines around it):
   `[ACPI] power button: SCI line masked, PM1_EN=0x0000000000000100`, then
   `[NONOS] device interrupt routing ready`.
2. Press 1, QMP `system_powerdown` at 07:51:20, brought the shell's "Power
   off and wipe memory?" with the keyboard on Cancel (`22-press1.png`).
3. Press 2 at 07:51:31, while the dialog was open, changed nothing: one
   dialog, and the machine still up 10 s later (`23-press2.png`, identical
   to the first). There was no second dialog and no power-off without
   confirmation.
4. Tab moved to Power Off (`24-focus.png`) and Enter was pressed at
   07:51:45. Then came terminate's leave phase (`serial.log` 915 to 917:
   `[POWER] leave: driver.ahci0 done`, `driver.virtio_blk0 done`,
   `driver.usb_msc0 answered -19`, no such device).
5. QEMU exited by itself at 07:51:46 (`qemu.txt`: "QEMU stopped (exit 0)").

## How the run ended

QEMU powered itself off; nothing stopped it. The script judged liveness by
whether QEMU's QMP socket answered, not by process name.

## A correction to the earlier walk

`../merge-fd40dd41f/walk/README.md` says a monitor write setting PWRBTN_EN
"did not stay set". That is true, but the method caused it, not the machine.
In HMP, `o /w` and `i /w` are 32-bit accesses. QEMU's PM1 regions take only
1 or 2 bytes, so a 32-bit write is dropped and a 32-bit read returns 0
(Platform, from `hw/acpi/core.c`). A byte write to 0x603 did not take either.
A 16-bit `o /h 0x602 0x0100` does stick. The walk's PM1a_CNT = 0x01 came from
a byte read. On this head the kernel itself sets PM1_EN to 0x0100.
