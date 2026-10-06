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

## Data volume

The encrypted volume the disk plan places at sector 262,144 or above, where kept files such as Qwen models live, each sector sealed with ChaCha20-Poly1305. On an installed disk its key is derived from the TPM under the label `blockfs.data.v1`, or is a random key sealed under a passphrase when the key header says so; on a live stick the volume is held in RAM under a key drawn for that boot and is gone at power off. Explained in [Device secrets and keys](../security/device-secrets-and-keys.md#the-data-volume). Code: `src/fs/blockfs_volume/open_machine.rs`, `src/fs/blockfs_volume/session.rs`.

## Development image

A profile's development twin, `<profile>-dev`: the same kernel features, path-only attestation and the `dev-qemu` loader policy, sealed with throwaway keys by `make dev-image` in a copy of the checkout under `target/dev/tree`. Its gates take a capsule on its Merkle path without the STARK proof, and the seal refuses it for a release. Explained in [The seal](../build/seal.md#without-the-release-keys). Code: `tools/nonos-dev-image`.

## Device secret

Four field words the TPM derives as the witness of the anonymous device proof, under a policy the release approves over PCR 9 and this machine's PCRs 0, 4 and 7. It is never stored, and a firmware or loader change gives a new one. In this release only `app.prove` holds the `DeviceSecret` capability that receives it. Explained in [Device secrets and keys](../security/device-secrets-and-keys.md#the-device-secret). Code: `src/security/tpm/device_secret/mod.rs`.

## Directmap

The kernel's linear, never executable mapping of the first 256 GiB of physical memory at 0xFFFF_8000_0000_0000, PML4 slot 256. The bootloader builds it, and the kernel reaches page tables, user frames and the handoff through it. Explained in [Memory and paging](../kernel/memory-and-paging.md). Code: `src/memory/layout/constants/regions.rs`.

## Disk plan

One plain sector at LBA 245,760, with the magic `NONOSDP1`, that names the data volume's range and up to 30 files to import. The kernel checks every range in it against the disk and against each other before it uses any; a live stick's plan names no volume. Explained in [Storage drivers](../drivers/storage/README.md#the-disk-layout). Code: `src/fs/blockfs_volume/plan_types.rs`.

## DMA pool

Memory the hardware broker reserves for DMA buffers: a low pool below 4 GiB for devices that can only name 32-bit addresses, and a high pool for display surfaces. A run of pages goes back only to the pool it came from. Explained in [The hardware broker](../kernel/hardware-broker.md#dma-buffers). Code: `src/hardware/broker/dma/pool/mod.rs`.

## Driver capsule

A capsule, built from a `userland/capsule_driver_*` crate, that drives one kind of device from ring 3. It reaches its device only through grants from the hardware broker and serves it to other capsules over IPC. The tree has 27 driver crates, and 18 of them are built into images in this release. Explained in [Writing a driver](../drivers/writing-a-driver.md). Code: `mk/20-build.mk`, `userland/capsule_driver_virtio_rng/Capsule.mk`.

## Endpoint

A named IPC address with a port. A capsule's manifest declares its service endpoint, which others send to, and its reply endpoint; the kernel's service registry records each with the pid that serves it and the capability bits a sender must hold. Explained in [IPC](../kernel/ipc.md#the-model). Code: `src/services/registry/endpoint.rs`, `src/security/capsule_manifest/schema/endpoint.rs`.

## Enrollment

Committing a set of measurements, such as the BLAKE3 hash of every capsule's ELF, to one Merkle policy tree, then writing its root and a trailer for each member. The seal runs one enrollment for the capsule set and one each for the kernel and the loader; each draws a fresh pad seed, so the same members give a new root every time. Explained in [STARK attestation](../security/stark-attestation.md#who-makes-the-trailers). Code: `nonos-stark-enroll/src/commands.rs`.

## ESP

The EFI system partition, a FAT volume. It holds the loader as `EFI/BOOT/BOOTX64.EFI` and, under `EFI/nonos`, the signed `kernel.bin`, the loader's trailer, the boot-root record, the kernel approval when there is one, and `boot.cfg`. Explained in [Install to disk](../install/install-to-disk.md#what-is-written). Code: `tools/nonos_seal/media.py`.

## File store

The `vfs_pool` service, `capsule_vfs`. It holds every file the desktop and its apps see in its own memory, serves holders of FileSystem, and writes a file to the package store only when asked to keep it on a boot that keeps data. Explained in [Files](../using/files.md). Code: `userland/capsule_vfs/Capsule.mk`, `userland/capsule_vfs/src/server/handlers/persist_gate.rs`.

## Foreign process

<a id="guest"></a>A process that runs code the kernel has not verified, on x86_64 only; the Linux pages call it a guest. The Linux personality creates one with `MkForeignSpawn`, which needs ForeignExec, and it starts with a capability word of 0. A system call number the kernel does not know is parked for its supervising capsule to answer. Explained in [The Linux personality](../userland/linux-personality.md). Code: `src/process/foreign/spawn.rs`, `src/process/foreign/trap.rs`.

## Grant

One revocable piece of a claimed device that the hardware broker hands the claiming process: an MMIO window, a DMA buffer, an interrupt binding or, on x86_64, a port range. Each has a grant id, and the broker revokes it on unmap, on device release and when the process exits. Explained in [The hardware broker](../kernel/hardware-broker.md#revocation). Code: `src/hardware/broker/grant.rs`, `src/process/exit/finalize.rs`.

## Hardware broker

<a id="broker"></a>The ring 0 code through which a driver capsule lists devices, claims one and receives grants on it: MMIO windows, DMA buffers, interrupt bindings and, on x86_64 only, port I/O, each behind its own capability. When a remapping unit covers a claimed PCI device, the broker moves it into the capsule's IOMMU domain before powering it, and every grant a process holds is released when it exits. Explained in [The hardware broker](../kernel/hardware-broker.md), with the calls in [Broker ABI](../abi/broker.md). Code: `src/hardware/broker/mod.rs`, `src/hardware/broker/claim/claim.rs`.

## Held endpoint

A service endpoint that only the services named for it may send to, whatever capabilities a sender holds. Fifteen driver endpoints are held this way: a wired network driver takes sends from `net.core` and `net.l2`, a Wi-Fi driver from `net.core`, Settings and setup, and the keyboard, USB HID, I2C-HID, USB storage and random-source drivers from no capsule at all, because the kernel drives them itself. Explained in [IPC](../kernel/ipc.md#who-may-send-to-whom). Code: `src/services/registry/held_table.rs`.

## Identity domain

The VT-d domain every device found by the boot PCI scan starts in. It maps physical memory one to one up to the top of the managed range rounded up to 1 GiB, and never less than 4 GiB; a device the scan did not find has no entry and is denied. Explained in [IOMMU](../kernel/iommu.md#when-vt-d-comes-into-service). Code: `src/arch/x86_64/iommu/unit/bringup/domain.rs`, `src/arch/x86_64/iommu/unit/bringup/limit.rs`.

## Inbox

A named, bounded message queue in the kernel into which IPC messages are delivered, such as `proc.<pid>`, where a capsule's requests arrive, and `stdin.<pid>`, which its parent feeds. An inbox holds 1024 messages by default and at most 16 MiB. Explained in [IPC](../kernel/ipc.md#limits). Code: `src/ipc/nonos_inbox/inbox.rs`, `src/ipc/nonos_inbox/budget.rs`.

## IOMMU

The DMA remapping unit, which limits the memory a device can reach. This kernel drives Intel VT-d; its AMD-Vi backend sits behind a feature no build profile turns on, so a device that no unit in service covers, on an AMD-Vi machine among others, goes ahead unconfined and the boot log says so. Explained in [IOMMU](../kernel/iommu.md). Code: `src/hardware/broker/confine/posture.rs`.

## IOMMU domain

A set of I/O page tables the remapping unit applies to the devices attached to it. The hardware broker gives each driver capsule one domain, shared by every PCI device it claims, which maps only the DMA buffers granted to it, so the device faults on everything else. ACPI and platform devices get no domain. Explained in [IOMMU](../kernel/iommu.md#per-capsule-domains). Code: `src/hardware/broker/confine/attach.rs`.

## Kernel mirror

The kernel module that carries one capsule, named by `CAPSULE_KERNEL_MIRROR` in its `Capsule.mk`, such as `src/hardware/virtio_rng_capsule` or `src/userspace/capsule_linux`. It embeds the capsule's four files with `include_bytes!` and spawns it through the spawn gate with the capabilities it offers. Explained in [Writing a driver](../drivers/writing-a-driver.md#11-the-kernel-mirror). Code: `nonos-mk/capsule.mk`, `src/hardware/virtio_rng_capsule/embed.rs`.

## Linux personality

The capsule `capsule_linux`, `app.linux`, which runs unmodified x86_64 Linux programs as guests and answers the Linux system calls the kernel hands it. It is the only capsule that holds ForeignExec, and it runs a program from the store only when the proof kept beside it verifies. Explained in [The Linux personality](../userland/linux-personality.md). Code: `userland/capsule_linux/Capsule.mk`.

## Loader policy

The bootloader policy an image is built with, weakest first: `dev-qemu`, `standard-qemu`, `standard` or `production`. Each build profile sets the weakest it allows, `production` for hardened and airgapped, and `dev-qemu`, which compiles in the development override, is never sealed for release. Explained in [Profiles](../build/profiles.md#the-floors). Code: `tools/nix/config.nix`.

## Machine key

A 32-byte key the TPM derives on request as an HMAC over a label, under a primary key whose policy binds PCRs 0, 4, 7 and 9. Nothing is stored: one machine in one boot state gets the same key every time, and a firmware, Secure Boot, loader or kernel change gives another. The data volume key and the key that seals saved Wi-Fi networks are machine keys, and a capsule holding Crypto asks for one with `CryptoMachineKey`. Explained in [Device secrets and keys](../security/device-secrets-and-keys.md#the-machine-key). Code: `src/security/tpm/machine_key/mod.rs`, `src/security/tpm/machine_key/pcrs.rs`.

## Manifest

<a id="capsule-manifest"></a>The publisher-signed binary record, schema version 3, that describes one capsule: its certificate id, namespace, version, target triple, the BLAKE3 hash of its ELF, its required and optional capabilities, up to 16 endpoints and up to 4 publisher signatures. `verify_with_publisher` checks it at every spawn, and a grant outside it is refused with `GrantOutsideManifest`. Explained in [Manifests and capabilities](../userland/manifests-and-capabilities.md). Code: `src/security/capsule_manifest/schema/manifest.rs`, `src/security/capsule_manifest/verify/mod.rs`.

## NONOS ID certificate

A publisher's certificate, signed by the trust anchor with both Ed25519 and ML-DSA-65. It binds the publisher's NONOS ID to its public keys, the namespaces it may publish under, its capability ceiling and the trust-anchor epoch it was issued under. Explained in [Signing and publisher keys](../userland/signing-and-publisher-keys.md). Code: `src/security/nonos_id_cert/schema/cert.rs`, `src/security/nonos_id_cert/policy.rs`.

## Nym mixnet

The Nym anonymity network, route value `NYM`, and the default network for the browser, the Terminal and the wallet. A program reaches it through `net.socks5`, and when the policy store does not answer, the route is read as Nym, never as Direct. Explained in [Privacy networks](../using/privacy-network.md). Code: `userland/policy_proto/src/route.rs`, `userland/nonos_route_link/src/chosen.rs`.

## Package store

<a id="store"></a>The region of a NONOS disk from sector 256 up to the disk plan at sector 245,760, headed by the magic `NONOSTR1`. The seal fills it with what `tools/nix/store.json` declares, the Linux userland, demo capsules, sample films and wallpapers among them, and the file store reads it and adds to it: installed apps, kept files and setup's answers. It is written unencrypted. Explained in [Storage drivers](../drivers/storage/README.md#the-disk-layout). Code: `userland/nonos_disk_map/src/places.rs`.

## PCR

A TPM platform configuration register. The firmware extends PCR 4 with each UEFI application it starts, the loader extends PCR 9 once for the admitted kernel, NONOS binds its machine keys to PCRs 0, 4, 7 and 9, and a quote covers PCRs 0, 1, 2 and 7. Explained in [Measured boot and the TPM](../security/measured-boot-and-tpm.md#pcrs). Code: `src/security/tpm/machine_key/pcrs.rs`, `src/security/tpm/boot_reads/pcr4.rs`.

## Peer list

A kernel table that holds a named capsule to the endpoints listed for it, whatever its capabilities admit; a capsule not on it is unaffected. In this release it has one row: the Shield prover may send only to `shield.core`. Explained in [IPC](../kernel/ipc.md#who-may-send-to-whom). Code: `src/services/registry/peers.rs`.

## Policy root

The 32-byte root of the capsule attestation tree, written by the seal's enrollment of the capsule set and compiled into the kernel. The spawn gate tries every capsule's trailer against it first, then against any signing roots enrolled on this machine. Explained in [STARK attestation](../security/stark-attestation.md#three-trees). Code: `src/security/capsule_attest/policy_root.rs`, `src/security/capsule_attest/verify.rs`.
