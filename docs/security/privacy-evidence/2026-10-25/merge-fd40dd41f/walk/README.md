# The wiped power-off, walked on standard-dev

The image is the standard profile's development twin built at a scratch
merge, fd40dd41f, of privacy/k-series 6c1848052 and privacy/power-off
1b9afc7ea. It was never pushed, and its sha256 is
b1a79ab1d4ea940f1203f9ed4bded6c9bc1cae6aef1a4741c2f73cf1d0e3cd4e
(`heads.txt`).

QEMU 11.1.1 with OVMF, booted with `--tpm --fresh --headless --mem 6G
--lane iommu=intel --lane usb-kbd --lane usb-tablet` (`walk-qemu.txt`).

First-boot setup was driven by keys over QMP, using the sequence from
build-and-boot's D-13 walk (): Amnesic, no
network, the name "nonos". It reached the review screen and then the
desktop (`13-review.png`, `20-desktop.png`). The serial log shows
`[SPAWN] name=power … caps=0x…218` and `[POWER] capsule spawned`.

## The shell's Power Off: booted, and the machine powered off with the wipe

1. Desktop menu, then Power Off (`41-desktop-menu.png`).
2. The confirmation "Power off and wipe memory?" appears, with the keyboard
   on Cancel (`42-confirm.png`).
3. Tab moves to Power Off (`43-focus-power-off.png`), and Enter is pressed at
   06:09:56.
4. The serial log shows terminate's leave phase: `[POWER] leave:
   driver.ahci0 done`, `driver.virtio_blk0 done`, `driver.usb_msc0 answered
   -19` (no device).
5. QEMU exits by itself at 06:09:59, 3 s after Enter (`walk-qemu.txt`:
   "QEMU stopped (exit 0)").

That is power.off answered by capsule_power, then AdminShutdown, then
zerostate's terminate, which leaves, wipes and powers off. Terminate's wipe
prints nothing to serial: its line goes to the kernel log, which the wipe
clears.

## The power button: NOT shown

QMP `system_powerdown` (the ACPI power button) was sent three times, and the
shell never showed its confirmation. `32-press-25s.png` shows the desktop
25 s after a press, unchanged. The kernel had said "[ACPI] power button:
fixed feature, polled". With SCI_EN set (0x604 read 0x01), PM1a_STS
(0x600/0x601) and PM1a_EN (0x602/0x603) read 0, and a monitor write setting
PWRBTN_EN did not stay set.

Likely cause, not verified against QEMU's source: QEMU's ICH9 sets
PWRBTN_STS only while PWRBTN_EN is set. In ACPI's fixed-feature model a
status bit latches whatever its enable bit says, which is what the kernel's
poll counts on (`src/arch/x86_64/acpi/hw/power_button.rs`). If so, the poll,
which leaves PWRBTN_EN clear, sees no press under QEMU. Not established:
whether it sees one on hardware, and why the monitor's write did not take.

The first walk's Tab and Enter, sent after a press that brought no dialog,
landed on the dock and opened the process manager (`walk-run.txt`).
