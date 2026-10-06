# Troubleshooting

The messages NONOS shows when a boot, setup or an install stops, what each one means, and what to do about it.

## Collect what NONOS says

Before you change anything, keep what the machine told you:

- The screen. A refusal from the loader stays up for 30 seconds or until you press a key, then the machine restarts (`nonos-bootloader/src/display/boot/refusal/hold.rs`). Write down its title and the text under `THE LOADER'S REASON` (`nonos-bootloader/src/display/boot/refusal/draw.rs`).
- The kernel log, from the Terminal, once the desktop is up. `log` shows the newest 200 lines, `log` with words shows only the lines that name one of them, and `log >` with a file name keeps them (`userland/capsule_terminal/src/command/builtin/log.rs`). The kernel keeps the first 64 KiB of the boot and the last 64 KiB after it (`src/sys/serial/tail.rs`).
- The serial console, if the machine has a serial port. Under QEMU it is the terminal `make boot` runs in, and with `--headless` the boot tool writes it to `target/qemu/serial.log` (`tools/nonos_qemu/__main__.py`).

```
log
log rtl wifi
log > boot-log.txt
```

Not tested in this release.

The kernel keeps that log only on images built with capsule serial output, such as `standard` and `qemu`. A `hardened` or `airgapped` image keeps nothing, and `log` prints `log: no line matches`. If `log` prints `log: the kernel would not hand over its log (this terminal needs AttestRead)`, this Terminal was not granted the log.

To report a machine, follow [Reporting a machine](../hardware/report.md).

## The boot menu says REFUSED HERE

Under each entry the menu says whether this machine meets what that entry needs (`nonos-bootloader/src/bootmenu/ready.rs`). After `REFUSED HERE: NO` it names what is missing:

| Name | What to do |
|---|---|
| `CRYPTO SELF-TEST` | the loader's own BLAKE3 or Ed25519 gave a wrong answer; write the stick again |
| `SIGNING KEYS` | this loader carries no key to check the kernel with; use a sealed image |
| `HARDWARE RNG` | turn on RDRAND or the TPM in the firmware; a virtual machine needs virtio-rng |
| `SECURE BOOT`, `PK`, `DB` | Hardened, or any entry on a `hardened` or `airgapped` image: turn Secure Boot on, with its keys enrolled, or pick Standard on a `standard` image |
| `TPM 2.0` | Hardened, or any entry on a `hardened` or `airgapped` image: turn the TPM on in the firmware, or pick Standard on a `standard` image |

Air-Gapped also needs a TPM, though the menu does not list it there ([Boot modes](boot-modes.md)).
