# Glossary: Amnesic boot

Part of [Glossary](glossary.md).

## Amnesic boot

<a id="amnesic"></a>A boot that keeps nothing on a disk. Every boot is amnesic until first-boot setup records the choice to install in the policy store's `Persistent` field, labelled `Keep data across reboots`; until then the file store refuses every request to keep a file, and an image built with `install = false` has no setup, so it never keeps anything. Explained in [Design principles](design-principles-amnesic-by-default.md#amnesic-by-default). Code: `userland/capsule_vfs/src/server/handlers/persist_gate.rs`.

## Anyone network

The Anyone onion network, route value `ANYONE`, reached through `net.anon`,
whose circuits always have three relays. Qwen model downloads and Linux package
installs take it whatever the default network is, with two exceptions: `qwen get
--direct`, or `d` in the Marketplace, sends one model download directly, and a
package mirror at a private address is dialled directly. Explained in [Privacy
networks](../using/privacy-network.md). Code:
`userland/nonos_route_link/src/chosen.rs`,
`userland/capsule_net_anon/src/protocol/limits.rs`,
`userland/capsule_model_fetch/src/get/offer.rs`.

## Application processor

Any CPU other than the boot CPU. On x86_64 the boot CPU starts each one in the
last stage of kernel init, with INIT and two STARTUP interrupts through a
real-mode trampoline at physical 0x8000. Explained in [Scheduler and
SMP](../kernel/scheduler-and-smp.md). Code: `src/smp/init/ap_unit.rs`.

## Attestation

Evidence that what runs is what was enrolled. The loader checks the kernel's
trailer before the jump, the kernel checks the loader against the boot-root
record, and the spawn gate checks every capsule's trailer; the kernel records
each running capsule's measurement and the authority whose tree proved it, which
a holder of `AttestRead` can read. About's Proofs screen marks each part with a
tick when it holds, a cross when it is broken and a dash when it could not be
read, and never draws an unread part as a pass. Explained in [STARK
attestation](../security/stark-attestation.md#who-checks-them). Code:
`src/security/attest_registry/mod.rs`,
`userland/capsule_about/src/about/data/proofs/session.rs`,
`userland/capsule_about/src/about/ui/screens/verify_mark.rs`.

## Attestation trailer

<a id="trailer"></a>The proof a capsule, the kernel or the loader carries that its measurement fills a slot of an enrolled tree: a Merkle path and a STARK proof of the same slot, in a v4 container with the magic `NATTV4`. For a capsule the measurement is the BLAKE3 hash of its ELF, bound to its manifest's required capabilities, and an empty or refused trailer ends the spawn with `AttestationRejected`. The seal writes one for every capsule, the kernel and the loader; a development image's trailers carry the path alone. Explained in [STARK attestation](../security/stark-attestation.md#the-trailer). Code: `nonos-attest-path/src/v4/layout.rs`, `src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs`.

## Baseline

A committed list or count of the known sites of something the tree should not
have, such as lint switches, stub admissions or architecture leaks. Its check
fails when a new site appears or the count grows, so a baseline may only shrink.
Explained in [Tests and
proofs](../contributing/tests-and-proofs-static-checks.md#static-checks). Code:
`scripts/gate.py`, `nonos-ci/check-baseline.sh`.

## Boot handoff

<a id="handoff"></a>What the loader gives the kernel at the jump. On x86_64 it is `BootHandoffV1`, magic 0x4E4F4E4F, version 2, which carries the memory map, the framebuffer, the ACPI pointer, the flags, the measurements, the attestation policy and results, and a random seed; flag bit 11 asks for the installer and bits 12 to 15 carry the boot profile. The shared kernel reads it wrapped in a `KernelHandoff`, which on aarch64 is built from the device tree instead. Explained in [Boot handoff](../kernel/boot-handoff.md). Code: `src/boot/handoff/types/handoff.rs`, `src/boot/handoff/kernel_handoff/arch.rs`.

## Boot profile

<a id="boot-mode"></a>The posture chosen in the boot menu at each boot, also called the boot mode: Standard, Hardened, Safe Mode, Air-Gapped or Recovery. The loader passes it in handoff flag bits 12 to 15 and the kernel reads it as `BootProfile`, Standard when there is no handoff. Only Standard and Hardened let a network driver or service start. The other three take Network from every capsule; Safe Mode also starts no audio and neither Snake nor the Hello demo, and Recovery skips setup. It is not the build profile: it can narrow what the image was built to do, never widen it. Explained in [Boot modes](../install/boot-modes.md). Code: `src/boot/handoff/api/profile.rs`, `src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs`.

## Boot stop

The kernel's controlled stop when a boot step cannot go on: a `[FATAL]` line
naming the step on the serial console, a NONOS BOOT STOPPED band on the panel
when a framebuffer is reachable, then a halt loop on the CPU that stopped.
Explained in [Panic and boot
stop](../kernel/panic-and-boot-stop.md#a-boot-step-fails). Code:
`src/boot/stop.rs`.
