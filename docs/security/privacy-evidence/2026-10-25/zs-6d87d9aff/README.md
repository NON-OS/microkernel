# The zero-state check, booted at 6d87d9aff

The image is the standard profile's development twin, built at 6d87d9aff
(`tools/nonos-dev-image --profile standard`, throwaway keys), sha256
`2e8370587dd7a573…`. It was booted as a live stick under QEMU (TCG, 4 GiB)
with `--tpm --stick --lane hold-at-off --lane iommu=intel --lane usb-kbd
--lane usb-tablet`, and driven by `tools/nonos-zero-state-check` with the
steps in `tools/nonos_zero_state/steps/`.

## What the run did

1. It made a secret no image holds: `zqolaeyrfipfjjffte`.
2. It walked first-boot setup by keys: US layout, the secret as the name,
   UTC, Amnesic, no network, the defaults after, and the secret as the
   computer name. The run then reached the desktop, with its menu bar and
   dock (`20-desktop.png`), and answered the live-session notice Not now.
3. It dumped guest RAM, and the secret was there 8 times. This is the
   control: a wipe can only be judged against a secret the machine held.
4. It pressed the Power key, got "Power off and wipe memory?"
   (`30-confirm.png`), pressed Tab to Power Off, then Enter. The serial log
   shows `[POWER] leave:`, the path to `terminate`.
5. QEMU held the machine at power-off (`-action shutdown=pause`). It
   dumped guest RAM again and searched it as bytes, UTF-16 and UTF-32.
6. It compared the disk the boot ran from with the image, sector by sector,
   and searched it for the secret.
7. It booted that disk again and waited for setup (`40-setup-again.png`).

## What it found

| Check | Result |
|---|---|
| The secret in RAM before Power Off | 8 copies (required: at least one) |
| Power Off went through terminate | yes, `[POWER] leave:` on serial |
| The secret in RAM after the wipe | 0 copies |
| Disk sectors the boot changed | none |
| The secret on the disk | 0 copies |
| The secret known to the second boot | no; setup ran again from step one |

## The two runs before it

At 292696585, a walk by hand left the name 2 times and the host name 4
times after the wipe. At 6b894ed9b, the first build with frames zeroed at
teardown and the host name wiped, the check found 5 copies. Their
addresses showed two live processes' memory outside the VMAs the
per-process wipe walks: a capsule's static record, and vfs's copy of
setup's answers. 6d87d9aff adds a walk of each live address space's page
tables, which scrubs every present leaf frame but device windows.

## What it does not show

- A power cut, or holding the button down, runs no wipe.
- The parts of memory listed in
  [privacy-guarantees.md](../../../privacy-guarantees.md) as out of the
  wipe's reach were not searched for anything but this secret.
- One run, under TCG, on one virtual machine; real hardware is not covered.
