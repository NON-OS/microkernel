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

## The loader stops

A loader refusal shows `BOOT REFUSED`, a title, then three numbered parts: `WHAT HAPPENED`, `WHAT TO DO` and `THE LOADER'S REASON` (`nonos-bootloader/src/display/boot/refusal/draw.rs`). The title comes from the start of the reason (`advice` in `nonos-bootloader/src/display/boot/refusal/advice.rs`). The titles this release shows:

| Title | Meaning | What to do |
|---|---|---|
| No kernel on this disk | no `EFI/nonos/kernel.bin` on the EFI partition | boot the stick you made, or write it again |
| The kernel is not signed | the kernel carries no release signature | boot a sealed image |
| The kernel signature does not match | Ed25519 and ML-DSA-65 did not both verify | write the stick again from one release: loader and kernel must match |
| The kernel's STARK attestation failed | the proof did not verify against the root this loader enrolled | use the loader and kernel of one release together |
| The kernel has no STARK attestation | this loader boots only an enrolled kernel | the same |
| The kernel could not be read | its program headers did not load | the media may be damaged; write it again |
| This kernel is older than allowed | its rollback index is below this machine's TPM floor; the reason reads `Rollback: tpm floor` and the two numbers | boot a release at least as new as the last one used here ([Update](update.md)) |
| This machine lacks what the entry requires | the entry's policy check failed; the reason reads `Security policy enforcement failed`, and the menu's `REFUSED HERE` line names what is missing | pick an entry the menu shows as ready, or turn on Secure Boot, the TPM and RDRAND |
| Not enough randomness | the hardware gave too little randomness for the boot's keys | turn on RDRAND or the TPM |
| Boot stopped | any other reason; the reason itself says what stopped | keep the reason and report it |

Three reasons, as the screen gives them:

- `This CPU has no NX bit` or `This CPU addresses under 36 bits of physical memory`, under `Boot stopped`. The processor lacks a feature NONOS needs. If the firmware can turn NX off, check that it is on (`nonos-bootloader/src/boot/security/hardware.rs`).
- `Air-Gapped needs a TPM: its rollback floor keeps an older signed kernel from booting`, under `Boot stopped`, and the same for Hardened. Turn the TPM on, or pick another entry (`nonos-bootloader/src/boot/crypto/rollback/floor.rs`).
- `Rollback floor could not be raised to index` and a number, on Hardened and Air-Gapped. The TPM did not take the new floor. Because the reason starts with `Rollback`, the screen's title reads `This kernel is older than allowed`, though the kernel was not refused for its age. On other entries the loader only warns `Rollback floor not raised: an older kernel may still boot` (`nonos-bootloader/src/boot/crypto/rollback/raise.rs`).

The loader also has titles for each missing Secure Boot key, the TPM and the random source (`POLICY` in `nonos-bootloader/src/display/boot/refusal/policy.rs`), but in this release every policy refusal reaches the screen as `Security policy enforcement failed` (`nonos-bootloader/src/boot/security/policy.rs`), so those titles are not shown.

If the firmware itself will not start the stick, and no NONOS screen appears at all, one cause is Secure Boot turned on without the NONOS db certificate enrolled: the seal signs the loader with that key alone (`tools/nonos_seal/chain.py`). Turn Secure Boot off and boot Standard ([Requirements](requirements.md#secure-boot-and-the-tpm)). Not tested in this release.
