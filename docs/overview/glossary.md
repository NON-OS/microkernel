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

## Boot profile

<a id="boot-mode"></a>The posture chosen in the boot menu at each boot, also called the boot mode: Standard, Hardened, Safe Mode, Air-Gapped or Recovery. The loader passes it in the handoff flags and the kernel reads it as `BootProfile`, Standard when there is no handoff; only Standard and Hardened let a network driver or service start, the others take Network from every capsule, Safe Mode also starts no audio and no optional app, and Recovery skips setup. It narrows what the image's build profile allows and never widens it. Explained in [Boot modes](../install/boot-modes.md). Code: `src/boot/handoff/api/profile.rs`, `src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs`.

## Boot stop

The kernel's controlled stop when a boot step cannot go on: a `[FATAL]` line naming the step on the serial console, a NONOS BOOT STOPPED band on the panel when a framebuffer is reachable, then a halt loop on the CPU that stopped. Explained in [Panic and boot stop](../kernel/panic-and-boot-stop.md#a-boot-step-fails). Code: `src/boot/stop.rs`.

## Boot-root record

The 104-byte file `boot_root.approval` on the ESP: the bootloader tree's root and an epoch, signed with the device policy key, ECDSA P-256. The kernel holds the loader's measurement, taken from the firmware's PCR 4 log, to that root and, on a measured boot, the epoch to the rollback floor. Explained in [Measured boot and the TPM](../security/measured-boot-and-tpm.md#the-kernel-checks-its-loader). Code: `nonos-boot-measure/src/record/mod.rs`.

## Build profile

<a id="profile"></a>The kind of image `profile` in `nonos.toml` selects: standard, hardened, airgapped, qemu, dev or core. It fixes the kernel features, what is taken out of the binary (serial debug output for hardened, that and every network feature for airgapped) and the weakest loader policy allowed, so it decides what an image can ever do. The boot profile is a separate choice, made in the boot menu at each boot. Explained in [Profiles](../build/profiles.md). Code: `tools/nix/config.nix`.

## Capability

One named right, one bit of a 64-bit word. The kernel defines 36, from `CoreExec` at bit 0 to `DeviceSecret` at bit 35, and checks the caller's bits before it runs a system call; `IO` and `Hardware` enforce nothing. Explained in [Capabilities](../kernel/capabilities.md#what-each-bit-admits), with every bit in [Capabilities ABI](../abi/capabilities.md). Code: `src/capabilities/types/defs.rs`, `abi/caps.toml`.

## Capability ceiling

The most capability bits something may hold. A publisher's NONOS ID certificate carries one, and the spawn gate refuses a manifest whose required or optional bits go past it. Each build also writes an image-wide ceiling into the kernel, but in this release a capsule above it only logs `[CEILING] not enforced` and starts anyway. Explained in [Profiles](../build/profiles.md#the-image-capability-ceiling). Code: `src/security/capsule_manifest/verify/caps.rs`, `src/security/image_ceiling/admits.rs`.

## Capability token

The kernel's record of a process's capabilities, bound to its pid, its address space, a nonce drawn for this boot and a revocation epoch, and sealed with a 64-byte MAC made of two keyed BLAKE3 hashes that only the kernel can make. Every system call the kernel knows resolves the caller's token first, and a bad MAC, a broken binding, a revoked token or a missing capability is refused with EPERM. Explained in [Capabilities](../kernel/capabilities.md#the-token). Code: `src/capabilities/token/types/defs.rs`, `src/syscall/contract/resolver/resolve.rs`.

## Capability word

The 64-bit mask of capability bits a process holds. At spawn it is the manifest's required bits plus the optional bits the spawn site grants, with Network removed on a boot profile without network; a Linux guest gets 0. Explained in [Manifests and capabilities](../userland/manifests-and-capabilities.md#how-the-word-is-fixed). Code: `src/security/capsule_manifest/verify/caps_bits.rs`.

## Capsule

A signed ring 3 program, the form in which NONOS runs everything outside the kernel. It ships as four files, declared once in a `Capsule.mk`: its ELF, its NONOS ID certificate, its signed manifest and its attestation trailer. The kernel starts it as its own process only after the spawn gate verifies all four, with its own address space, its `proc.<pid>` and `stdin.<pid>` inboxes, and the capability word its manifest allows. Explained in [Userland](../userland/README.md#what-a-capsule-is). Code: `nonos-mk/capsule.mk`, `src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs`.

## Claim epoch

The number `MkDeviceClaim` returns, taken from one counter that starts at 1 and grows with every claim. Every later MMIO, DMA, interrupt, port and PCI call on that device must pass it back, and a call with an old one fails with ESTALE, -116. Explained in [Broker ABI](../abi/broker.md). Code: `src/hardware/broker/claim/state.rs`.

## Correlation token

The nonzero number the kernel gives each `MkIpcCall` from a counter. Only a reply that carries the same number is delivered to the caller; a plain send carries 0, so it cannot pass as a reply. Explained in [IPC](../kernel/ipc.md#blocking-waking-and-timeouts). Code: `src/syscall/microkernel/ipc/call/sys_ipc_call.rs`.
