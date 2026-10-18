# Glossary: STARK proof

Part of [Glossary](glossary.md).

## STARK proof

A proof, checked by the `nox_verify` verifier from the STARKs repository the
flake pins, that a measurement and its context fill a slot of an enrolled tree.
Every attestation trailer carries one beside its Merkle path, except a
development image's, and a gate admits only when both the path and the proof
pass. Explained in [STARK
attestation](../security/stark-attestation.md#the-statement). Code:
`src/security/capsule_attest/path.rs`.

## Syscall tag

Four ASCII letters packed little-endian into the 64-bit system call number by
`tag4`, first letter in the lowest byte. `MkIpcSend` is `MISD`, so its number is
0x4453494D. Explained in [Syscalls](../abi/syscalls-numbers.md#numbers). Code:
`src/syscall/abi/tag.rs`.

## TLB shootdown

The round in which a CPU that changed a page table makes every other CPU that
may cache the old translation drop it and answer. A round still unanswered after
50 ms is sent again as an NMI, and after 2000 ms the machine stops. Explained in
[Scheduler and
SMP](../kernel/scheduler-and-smp.md#shootdowns-and-stopping-the-other-cpus).
Code: `src/memory/paging/manager/shootdown/request.rs`,
`src/memory/paging/manager/shootdown/slow.rs`.

## TPM

The TPM 2.0 chip. NONOS keeps its rollback floor there, derives machine keys and
the device secret from it without storing them, and reads PCR 4 from it to check
the loader. Hardened and Air-Gapped boots refuse to start without one, and a
data volume keyed by the TPM stays closed without it. Explained in [Measured
boot and the TPM](../security/measured-boot-and-tpm.md). Code:
`src/security/tpm/mod.rs`, `nonos-bootloader/src/menu/types/mode.rs`.

## Trust anchor

The hybrid key pair, one Ed25519 and one ML-DSA-65 key, that signs every NONOS
ID certificate. The kernel compiles in the trust-anchor policy built from its
public halves, and a tree without that policy file does not build. Explained in
[Signing and publisher
keys](../userland/signing-and-publisher-keys.md#the-keys). Code:
`src/security/nonos_trust_anchor/baked.rs`.

## Trust set

The public files the seal writes under `nonos-data/trust`: each capsule's NONOS
ID certificate, signed manifest and STARK trailer, the attestation roots and
policies, and the kernel's public keys. The kernel embeds the capsule files and
the loader compiles in the kernel's keys; for a tree that lacks a capsule's
files, the flake builds no kernel and writes a note naming the capsule.
Explained in [Build](../build/README.md). Code: `tools/nix/image.nix`,
`tools/nix/artifacts.nix`.

## ZeroState

The wipe `terminate` runs before every shutdown and restart. It stops the other
CPUs and the claimed devices, wipes DMA buffers, process memory, kernel stacks,
file system caches, the key vault, the RAM log and the kernel heap, then hands
the machine to the firmware. A kernel panic, a forced power-off or a power cut
skips it, and apart from the RAM log it does not reach kernel statics outside
the heap, the data volume key among them, or a live stick's in-memory volume. In
0.9.2 only the installer's restart runs it, since the desktop cannot shut down.
Explained in [Device secrets and
keys](../security/device-secrets-and-keys.md#wiped-at-shutdown-and-reboot).
Code: `src/security/zerostate/terminate.rs`,
`src/security/hardening/memory_sanitization/api.rs`.
