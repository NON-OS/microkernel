# Security: The trust model

Part of [Security](README.md).

## The trust model

NONOS trusts a short chain of code. Each link starts the next, and the kernel
checks everything that runs above it.

The UEFI firmware starts the machine. NONOS cannot inspect or contain it. What
NONOS can do is bind secrets to what was measured at boot: the [machine
key](../overview/glossary-linux-personality.md#machine-key) is derived under a
[TPM](../overview/glossary-stark-proof.md#tpm) policy over `BOUND_PCRS`,
[PCR](../overview/glossary-linux-personality.md#pcr) 0, 4, 7 and 9, which the
code describes as the firmware code, the boot manager the firmware measured, the
[Secure Boot](../overview/glossary-rollback-index.md#secure-boot) policy and the
kernel hash the bootloader extends
(`src/security/tpm/machine_key/pcrs.rs:21-24`). A changed firmware, NONOS
bootloader or kernel therefore gets a different key. [Measured boot and the
TPM](measured-boot-and-tpm.md) has the details.

The NONOS bootloader loads the kernel. It stops the boot when the kernel's
signature fails or its [rollback
index](../overview/glossary-rollback-index.md#rollback-index) is below the TPM
floor, in every mode but Development, and when its STARK trailer fails, in every
mode (`verify_signature` in
`nonos-bootloader/src/boot/crypto/signature/verify.rs:25-40`, `enforce_floor` in
`nonos-bootloader/src/boot/crypto/rollback/floor.rs:28-40`, `attest_kernel` in
`nonos-bootloader/src/boot/attestation/kernel_gate.rs:34-60`). The checks that
reach those verdicts sit in the loader's verification module, which these pages
do not describe; [Boot chain and signatures](boot-chain-and-signatures.md) and
[Rollback protection](rollback-protection.md) say what the loader does with each
one. Before any program starts, the kernel checks the bootloader in turn against
the signed [boot-root
record](../overview/glossary-boot-root-record.md#boot-root-record), and stops
when that check fails (`check_bootloader` in
`src/kernel_core/init/entry/microkernel_main.rs:24-27`).

The kernel runs in ring 0 and is trusted. It admits a
[capsule](../overview/glossary-boot-root-record.md#capsule) only after
`verify_id_cert` has checked its [NONOS ID
certificate](../overview/glossary-linux-personality.md#nonos-id-certificate) and
`verify_with_publisher` has checked its
[manifest](../overview/glossary-linux-personality.md#manifest) and its image
(`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:41-66`). The
signature policy it passes, `NONOS_PRODUCTION_POLICY`, requires both an Ed25519
and an ML-DSA-65 signature (`src/security/nonos_id_cert/policy.rs:30-32`). The
[capabilities](../overview/glossary-boot-root-record.md#capability) a manifest
asks for must fit under the
[ceiling](../overview/glossary-boot-root-record.md#capability-ceiling) in that
certificate, or `check_ceiling` refuses the capsule
(`src/security/capsule_manifest/verify/caps.rs:21-29`).

Capsules run in ring 3, each in its own address space, and hold only the
capability bits their verified manifest granted. Every NONOS system call passes
through one `dispatch` function, which refuses with `EPERM` when the caller's
[capability token](../overview/glossary-boot-root-record.md#capability-token)
does not admit that call (`src/syscall/contract/dispatch.rs:31-40`). A capsule
with no bits can compute, and every call it makes except `MkTtyQuery` is
refused.

Linux programs run as guests of the [Linux
personality](../overview/glossary-linux-personality.md#linux-personality). The
kernel creates each guest with an empty capability set through `install_spawn`
(`src/process/foreign/spawn.rs:65-68`), so a guest reaches nothing except
through the capsule that answers its system calls.
