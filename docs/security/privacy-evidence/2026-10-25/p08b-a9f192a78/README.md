# P-08b: power.off has one owner, and capsule_acpi's SCI line is opened, booted

The passing boots ran at **a9f192a78**: P-08b with the SCI-ack fix (`head.txt`, image sha256 `6dc7cf75…57bdb`). The branch head 6f6af29bc adds main fbece1e19, two proof pins and inputs.json on top; the capsule and kernel code that runs is the same.

**Setup:**
- profile `p08acpi`, scratch tree only (`boot.sh.txt`): the kernel features `microkernel-full-gui` and `nonos-capsule-acpi`, with `capsule_acpi/Capsule.mk` included for the build;
- `tools/nonos-dev-image`, throwaway keys;
- QEMU 11.1.1, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`, `usb-kbd` and `usb-tablet`;
- setup walked with the D-13 keys.

## B: the shell's Power key (`B-serial.log`)

**Steps:** the Power key at 21:32:54, then the dialog (`B-dialog.png`), then Tab and Enter at 21:33:08.

**What the log shows:**
- power.off went to capsule_power, which asked capsule_acpi for OP_PREPARE_SLEEP: `[ACPI] _TTS absent … _PTS absent` (line 1783);
- `\_S5 0/0 handed to the kernel's soft-off`, then `[POWER] leave: …` (line 1784).

**How it ended:** QEMU exited by itself at 21:33:16, with code 0.

## A: QMP system_powerdown into capsule_acpi's own button (`A-serial.log`)

**What the log shows:**
- `[ACPI] power button: served by the ACPI capsule; the kernel's poll stands down`, then `served here` (lines 599-600);
- `system_powerdown` at 21:39:39;
- `[ACPI] power off with the wipe: power button`, then `\_S5 0/0 handed to the kernel's soft-off` and `[POWER] leave: …` (lines 1728-1729).

**How it ended:** QEMU exited by itself at 21:39:46, with code 0.

Neither log has a FATAL, PANIC, #PF, #GP or `panicked` line.

## The failing run before the fix: `before-8abaaef14/`

**B** passed the same way.

**A failed.** After `system_powerdown` at 21:02:36:
- no press was logged and nothing powered off (`A-after.png`, the desktop 2 min later);
- QEMU was stopped through QMP `quit` at 21:05:00.

**Why:**
- the kernel binds an INTx line masked, and only MkIrqAck opens it;
- capsule_acpi bound the SCI and acked only after an SCI had fired, which a masked line never does;
- it had already told the kernel it served the button, so the kernel's poll had stood down.

The fix (a9f192a78) acks once after a successful bind and before `take_button`. capsule_acpi_proofs pins both orders, and 3 of 3 mutants are caught: the ack dropped, the ack before the bind, and the button taken before the bind.

## Not shown

A press latched before capsule_acpi binds can't be arranged in QEMU:
- its PM1 latches PWRBTN_STS only once the kernel's arm has set PWRBTN_EN (line 141);
- from then until the hand-over (line 599), the kernel's own poll reads and clears any press.
