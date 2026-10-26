
hw/xhci-switch df726eeef (main 1beacdd37 merged in), built as standard-dev
in the dev copy of the tree and sealed with throwaway keys; no key under
`.keys/` was read. QEMU only, under TCG. **Not hardware-verified**: no
Intel 7, 8 or 9 series PCH was booted, and QEMU has none, so the switch
itself has run only on the host (`xhci_proofs::conformance::intel_ports_*`).
Later commits on the branch (8f5c1e6f2, a test; the hw/xhci-reply-race
merge, the proofs' reply race) change no code the image runs.

## Host

- kernel_proofs, whole crate: 1686 passed, clippy clean.
- xhci_proofs: 292 passed, three runs of three, clippy clean.
- x86_64 default and nonos-smp: 0 warnings; aarch64 (nonos-arch-preview):
  the 5 lib warnings main has, none in a file this unit touches.
- static-abi 39/39, `tools/nix/inputs.py --check` clean, rustfmt clean,
  every touched file at or under 75 lines.
- Mutants: 20 of 20 killed (`MUTANTS.md`, `mutants.py`).

## Boot 1: d/19's machine (`qemu-d19-config.txt`, `d19-config.serial.log`)

`--tpm --fresh --install-target --lane iommu=intel --smp 4 --mem 6G`: q35,
VT-d, a software TPM, the blank NVMe target, qemu-xhci with nothing on it.
`earlyprobe.py` moved the pointer from the first frame for 300 s: 99 of 99
moves landed at the target, none at the 1024x768 default
(`probe-d19-config.txt`). Setup walked by keys, then `d19-desktop.png`;
"Not now", then a click on the dock's installer icon opened the installer
(`d19-dock-installer.png`, and `app.install` spawned in the log). This is

## Boot 2: the USB path (`qemu-usb-lanes.txt`, `usb-lanes.serial.log`)

`--lane iommu=intel --lane usb-kbd --lane usb-tablet --lane xhci-stick`.
QEMU's `info usb`: the keyboard on port 1, the tablet on port 2, the stick
on port 3 at 5 Gb/s; `query-mice` had the HID tablet current. 39 of 39
probe moves exact through the tablet (`probe-usb-lanes.txt`); setup walked
by keys to the desktop (`usb-desktop.png`). The stick, blank, is asked for
by nothing and usb_msc has no Debug grant on main, so it has no line.

In both logs: no `intel port switch` line (qemu-xhci is 1b36:000d, not on
the list) and no `[EXIT]` of xhci, usb_hid, usb_msc or usb_hub. Requester
