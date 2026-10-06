# Measured boot and the TPM

NONOS uses a TPM 2.0 to record which kernel the loader admitted, to check the loader that started the kernel, to derive keys that exist only in one boot state, and to sign quotes; without a TPM every mode but Hardened and Air-Gapped still boots, and the last table on this page says what is lost.

## PCRs

| PCR | Extended by | Used by NONOS for |
|---|---|---|
| 0 | the firmware | the machine key, the device secret, quotes |
| 1, 2 | the firmware | quotes |
| 4 | the firmware, once for each UEFI application it starts | the kernel's check of its loader, the machine key, the device secret |
| 7 | the firmware, for the Secure Boot state | the machine key, the device secret, quotes |
| 9 | the loader, once, for the kernel it admitted | the machine key, the device secret's approval |

The firmware hashes every UEFI application it starts with the PE Authenticode SHA-256, extends [PCR](../overview/glossary.md#pcr) 4 with it and logs an event, as the header of the boot-measure crate says, whose `authenticode` module rebuilds that digest (`nonos-boot-measure/src/lib.rs:17-31`). The kernel reads PCR 4 from the SHA-256 bank with `pcr4` (`src/security/tpm/boot_reads/pcr4.rs:25-52`). The machine key binds `BOUND_PCRS`, PCRs 0, 4, 7 and 9 (`src/security/tpm/machine_key/pcrs.rs:21-24`). The device secret binds `APPROVED_PCRS`, PCR 9, and `MACHINE_PCRS`, PCRs 0, 4 and 7 (`src/security/tpm/device_secret/consts.rs:38-42`). A quote covers `QUOTED_PCRS`, PCRs 0, 1, 2 and 7 (`src/security/attest_doc/produce.rs:29-33`). Nothing in the kernel's TPM code extends a PCR.

## PCR 9: the admitted kernel

`attest_kernel` extends PCR 9 once, in `measure_admitted_kernel`, after the kernel's STARK trailer has passed, and only when the loader found measured boot active (`nonos-bootloader/src/boot/attestation/kernel_gate.rs:43-47`). The data is 64 bytes, the kernel's BLAKE3 hash then the kernel root the loader was built with. The loader hashes it with SHA-256 and the firmware's TCG2 extend hashes it again, so on a PCR 9 that starts at zero, as `measure_admitted_kernel` documents (`nonos-bootloader/src/boot/attestation/kernel_gate.rs:62-85`):

```text
PCR9 = SHA-256(0^32 || SHA-256(SHA-256(kernel_blake3 || kernel_attest_root)))
```

That value is the same on every machine for one release, which is what lets a release sign it. `kernel_pcr9` in the release tool computes the same value (`tools/nonos-policy-approve:73-77`). A failed extend is not fatal: the loader logs `the admitted kernel was not measured into PCR 9`, and as `measure_admitted_kernel` notes, nothing bound to PCR 9 then unseals, which fails closed. How the loader decides that measured boot is active, and the TCG2 calls it makes, are in its verification module and are not described here. Its panel shows `TPM2 MeasuredBoot active` or `TPM2 not available` (`display_subsystem_status`, `nonos-bootloader/src/boot/security/platform.rs:50-60`).

## The kernel's TPM driver

The loader's TPM stack ends with boot services. The kernel has its own, reached through `transact`, because a quote must be taken while the machine runs, against a nonce that did not exist at boot (`src/security/tpm/mod.rs:17-37`). It lives in `src/security/tpm`; the device-level view is on [Platform drivers](../drivers/platform.md).

- The register window is `TPM_MMIO_BASE`, `0xFED40000`, one 4 KiB page for each of localities 0 to 4, mapped once, uncached (`src/security/tpm/mmio/map.rs:25-49`).
- `start_method` reads the ACPI `TPM2` table, where start method 6 is FIFO and 7 and 8 are CRB, and ignores a table whose length is outside 52 to 4096 bytes or whose checksum fails (`src/security/tpm/transport/acpi.rs:22-74`).
- Start method 2 is refused, because its doorbell is an ACPI `_DSM` call and the kernel has no AML interpreter to make it; the code names it as the start method of AMD's firmware TPM (`START_ACPI`, `src/security/tpm/transport/acpi.rs:26-28`). `detect` refuses any start method other than 2, 6, 7 and 8 too (`src/security/tpm/transport/detect.rs:29-47`).
- `identify` reads the interface identity register, `INTERFACE_ID`, at offset `0x30`: type 0 is FIFO, type 1 is CRB, and a TIS 1.3 part that reads all ones counts as FIFO when its access register shows a valid part (`src/security/tpm/transport/ident.rs:28-60`).
- `detect` refuses when ACPI and the part disagree, because the two register files overlap and driving one through the other's offsets would write command bytes into control registers. With no usable ACPI table, `detect` lets the identity register decide alone, and it takes a CRB control area outside the window from the table alone (`src/security/tpm/transport/detect.rs:17-79`).
- `transact` runs detection on first use, keeps the answer for the rest of the boot, and holds a lock so one command is in flight at a time (`src/security/tpm/transport/dispatch.rs:26-49`).
- `transact_resending` sends a command again while the TPM answers `TPM_RC_RETRY`, at most three tries; on a TPM built from the reference code, the first authorization of the attestation key after each startup costs one retry (`src/security/tpm/resend.rs:17-56`).
- A missing part, a timeout and an unreadable answer are the three `TpmError` values (`src/security/tpm/error.rs:17-37`).

The serial console gets one line when the part is found, from `announce`, for example `[TPM] interface CRB at 0xFED40000, locality 0 (id ..., ACPI agrees)` (`src/security/tpm/transport/detect.rs:81-92`). The command builders and parsers are tested against byte vectors from the TPM 2.0 specification, the machine-key sequence against swtpm, and the FIFO protocol against a modelled register file, by `tpm_key_proofs` (`userland/tpm_key_proofs/Cargo.toml:4-9`). That crate passed 42 tests in this release's flake checks. The CRB and FIFO transports on a hardware TPM were not tested for this release.
