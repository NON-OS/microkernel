# Randomness and cryptography

Use this page to trace where NONOS gets its random bytes, how the kernel stretches and reseeds them, and which code does which cryptography.

## Sources

On x86_64, the loader, the kernel and two capsules read four sources of randomness.

| Source | Read by |
|---|---|
| RDRAND | the `entropy_pool` capsule, the keyring capsule, the kernel generator, the loader, the `CryptoRandom` fallback |
| RDSEED | the keyring capsule, the kernel generator, the loader, the `CryptoRandom` fallback |
| the `entropy_pool` capsule's bytes | both generators' seeds, mixed with RDSEED and RDRAND and never counted as enough alone |
| the loader's 32-byte seed | the kernel generator, once per boot |

The [TPM](../overview/glossary-stark-proof.md#tpm) is not on the list. The kernel asks it for the [machine key](../overview/glossary-linux-personality.md#machine-key), a key derived under a policy over [PCRs](../overview/glossary-linux-personality.md#pcr) 0, 4, 7 and 9, and never for random bytes (`BOUND_PCRS`, `src/security/tpm/machine_key/pcrs.rs:21-24`).

`random_u64` reads RDRAND and `entropy_u64` reads RDSEED, and each answers `None` when the CPU lacks the instruction or stays busy through its attempts (`src/arch/cpu_random/read.rs:29-57`). On x86_64 that is 10 attempts for RDRAND and 100 for RDSEED (`RDRAND_ATTEMPTS`, `src/arch/x86_64/cpu_random/read.rs:24-25`). On aarch64 the same two calls read RNDR and RNDRRS, and on riscv64 `random_u64` always answers `None` (`src/arch/cpu_random/read.rs:29-41`).

The kernel drives no RNG device of its own. `init_entropy` logs `Entropy: CPU RNG` while the platform comes up, or `Entropy: no CPU RNG` when the CPU has neither instruction (`src/kernel_core/init/platform/entropy.rs:19-27`). The `capsule_driver_virtio_rng` [driver capsule](../overview/glossary-dma-pool.md#driver-capsule) claims a virtio-rng device, but no kernel code calls its client, `fill_random` (`src/hardware/virtio_rng_capsule/client/fill_random.rs:27`).

The loader collects the kernel's first seed. `collect_hw_rng_bytes` folds up to `HW_RNG_ITERATIONS`, 32, words into 64 bytes with cycle-counter jitter, each word from RDSEED or, where RDSEED fails, from RDRAND (`nonos-bootloader/src/entropy/sources.rs:45-88`, `nonos-bootloader/src/entropy/types.rs:23-25`). `collect_boot_entropy` hashes those bytes with BLAKE3, together with `TSC_JITTER_ROUNDS`, 256, rounds of timer jitter and one more cycle-counter reading, and `collect_boot_entropy_64_with_st` hashes that result again with the real-time clock (`nonos-bootloader/src/entropy/collector.rs:29-105`). `collect_entropy` keeps the first 32 of the 64 output bytes for the [boot handoff](../overview/glossary-amnesic-boot.md#boot-handoff) (`nonos-bootloader/src/boot/prepare/entropy.rs:25-39`). The boot screen names the CPU source it found, as `Entropy source RDSEED, TSC jitter, RTC`, `Entropy source RDRAND, TSC jitter, RTC` or `Entropy source TSC jitter, RTC` (`entropy_source`, `nonos-bootloader/src/boot/prepare/display.rs:33-41`). The UEFI RNG protocol is read only by a loader built with the `efi-rng` feature (`collect_efi_rng`, `nonos-bootloader/src/entropy/collector.rs:107-118`), and no build in the tree turns that feature on.

## Parts

- [The two generators](randomness-generators.md)
- [When a source is missing; the HARDWARE RNG line](randomness-when-missing.md)
- [The two services; who may call them; the crypto system calls](randomness-services.md)
- [Which algorithm is used where; post-quantum code; trust; host tests](randomness-algorithms.md)

## When a source is missing

On [its own page](randomness-when-missing.md#when-a-source-is-missing).

## Who may call them

On [the services page](randomness-services.md#who-may-call-them).

## The crypto system calls

On [the services page](randomness-services.md#the-crypto-system-calls).

## See also

- [Security](README.md)
- [Device secrets and keys](device-secrets-and-keys.md)
- [Boot chain and signatures](boot-chain-and-signatures.md)
- [Measured boot and the TPM](measured-boot-and-tpm.md)
- [Capsule isolation](capsule-isolation.md)
- [TLS and certificate trust](tls-and-certificates.md)
- [Protections and limits](protections-and-limits.md)
- [Syscalls ABI](../abi/syscalls.md)
- [Capabilities ABI](../abi/capabilities.md)
- [IPC](../kernel/ipc.md)
- [IPC services](../userland/ipc-services.md)
- [Boot modes](../install/boot-modes.md)
