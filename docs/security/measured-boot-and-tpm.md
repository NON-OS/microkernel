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
