# Glossary: Peer list

Part of [Glossary](glossary.md).

## Peer list

A kernel table that holds a named capsule to the endpoints listed for it,
whatever its capabilities admit; a capsule not on it is unaffected. In this
release it has one row: the Shield prover may send only to `shield.core`.
Explained in [IPC](../kernel/ipc.md#who-may-send-to-whom). Code:
`src/services/registry/peers.rs`.

## Policy root

The 32-byte root of the capsule attestation tree, written by the seal's
enrollment of the capsule set and compiled into the kernel. The spawn gate tries
every capsule's trailer against it first, then against any developer roots
enrolled on this machine. Explained in [STARK
attestation](../security/stark-attestation.md#three-trees). Code:
`src/security/capsule_attest/policy_root.rs`,
`src/security/capsule_attest/verify.rs`.

## Policy store

The policy service, `capsule_policy`, on port 4108: a typed key-value store of
system-wide settings such as the keyboard layout, the default network and `Keep
data across reboots`. Settings and setup write it and other capsules read it
through `nonos_policy_client`; it holds the values in memory and, on a machine
that keeps data, writes the kept ones to disk about a second after the last
change. Explained in [Settings](../using/settings.md#how-long-a-change-lasts).
Code: `userland/capsule_policy/Capsule.mk`,
`userland/capsule_policy/src/keep/tick.rs`.

## Proof crate

A host Rust crate, mostly a `userland/*_proofs` directory, that compiles
shipping kernel or capsule source through `#[path]` and tests it on the build
machine. `nix flake check` runs each one that has a `Cargo.lock` as
`proofs-<name>`: the tests in release with overflow checks on, then clippy with
warnings denied, except for the crates still listed as not lint clean. Explained
in [Tests and proofs](../contributing/tests-and-proofs-proof-crates.md#proof-crates). Code:
`tools/nix/checks.nix`.

## Publisher

Whoever holds a NONOS ID certificate and the keys it names, and signs capsule
manifests with them; a manifest's namespace must match one of the certificate's
namespace globs. A capsule under `systems.nonos` is in the enrolled tier and any
other in the publisher tier, and both must still pass the attestation trailer
check. Explained in [Signing and publisher
keys](../userland/signing-and-publisher-keys.md#publishers-outside-the-project).
Code: `src/kernel_core/process_spawn/capsule_spawn/runner/tier.rs`.

## Reply inbox

The kernel-owned inbox the spawn path registers for each capsule under the name
of its reply endpoint. Replies to that capsule's `MkIpcCall` requests arrive
there, and only one that carries the call's correlation token is delivered.
Explained in [IPC](../kernel/ipc.md#the-model). Code:
`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`,
`src/syscall/microkernel/ipc/reply_inbox.rs`.

## Rollback floor

How far the TPM NV counter at 0x01000020 has risen above its base at 0x01000021.
The loader raises it to each admitted kernel's rollback index and, in every mode
but Development, refuses a kernel whose index is below it; on a measured boot
the kernel holds the boot-root record's epoch to the same floor. Explained in
[Rollback protection](../security/rollback-protection.md#where-the-floor-lives).
Code: `src/security/tpm/boot_reads/floor.rs`.
