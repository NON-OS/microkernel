# Glossary: Boot-root record

Part of [Glossary](glossary.md).

## Boot-root record

The 104-byte file `boot_root.approval` on the ESP: the bootloader tree's root
and an epoch, the release's rollback index, signed with the device policy key,
ECDSA P-256. The kernel checks its loader's slot under that root and the epoch
against the TPM's rollback floor; with no TPM log to replay, it hashes the
loader file it was handed instead and checks the epoch against a floor of 0.
Explained in [Measured boot and the
TPM](../security/measured-boot-and-tpm.md#the-kernel-checks-its-loader). Code:
`nonos-boot-measure/src/record/mod.rs`,
`nonos-boot-measure/src/gate/verdict.rs`.

## Build profile

<a id="profile"></a>The kind of image `profile` in `nonos.toml` selects: standard, hardened, airgapped, qemu, dev or core. It fixes the kernel features, what is taken out of the binary (capsule output to the serial console for hardened, that and every network feature for airgapped) and the weakest loader policy allowed, so it decides what an image can ever do. The hardened and airgapped build profiles are not the Hardened and Air-Gapped boot entries, which are boot profiles chosen in the boot menu at each boot. Explained in [Profiles](../build/profiles.md). Code: `tools/nix/config.nix`.

## Capability

One named right, one bit of a 64-bit word. The kernel defines 36, from
`CoreExec` at bit 0 to `DeviceSecret` at bit 35, and checks the caller's bits
before it runs a system call; `IO` and `Hardware` enforce nothing. Explained in
[Capabilities](../kernel/capabilities-what-each-bit.md#what-each-bit-admits),
with every bit in [Capabilities ABI](../abi/capabilities.md). Code:
`src/capabilities/types/defs.rs`, `abi/caps.toml`.

## Capability ceiling

The most capability bits something may hold. A publisher's NONOS ID certificate
carries one, and the spawn gate refuses a manifest whose required or optional
bits go past it. Each build also writes an image-wide ceiling into the kernel,
but in this release a capsule above it only logs `[CEILING] not enforced` and
starts anyway. Explained in
[Profiles](../build/profiles.md#the-image-capability-ceiling). Code:
`src/security/capsule_manifest/verify/caps.rs`,
`src/security/image_ceiling/admits.rs`.

## Capability token

The kernel's record of a process's capabilities, bound to its pid, its address
space, a nonce drawn for this boot and a revocation epoch, and sealed with a
64-byte MAC made of two keyed BLAKE3 hashes that only the kernel can make. Every
system call the kernel knows resolves the caller's token first, and a bad MAC, a
broken binding, a revoked token or a missing capability is refused with EPERM.
Explained in [Capabilities](../kernel/capabilities-the-token.md#the-token).
Code: `src/capabilities/token/types/defs.rs`,
`src/syscall/contract/resolver/resolve.rs`.

## Capability word

The 64-bit mask of capability bits a process holds. At spawn it is the
manifest's required bits plus the optional bits the spawn site grants, with
Network removed on a boot profile without network; a Linux guest gets 0.
Explained in [Manifests and
capabilities](../userland/manifests-and-capabilities-how-the-word.md#how-the-word-is-fixed).
Code: `src/security/capsule_manifest/verify/caps_bits.rs`.

## Capsule

A signed ring 3 program, the form in which NONOS runs its own software outside
the kernel; a Linux guest is not a capsule. It ships as four files, declared
once in a `Capsule.mk`: its ELF, its NONOS ID certificate, its signed manifest
and its attestation trailer. The kernel starts it as its own process only after
the spawn gate verifies all four, with its own address space, its `proc.<pid>`
and `stdin.<pid>` inboxes, and the capability word its manifest and its spawn
site allow. Explained in [Userland](../userland/README.md#what-a-capsule-is).
Code: `nonos-mk/capsule.mk`,
`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs`.
