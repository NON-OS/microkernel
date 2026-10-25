# Platform: TPM 2.0

Part of [Platform](platform.md).

## TPM 2.0

The kernel has a runtime [TPM](../overview/glossary-stark-proof.md#tpm) 2.0 driver in
`src/security/tpm`, for quotes against a fresh nonce, attestation keys,
enrolment, the machine key and the device secret. The bootloader has a separate
one for measuring the boot chain, as the module comment says above `transact`
(`src/security/tpm/mod.rs:17-37`).

`detect` decides which register file is behind the window at `TPM_MMIO_BASE`,
0xFED40000, from two sources (`src/security/tpm/transport/detect.rs:29-79`,
`src/security/tpm/mmio/map.rs:25`):

- The ACPI TPM2 table's start method: 6 is `START_FIFO` (TIS), 7 and 8 are CRB
  (`src/security/tpm/transport/tpm2_start.rs:23-25`). A table whose length or
  checksum is wrong is ignored by `tpm2_table`
  (`src/security/tpm/transport/acpi.rs:30-48`).
- The part's own interface id register.

When the two disagree the TPM is refused, because the two register files
overlap. A CRB control area outside the window, as an AMD firmware TPM has, is
taken from the table. Start method 2, `START_ACPI`, is refused: its doorbell is
an ACPI `_DSM` call and the kernel has no AML interpreter
(`src/security/tpm/transport/detect.rs:35-61`). The driver uses locality 0, and
`announce` logs one `[TPM] interface` line
(`src/security/tpm/transport/detect.rs:81-92`).

Two host proof crates cover part of this driver. The key crate takes the
kernel's FIFO (TIS) protocol in by `#[path]` and drives it against a modelled
register file (`userland/tpm_key_proofs/src/security/tpm/fifo/mod.rs:17-19`).
Both `tpm_key_proofs` and `tpm_enroll_proofs` run live tests against a software
TPM, and the flake fails when a live test is skipped, through `needsTpm`
(`tools/nix/checks.nix:32-34`). At this commit they pass with 42 and 37 tests.
The CRB path and `detect` have no host test. What the TPM measures and seals is
in [Measured boot and the TPM](../security/measured-boot-and-tpm.md).
