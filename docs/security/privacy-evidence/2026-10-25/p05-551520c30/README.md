# P-05 booted: the power button did NOT arm

The image is standard-dev at platform/review-pwrbtn 551520c30, built with
throwaway keys; `head.txt` has its sha256. It ran under QEMU 11.1.1 with
OVMF, `--tpm --fresh --headless --mem 6G --lane iommu=intel --lane usb-kbd
--lane usb-tablet` (`qemu.txt`). Setup was driven to the desktop with
build-and-boot's D-13 keys (`20-desktop.png`).

## Result

1. The kernel printed, early in boot (`serial.log` line 101, right after
   `[ACPI] sci irq=9 device=0xD`):
   `[ACPI] power button: the SCI line could not be masked, so PWRBTN_EN stays clear`.
   The expected "SCI line masked, PM1_EN=…" line never came.
2. Press 1, QMP `system_powerdown` at 07:26:06: no dialog 10 s later
   (`22-press1.png`).
3. Press 2 at 07:26:17: no dialog (`23-press2.png`). QEMU stayed up, and the
   machine did not power off unconfirmed.
4. Tab and Enter at 07:26:31 went to the desktop, since there was no dialog.
   Nothing powered off (`25-after.png`).
5. Afterwards the QEMU monitor read PM1a_STS (0x601) = 0x00, PM1a_EN (0x603)
   = 0x00 and PM1a_CNT (0x604) = 0x01 (SCI_EN set).

## How the run ended

QEMU did not exit by itself. After the readings in step 5 it was stopped
through QMP `quit`. `run.txt` says "OFF WITHOUT CONFIRMATION" and "QEMU exited
by itself", and both are wrong. The script's liveness check matched the
process name `qemu-system`, but under nix the process is named
`.qemu-system-x8`, so the check never saw QEMU running.

## So

The masking step fails on this machine (q35, intel-iommu with intremap=on,
kernel-irqchip=split), and the arming is skipped, as it should be when masking
fails. The second-press behaviour (no stacked dialog, no unconfirmed
power-off) cannot be judged until a dialog appears.
