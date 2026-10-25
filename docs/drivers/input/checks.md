# Input: how it is checked

The proof crates that run the input drivers' shipping source on the host, the
QEMU target, and what real hardware has shown. Which driver handles which
device is in [Input drivers](README.md).

## How this is checked

The [proof crates](../../overview/glossary-peer-list.md#proof-crate) compile the shipping
driver source on the host and run it against models of the hardware. Their flake
checks pass at commit bff12b97:

| Proof crate | What it holds | Tests |
|---|---|---|
| `input_proofs` | PS/2 packets, touchpad decode, gestures, layout tables, the router's routing | 96 |
| `ps2_input_proofs` | i8042 presence, bring-up sequence, keyboard and mouse setup | 38 |
| `i2c_hid_proofs` | touchpad bind, wake, report descriptor, input mode | 51 |
| `i2c_pci_proofs` | LPSS reset, clocks, id tables, doorbell | 35 |
| `i2c_transfer_proofs` | the DesignWare transfer engine against a modelled bus | 26 |
| `pinctrl_proofs` | GPIO controller ids, pin resolution, pad register arithmetic | 8 |
| `setup_layout_proofs` | the screen layouts of setup and the installer, and, in 4 of its tests, that setup writes the chosen keyboard layout when the step is confirmed | 56 |

Since then `ps2_input_proofs` has grown to 59 tests (the key framing, the lock
LEDs, the bring-up of a mouse that reset), `i2c_pci_proofs` to 47,
`i2c_transfer_proofs` to 50 and `i2c_hid_proofs` to 52 (the reach of a client
and its narrowing, the sweep for a pad with no declared address, waits on a busy
machine).

The check for `input_proofs` runs its tests without clippy, which the flake
allows for that crate (`lintNone`, `tools/nix/checks.nix:64`).

The QEMU run target gives the guest its keyboard and mouse through the q35
machine's i8042, and attaches xHCI on its own (`QEMU_USB`,
`mk/10-qemu.mk:99-102`).

For the PS/2 keyboard with its layouts, the I2C-HID touchpad on Intel LPSS, the
power button and the volume keys there is one real-hardware report. Works on an
x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October
2026; the image commit was not recorded.

## See also

- [Input drivers](README.md)
- [Input: layouts and limits](layouts.md)
- [PS/2 keyboard and mouse](ps2.md)
- [I2C-HID touchpads](i2c-hid.md)
