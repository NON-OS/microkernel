# Measured boot and the TPM

NONOS uses a TPM 2.0 to record which kernel the loader admitted, to check the loader that started the kernel, to derive keys that exist only in one boot state, and to sign quotes; without a TPM every mode but Hardened and Air-Gapped still boots, and the last table on this page says what is lost.
