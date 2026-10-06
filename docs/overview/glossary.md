# Glossary

Every term the NONOS pages use, defined in a few sentences, with the page that explains it in full and the code it comes from.

Terms are in alphabetical order. Where two names mean one thing, one entry carries both, and a link to either name lands on it.

## Amnesic boot

<a id="amnesic"></a>A boot that keeps nothing on a disk. Every boot is amnesic until first-boot setup records the choice to install in the policy store's `Persistent` field, labelled `Keep data across reboots`; until then the file store refuses every request to keep a file, and an image built with `install = false` has no setup, so it never keeps anything. Explained in [Design principles](design-principles.md#amnesic-by-default). Code: `userland/capsule_vfs/src/server/handlers/persist_gate.rs`.

## Anyone network

The Anyone onion network, route value `ANYONE`, which puts three relays between this machine and the site and is reached through `net.anon`. App installs, Linux package installs and Qwen model downloads take it whatever the default network is; `qwen get --direct` sends one model download directly instead. Explained in [Privacy networks](../using/privacy-network.md). Code: `userland/nonos_route_link/src/chosen.rs`, `userland/capsule_model_fetch/src/get/offer.rs`.

## Application processor

Any CPU other than the boot CPU. On x86_64 the boot CPU starts each one in the last stage of kernel init, with INIT and two STARTUP interrupts through a real-mode trampoline at physical 0x8000. Explained in [Scheduler and SMP](../kernel/scheduler-and-smp.md). Code: `src/smp/init/ap_unit.rs`.

## Attestation

Evidence that what runs is what was enrolled. The loader checks the kernel's trailer before the jump, the kernel checks the loader against the boot-root record, and the spawn gate checks every capsule's trailer; the kernel records each running capsule's measurement and the root that vouched for it, which a holder of `AttestRead` can read. About's Proofs screen shows each part as Holds, Broken or Unknown, and never draws Unknown as a pass. Explained in [STARK attestation](../security/stark-attestation.md#who-checks-them). Code: `src/security/attest_registry/mod.rs`, `userland/capsule_about/src/about/data/proofs/session.rs`.

## Attestation trailer

<a id="trailer"></a>The proof a capsule, the kernel or the loader carries that its measurement fills a slot of an enrolled tree: a Merkle path and a STARK proof of the same slot, in a v4 container with the magic `NATTV4`. For a capsule the measurement is the BLAKE3 hash of its ELF, bound to its manifest's required capabilities, and an empty or refused trailer ends the spawn with `AttestationRejected`. The seal writes one for every capsule, the kernel and the loader; a development image's trailers carry the path alone. Explained in [STARK attestation](../security/stark-attestation.md#the-trailer). Code: `nonos-attest-path/src/v4/layout.rs`, `src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs`.

## Baseline

A committed list or count of the known sites of something the tree should not have, such as lint switches, stub admissions or architecture leaks. Its check fails when a new site appears or the count grows, so a baseline may only shrink. Explained in [Tests and proofs](../contributing/tests-and-proofs.md#static-checks). Code: `scripts/gate.py`, `nonos-ci/check-baseline.sh`.

## Boot handoff

<a id="handoff"></a>What the loader gives the kernel at the jump. On x86_64 it is `BootHandoffV1`, magic 0x4E4F4E4F, version 2, which carries the memory map, the framebuffer, the ACPI pointer, the flags, the measurements, the attestation policy and results, and a random seed; flag bit 11 asks for the installer and bits 12 to 15 carry the boot profile. On aarch64 the kernel builds the same `KernelHandoff` from the device tree instead. Explained in [Boot handoff](../kernel/boot-handoff.md). Code: `src/boot/handoff/types/handoff.rs`, `src/boot/handoff/kernel_handoff/arch.rs`.
