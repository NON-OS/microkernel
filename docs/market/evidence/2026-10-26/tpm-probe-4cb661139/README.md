# The TPM probe that believes an empty window no longer, booted both ways

tpm-probe 4cb661139 (an empty TPM window, reading zeros or ones, is no
TPM) booted in one qemu-dev image with b/proofs-lints and main (`head.txt`,
with the image's sha256; `build-receipt.json` the build's own), sealed with
throwaway keys in nonos-dev-image's copy; no key under `.keys/` was read.
Setup was walked by keystroke with every default.

## Without a TPM (`no-tpm-serial.log`, `no-tpm-qemu.txt`)

QEMU has no tpmdev: the window at 0xFED40000 reads zeros.

- No `[TPM] interface FIFO` line: the zero id is no longer taken for a FIFO.
- No `[MEASURE] PCR 14 extend failed`.
- `[ATTEST-DOC] refused: no tpm present`, where it said "did not respond".
- 67 `[SPAWN]`, 57 `[ZK-ATTEST] ok`, no capsule failed; `no-tpm-desktop.png`.

## With a TPM (`tpm-serial.log`, `tpm-qemu.txt`)

`--tpm`: swtpm behind tpm-crb.

- `[TPM] interface CRB at 0xFED40000, locality 0 (id 0x25C11, ACPI agrees)`.
- No `[MEASURE]` failure: PCR 14 extends.
- `[ATTEST-DOC] begin 16915` through `[ATTEST-DOC] end`: the document is out.
- 67 `[SPAWN]`, 57 `[ZK-ATTEST] ok`, no capsule failed; `tpm-desktop.png`,
  `tpm-terminal.png`.
