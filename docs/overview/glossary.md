# Glossary

Look up a term the NONOS pages link to: each is defined in a few sentences, with the page that explains it in full and the code it comes from.

Terms are in alphabetical order. Where two names mean one thing, one entry carries both, and a link to either name lands on it.

## Amnesic boot

<a id="amnesic"></a>A boot that keeps nothing on a disk. Every boot is amnesic until first-boot setup records the choice to install in the policy store's `Persistent` field, labelled `Keep data across reboots`; until then the file store refuses every request to keep a file, and an image built with `install = false` has no setup, so it never keeps anything. Explained in [Design principles](design-principles.md#amnesic-by-default). Code: `userland/capsule_vfs/src/server/handlers/persist_gate.rs`.

## Anyone network

The Anyone onion network, route value `ANYONE`, reached through `net.anon`, whose circuits always have three relays. Qwen model downloads and Linux package installs take it whatever the default network is, with two exceptions: `qwen get --direct`, or `d` in the Marketplace, sends one model download directly, and a package mirror at a private address is dialled directly. Explained in [Privacy networks](../using/privacy-network.md). Code: `userland/nonos_route_link/src/chosen.rs`, `userland/capsule_net_anon/src/protocol/limits.rs`, `userland/capsule_model_fetch/src/get/offer.rs`.

## Application processor

Any CPU other than the boot CPU. On x86_64 the boot CPU starts each one in the last stage of kernel init, with INIT and two STARTUP interrupts through a real-mode trampoline at physical 0x8000. Explained in [Scheduler and SMP](../kernel/scheduler-and-smp.md). Code: `src/smp/init/ap_unit.rs`.

## Attestation

Evidence that what runs is what was enrolled. The loader checks the kernel's trailer before the jump, the kernel checks the loader against the boot-root record, and the spawn gate checks every capsule's trailer; the kernel records each running capsule's measurement and the authority whose tree proved it, which a holder of `AttestRead` can read. About's Proofs screen marks each part with a tick when it holds, a cross when it is broken and a dash when it could not be read, and never draws an unread part as a pass. Explained in [STARK attestation](../security/stark-attestation.md#who-checks-them). Code: `src/security/attest_registry/mod.rs`, `userland/capsule_about/src/about/data/proofs/session.rs`, `userland/capsule_about/src/about/ui/screens/verify_mark.rs`.

## Attestation trailer

<a id="trailer"></a>The proof a capsule, the kernel or the loader carries that its measurement fills a slot of an enrolled tree: a Merkle path and a STARK proof of the same slot, in a v4 container with the magic `NATTV4`. For a capsule the measurement is the BLAKE3 hash of its ELF, bound to its manifest's required capabilities, and an empty or refused trailer ends the spawn with `AttestationRejected`. The seal writes one for every capsule, the kernel and the loader; a development image's trailers carry the path alone. Explained in [STARK attestation](../security/stark-attestation.md#the-trailer). Code: `nonos-attest-path/src/v4/layout.rs`, `src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs`.

## Baseline

A committed list or count of the known sites of something the tree should not have, such as lint switches, stub admissions or architecture leaks. Its check fails when a new site appears or the count grows, so a baseline may only shrink. Explained in [Tests and proofs](../contributing/tests-and-proofs.md#static-checks). Code: `scripts/gate.py`, `nonos-ci/check-baseline.sh`.

## Boot handoff

<a id="handoff"></a>What the loader gives the kernel at the jump. On x86_64 it is `BootHandoffV1`, magic 0x4E4F4E4F, version 2, which carries the memory map, the framebuffer, the ACPI pointer, the flags, the measurements, the attestation policy and results, and a random seed; flag bit 11 asks for the installer and bits 12 to 15 carry the boot profile. The shared kernel reads it wrapped in a `KernelHandoff`, which on aarch64 is built from the device tree instead. Explained in [Boot handoff](../kernel/boot-handoff.md). Code: `src/boot/handoff/types/handoff.rs`, `src/boot/handoff/kernel_handoff/arch.rs`.

## Boot profile

<a id="boot-mode"></a>The posture chosen in the boot menu at each boot, also called the boot mode: Standard, Hardened, Safe Mode, Air-Gapped or Recovery. The loader passes it in handoff flag bits 12 to 15 and the kernel reads it as `BootProfile`, Standard when there is no handoff. Only Standard and Hardened let a network driver or service start. The other three take Network from every capsule; Safe Mode also starts no audio and neither Snake nor the Hello demo, and Recovery skips setup. It is not the build profile: it can narrow what the image was built to do, never widen it. Explained in [Boot modes](../install/boot-modes.md). Code: `src/boot/handoff/api/profile.rs`, `src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs`.

## Boot stop

The kernel's controlled stop when a boot step cannot go on: a `[FATAL]` line naming the step on the serial console, a NONOS BOOT STOPPED band on the panel when a framebuffer is reachable, then a halt loop on the CPU that stopped. Explained in [Panic and boot stop](../kernel/panic-and-boot-stop.md#a-boot-step-fails). Code: `src/boot/stop.rs`.

## Boot-root record

The 104-byte file `boot_root.approval` on the ESP: the bootloader tree's root and an epoch, the release's rollback index, signed with the device policy key, ECDSA P-256. The kernel checks its loader's slot under that root and the epoch against the TPM's rollback floor; with no TPM log to replay, it hashes the loader file it was handed instead and checks the epoch against a floor of 0. Explained in [Measured boot and the TPM](../security/measured-boot-and-tpm.md#the-kernel-checks-its-loader). Code: `nonos-boot-measure/src/record/mod.rs`, `nonos-boot-measure/src/gate/verdict.rs`.

## Build profile

<a id="profile"></a>The kind of image `profile` in `nonos.toml` selects: standard, hardened, airgapped, qemu, dev or core. It fixes the kernel features, what is taken out of the binary (capsule output to the serial console for hardened, that and every network feature for airgapped) and the weakest loader policy allowed, so it decides what an image can ever do. The hardened and airgapped build profiles are not the Hardened and Air-Gapped boot entries, which are boot profiles chosen in the boot menu at each boot. Explained in [Profiles](../build/profiles.md). Code: `tools/nix/config.nix`.

## Capability

One named right, one bit of a 64-bit word. The kernel defines 36, from `CoreExec` at bit 0 to `DeviceSecret` at bit 35, and checks the caller's bits before it runs a system call; `IO` and `Hardware` enforce nothing. Explained in [Capabilities](../kernel/capabilities.md#what-each-bit-admits), with every bit in [Capabilities ABI](../abi/capabilities.md). Code: `src/capabilities/types/defs.rs`, `abi/caps.toml`.

## Capability ceiling

The most capability bits something may hold. A publisher's NONOS ID certificate carries one, and the spawn gate refuses a manifest whose required or optional bits go past it. Each build also writes an image-wide ceiling into the kernel, but in this release a capsule above it only logs `[CEILING] not enforced` and starts anyway. Explained in [Profiles](../build/profiles.md#the-image-capability-ceiling). Code: `src/security/capsule_manifest/verify/caps.rs`, `src/security/image_ceiling/admits.rs`.

## Capability token

The kernel's record of a process's capabilities, bound to its pid, its address space, a nonce drawn for this boot and a revocation epoch, and sealed with a 64-byte MAC made of two keyed BLAKE3 hashes that only the kernel can make. Every system call the kernel knows resolves the caller's token first, and a bad MAC, a broken binding, a revoked token or a missing capability is refused with EPERM. Explained in [Capabilities](../kernel/capabilities.md#the-token). Code: `src/capabilities/token/types/defs.rs`, `src/syscall/contract/resolver/resolve.rs`.

## Capability word

The 64-bit mask of capability bits a process holds. At spawn it is the manifest's required bits plus the optional bits the spawn site grants, with Network removed on a boot profile without network; a Linux guest gets 0. Explained in [Manifests and capabilities](../userland/manifests-and-capabilities.md#how-the-word-is-fixed). Code: `src/security/capsule_manifest/verify/caps_bits.rs`.

## Capsule

A signed ring 3 program, the form in which NONOS runs its own software outside the kernel; a Linux guest is not a capsule. It ships as four files, declared once in a `Capsule.mk`: its ELF, its NONOS ID certificate, its signed manifest and its attestation trailer. The kernel starts it as its own process only after the spawn gate verifies all four, with its own address space, its `proc.<pid>` and `stdin.<pid>` inboxes, and the capability word its manifest and its spawn site allow. Explained in [Userland](../userland/README.md#what-a-capsule-is). Code: `nonos-mk/capsule.mk`, `src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs`.

## Claim epoch

The number `MkDeviceClaim` returns, taken from one counter that starts at 1 and grows with every claim. The calls that map MMIO, bind an interrupt, take a DMA buffer or a port grant, or read or write PCI configuration on that device must pass it back, and a call with an old one fails with ESTALE, -116. Explained in [Broker ABI](../abi/broker.md). Code: `src/hardware/broker/claim/state.rs`.

## Correlation token

The nonzero number the kernel gives each `MkIpcCall` from a counter. Only a reply that carries the same number is delivered to the caller; a plain send carries 0, so it cannot pass as a reply. Explained in [IPC](../kernel/ipc.md#blocking-waking-and-timeouts). Code: `src/syscall/microkernel/ipc/call/sys_ipc_call.rs`.

## Data volume

The encrypted volume the disk plan places at sector 262,144 or above, which holds imported files such as the Qwen models; each 512-byte sector is sealed on its own with ChaCha20-Poly1305. On an installed disk its key is derived from the TPM under the label `blockfs.data.v1`; the kernel can also key it with a passphrase, but no capsule asks for that in this release. On a live stick the volume is held in RAM under a key drawn for that boot and is gone at power off. Explained in [Device secrets and keys](../security/device-secrets-and-keys.md#the-data-volume). Code: `src/fs/blockfs_volume/open_machine.rs`, `src/fs/blockfs_volume/session.rs`.

## Development image

A profile's development twin, `<profile>-dev`: the same kernel features, path-only attestation and the `dev-qemu` loader policy, sealed with throwaway keys by `make dev-image` in a copy of the checkout under `target/dev/tree`. Its gates take a capsule on its Merkle path without the STARK proof, and the seal refuses it for a release. Explained in [The seal](../build/seal.md#without-the-release-keys). Code: `tools/nonos-dev-image`.

## Device secret

Four field words the TPM derives as the witness of the anonymous device proof, under a policy the release approves over PCR 9 and this machine's PCRs 0, 4 and 7. It is never stored; a firmware or loader change gives a new one, while a kernel update the release approved keeps it. In this release only `app.prove` holds the `DeviceSecret` capability that receives it. Explained in [Device secrets and keys](../security/device-secrets-and-keys.md#the-device-secret). Code: `src/security/tpm/device_secret/mod.rs`.

## Directmap

The kernel's linear, never executable mapping of the first 256 GiB of physical memory at 0xFFFF_8000_0000_0000, PML4 slot 256. The bootloader builds it, and the kernel reaches page tables, user frames and the handoff through it. Explained in [Memory and paging](../kernel/memory-and-paging.md). Code: `src/memory/layout/constants/regions.rs`.

## Disk plan

One plain sector at LBA 245,760, with the magic `NONOSDP1`, that names the data volume's range and up to 30 files to import. The kernel checks every range in it against the disk and against each other before it uses any; a live stick's plan names no volume. Explained in [Storage drivers](../drivers/storage/README.md#the-disk-layout). Code: `src/fs/blockfs_volume/plan_types.rs`.

## DMA pool

Memory the hardware broker reserves for DMA buffers: a low pool below 4 GiB for devices that can only name 32-bit addresses, and a high pool for display surfaces. A run of pages goes back only to the pool it came from. Explained in [The hardware broker](../kernel/hardware-broker.md#dma-buffers). Code: `src/hardware/broker/dma/pool/mod.rs`.

## Driver capsule

A capsule, built from a `userland/capsule_driver_*` crate, that drives one kind of device from ring 3. It reaches its device only through grants from the hardware broker and serves it over IPC, to the kernel or to other capsules. The tree has 27 driver crates; 18 of them are built in this release, and the other nine are in no image. Explained in [Writing a driver](../drivers/writing-a-driver.md). Code: `mk/20-build.mk`, `userland/capsule_driver_virtio_rng/Capsule.mk`.

## Endpoint

A named IPC address with a port. A capsule's manifest declares its service endpoint, which others send to, and its reply endpoint; the kernel's service registry records each with the pid that serves it and the capability bits a sender must hold. Explained in [IPC](../kernel/ipc.md#the-model). Code: `src/services/registry/endpoint.rs`, `src/security/capsule_manifest/schema/endpoint.rs`.

## Enrollment

Committing a set of measurements, such as the BLAKE3 hash of every capsule's ELF, to one Merkle policy tree, then writing its root and a trailer for each member. The seal runs one enrollment for the capsule set and one each for the kernel and the loader, and keeps the capsule enrollment already in the tree when no capsule source changed. Each enrollment draws a fresh pad seed, so enrolling the same members again gives a new root. Explained in [STARK attestation](../security/stark-attestation.md#who-makes-the-trailers). Code: `nonos-stark-enroll/src/commands.rs`, `tools/nonos_seal/__main__.py`.

## ESP

The EFI system partition, a FAT volume. It holds the loader as `EFI/BOOT/BOOTX64.EFI` and, under `EFI/nonos`, the signed `kernel.bin`, the loader's trailer, the boot-root record, the kernel approval when there is one, and `boot.cfg`. Explained in [Install to disk](../install/install-to-disk.md#what-is-written). Code: `tools/nonos_seal/media.py`.

## File store

The `vfs_pool` service, `capsule_vfs`. It holds every file the desktop and its apps see in its own memory, serves holders of FileSystem, and writes a file to the package store only when asked to keep it on a boot that keeps data. Explained in [Files](../using/files.md). Code: `userland/capsule_vfs/Capsule.mk`, `userland/capsule_vfs/src/server/handlers/persist_gate.rs`.

## Foreign process

<a id="guest"></a>A process that runs code the kernel has not verified, on x86_64 only; the Linux pages call it a guest. The Linux personality creates one with `MkForeignSpawn`, which needs ForeignExec, and it starts with a capability word of 0. A system call number the kernel does not know is parked for its supervising capsule to answer. Explained in [The Linux personality](../userland/linux-personality.md). Code: `src/process/foreign/spawn.rs`, `src/process/foreign/trap.rs`.

## Grant

One revocable piece of a claimed device that the hardware broker hands the claiming process: an MMIO window, a DMA buffer, an interrupt binding or, on x86_64, a port range. Each has a grant id, and the broker revokes it on unmap, on device release and when the process exits. Explained in [The hardware broker](../kernel/hardware-broker.md#revocation). Code: `src/hardware/broker/grant.rs`, `src/process/exit/finalize.rs`.

## Hardware broker

<a id="broker"></a>The ring 0 code through which a driver capsule lists devices, claims one and receives grants on it: MMIO windows, DMA buffers, interrupt bindings and, on x86_64 only, port I/O, each behind its own capability. When a VT-d unit in service covers a claimed PCI device, the broker moves it into the capsule's IOMMU domain before powering it, and every grant a process holds is released when it exits. Explained in [The hardware broker](../kernel/hardware-broker.md), with the calls in [Broker ABI](../abi/broker.md). Code: `src/hardware/broker/mod.rs`, `src/hardware/broker/claim/claim.rs`.

## Held endpoint

A service endpoint that only the services named for it may send to, whatever capabilities a sender holds. The `HELD` table lists fifteen driver endpoints; among them, a wired network driver takes sends only from `net.core` and `net.l2`, a Wi-Fi driver from `net.core`, Settings and setup, the xHCI controller from the USB HID and USB storage drivers, and the keyboard, USB HID, I2C-HID, USB storage and random-source drivers from no capsule at all, because the kernel drives them itself. Explained in [IPC](../kernel/ipc.md#who-may-send-to-whom). Code: `src/services/registry/held_table.rs`.

## Identity domain

The VT-d domain every device found by the boot PCI scan starts in, and stays in until a driver claims it. It maps physical memory one to one up to the top of the managed range rounded up to 1 GiB, and never less than 4 GiB; a device the scan did not find has no entry and is denied. Explained in [IOMMU](../kernel/iommu.md#when-vt-d-comes-into-service). Code: `src/arch/x86_64/iommu/unit/bringup/domain.rs`, `src/arch/x86_64/iommu/unit/bringup/limit.rs`.

## Inbox

A named, bounded message queue in the kernel into which IPC messages are delivered, such as `proc.<pid>`, where a capsule's requests arrive, and `stdin.<pid>`, which its parent feeds. An inbox holds 1024 messages by default and at most 16 MiB. Explained in [IPC](../kernel/ipc.md#limits). Code: `src/ipc/nonos_inbox/inbox.rs`, `src/ipc/nonos_inbox/budget.rs`.

## IOMMU

The DMA remapping unit, which limits the memory a device can reach. On Intel VT-d machines every image turns it on at boot and confines each claimed PCI device to its driver capsule's IOMMU domain. AMD-Vi is not driven and interrupt remapping is off in every build profile, so on an AMD machine device DMA is unrestricted; a claim on a device no unit in service covers goes ahead unconfined, and the boot log says so. Explained in [IOMMU](../kernel/iommu.md). Code: `src/hardware/broker/confine/posture.rs`.

## IOMMU domain

A set of I/O page tables the remapping unit applies to the devices attached to it. The hardware broker gives each driver capsule one domain, shared by every claimed PCI device that a unit in service covers, which maps only the DMA buffers granted to the capsule, so the device faults on everything else. ACPI and platform devices get no domain. Explained in [IOMMU](../kernel/iommu.md#per-capsule-domains). Code: `src/hardware/broker/confine/attach.rs`.

## Kernel mirror

The kernel module that carries one capsule, named by `CAPSULE_KERNEL_MIRROR` in its `Capsule.mk`, such as `src/hardware/virtio_rng_capsule` or `src/userspace/capsule_linux`. It embeds the capsule's four files with `include_bytes!` and spawns it through the spawn gate with the capabilities it offers. Explained in [Writing a driver](../drivers/writing-a-driver.md#11-the-kernel-mirror). Code: `nonos-mk/capsule.mk`, `src/hardware/virtio_rng_capsule/embed.rs`.

## Linux personality

The capsule `capsule_linux`, `app.linux`, which runs unmodified x86_64 Linux programs as guests and answers the Linux system calls the kernel hands it. It is the only capsule that holds ForeignExec, and it runs a program from the store only when the proof kept beside it verifies. Explained in [The Linux personality](../userland/linux-personality.md). Code: `userland/capsule_linux/Capsule.mk`.

## Loader policy

The bootloader policy an image is built with, weakest first: `dev-qemu`, `standard-qemu`, `standard` or `production`. Each build profile sets the weakest it allows, `production` for hardened and airgapped, and `dev-qemu`, which compiles in the development override, is never sealed for release. Explained in [Profiles](../build/profiles.md#the-floors). Code: `tools/nix/config.nix`.

## Machine key

A 32-byte key the TPM derives on request as an HMAC over a label, under a primary key whose policy binds PCRs 0, 4, 7 and 9. Nothing is stored: one machine in one boot state gets the same key every time, and a firmware, Secure Boot, loader or kernel change gives another. The data volume key and the key that seals saved Wi-Fi networks are machine keys; a capsule holding Crypto asks for one with `CryptoMachineKey`, under any label except the kernel's own, which start with a zero byte. Explained in [Device secrets and keys](../security/device-secrets-and-keys.md#the-machine-key). Code: `src/security/tpm/machine_key/mod.rs`, `src/security/tpm/machine_key/pcrs.rs`, `src/security/tpm/machine_key/kernel_label.rs`.

## Manifest

<a id="capsule-manifest"></a>The publisher-signed binary record, schema version 3, that describes one capsule: its certificate id, namespace, version, target triple, the BLAKE3 hash of its ELF, its required and optional capabilities, up to 16 endpoints and up to 4 publisher signatures. `verify_with_publisher` checks it at every spawn, and a grant outside it is refused with `GrantOutsideManifest`. Explained in [Manifests and capabilities](../userland/manifests-and-capabilities.md). Code: `src/security/capsule_manifest/schema/manifest.rs`, `src/security/capsule_manifest/verify/mod.rs`.

## NONOS ID certificate

A publisher's certificate, signed by the trust anchor with both Ed25519 and ML-DSA-65. It binds the publisher's NONOS ID to its public keys, the namespaces it may publish under, its capability ceiling and the trust-anchor epoch it was issued under. Explained in [Signing and publisher keys](../userland/signing-and-publisher-keys.md). Code: `src/security/nonos_id_cert/schema/cert.rs`, `src/security/nonos_id_cert/policy.rs`.

## Nym mixnet

The Nym anonymity network, route value `NYM`, and the default network for the browser, the Terminal and the wallet. A program reaches it through `net.socks5`, and when the policy store does not answer, the route is read as Nym, never as Direct. Explained in [Privacy networks](../using/privacy-network.md). Code: `userland/policy_proto/src/route.rs`, `userland/nonos_route_link/src/chosen.rs`.

## Package store

<a id="store"></a>The region of a NONOS disk from sector 256 up to the disk plan at sector 245,760, headed by the magic `NONOSTR1`. The seal fills it with what `tools/nix/store.json` declares, the Linux userland, demo capsules, sample films and wallpapers among them; on a boot that keeps data, the file store adds each file a program asks it to keep, setup's answers and kept settings among them. It is written unencrypted, and a Linux package install is not added to it but held in memory until restart. Explained in [Storage drivers](../drivers/storage/README.md#the-disk-layout). Code: `userland/nonos_disk_map/src/places.rs`, `userland/capsule_vfs/src/server/handlers/store_persist.rs`, `userland/capsule_linux/src/linux/install/place.rs`.

## PCR

A TPM platform configuration register. The firmware extends PCR 4 with each UEFI application it starts, the loader extends PCR 9 once for the admitted kernel, NONOS binds its machine keys to PCRs 0, 4, 7 and 9, and a quote covers PCRs 0, 1, 2 and 7. Explained in [Measured boot and the TPM](../security/measured-boot-and-tpm.md#pcrs). Code: `src/security/tpm/machine_key/pcrs.rs`, `src/security/tpm/boot_reads/pcr4.rs`.

## Peer list

A kernel table that holds a named capsule to the endpoints listed for it, whatever its capabilities admit; a capsule not on it is unaffected. In this release it has one row: the Shield prover may send only to `shield.core`. Explained in [IPC](../kernel/ipc.md#who-may-send-to-whom). Code: `src/services/registry/peers.rs`.

## Policy root

The 32-byte root of the capsule attestation tree, written by the seal's enrollment of the capsule set and compiled into the kernel. The spawn gate tries every capsule's trailer against it first, then against any developer roots enrolled on this machine. Explained in [STARK attestation](../security/stark-attestation.md#three-trees). Code: `src/security/capsule_attest/policy_root.rs`, `src/security/capsule_attest/verify.rs`.

## Policy store

The policy service, `capsule_policy`, on port 4108: a typed key-value store of system-wide settings such as the keyboard layout, the default network and `Keep data across reboots`. Settings and setup write it and other capsules read it through `nonos_policy_client`; it holds the values in memory and, on a machine that keeps data, writes the kept ones to disk about a second after the last change. Explained in [Settings](../using/settings.md#how-long-a-change-lasts). Code: `userland/capsule_policy/Capsule.mk`, `userland/capsule_policy/src/keep/tick.rs`.

## Proof crate

A host Rust crate, mostly a `userland/*_proofs` directory, that compiles shipping kernel or capsule source through `#[path]` and tests it on the build machine. `nix flake check` runs each one that has a `Cargo.lock` as `proofs-<name>`: the tests in release with overflow checks on, then clippy with warnings denied, except for the crates still listed as not lint clean. Explained in [Tests and proofs](../contributing/tests-and-proofs.md#proof-crates). Code: `tools/nix/checks.nix`.

## Publisher

Whoever holds a NONOS ID certificate and the keys it names, and signs capsule manifests with them; a manifest's namespace must match one of the certificate's namespace globs. A capsule under `systems.nonos` is in the enrolled tier and any other in the publisher tier, and both must still pass the attestation trailer check. Explained in [Signing and publisher keys](../userland/signing-and-publisher-keys.md#publishers-outside-the-project). Code: `src/kernel_core/process_spawn/capsule_spawn/runner/tier.rs`.

## Reply inbox

The kernel-owned inbox the spawn path registers for each capsule under the name of its reply endpoint. Replies to that capsule's `MkIpcCall` requests arrive there, and only one that carries the call's correlation token is delivered. Explained in [IPC](../kernel/ipc.md#the-model). Code: `src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`, `src/syscall/microkernel/ipc/reply_inbox.rs`.

## Rollback floor

How far the TPM NV counter at 0x01000020 has risen above its base at 0x01000021. The loader raises it to each admitted kernel's rollback index and, in every mode but Development, refuses a kernel whose index is below it; on a measured boot the kernel holds the boot-root record's epoch to the same floor. Explained in [Rollback protection](../security/rollback-protection.md#where-the-floor-lives). Code: `src/security/tpm/boot_reads/floor.rs`.

## Rollback index

The anti-rollback number signed into the kernel image together with its BLAKE3 hash, set by `rollback_index` in `nonos.toml`, 1 by default; the flake refuses a value below 1. Raising it for a release retires every older kernel on each machine with a TPM where the new one boots. Explained in [Rollback protection](../security/rollback-protection.md#the-kernels-rollback-index). Code: `nonos-bootloader/tools/sign-kernel/src/message.rs`, `tools/nix/config.nix`, `nonos.toml`.

## Seal

The step between the reproducible build and a bootable image, run as `make seal` or `nix run .#seal`. In six phases it signs the market inputs, gives every capsule its certificate, manifest and trailer, enrolls the kernel and the loader, writes the signed image with its ESP, store, USB image and ISO, and checks what it wrote. It compiles nothing itself, and the build never holds a key. Explained in [The seal](../build/seal.md). Code: `tools/nonos_seal/__init__.py`.

## Secure Boot

The UEFI firmware feature that starts only a loader whose signature the firmware's signature database, db, trusts. The Hardened boot entry needs it on, with a platform key and a db; the hardened and airgapped build profiles, built with the `production` loader policy, are described in `tools/nix/config.nix` as refusing to start without Secure Boot and a TPM. When the seal has the NONOS db key it signs `BOOTX64.EFI` with that key alone, and a release seal of a `production` loader stops without it. Explained in [Requirements](../install/requirements.md#secure-boot-and-the-tpm). Code: `nonos-bootloader/src/bootmenu/ready.rs`, `tools/nix/config.nix`, `tools/nonos_seal/chain.py`.

## Serial console

The kernel's main log, a UART that takes its tagged lines: on x86_64 the 16550 at I/O port 0x3F8, set to 115200 8N1, and on aarch64 the PL011. With no UART present the output is dropped and the boot goes on; a capsule may write to it only with the Debug capability, which a spawn grants only in an image built with `capsule-serial-debug`. Explained in [Logging](../kernel/logging.md#the-serial-console). Code: `src/arch/x86_64/console.rs`, `src/capabilities/serial_debug.rs`.

## SMP

Symmetric multiprocessing: the kernel runs on every core, not only the boot CPU. It starts every CPU the firmware enables in the ACPI MADT, up to 256, and every one of them takes processes from one run queue shared by the whole machine. On an Intel hybrid part an idle performance core is offered new work before an efficiency core. Every build profile turns it on through the `nonos-smp` feature unless `nonos.toml` sets `smp = false`, and a kernel built without that feature runs on the boot CPU alone. A Linux program is still told it has one CPU. Explained in [Scheduler and SMP](../kernel/scheduler-and-smp.md). Code: `src/smp/init/ap_start.rs`, `src/process/scheduler/dispatch/run_queue.rs`, `src/smp/ipi_handler.rs`.

## Spawn gate

The kernel path every capsule passes before it becomes a process, `spawn_verified_as`. It checks, in order, that the boot profile allows the capsule, its NONOS ID certificate, its manifest (namespace, capability ceiling, signatures, payload hash, target, endpoints and the grant), then its attestation trailer, and refuses the spawn at the first failure. Explained in [Processes and capsule spawn](../kernel/processes-and-spawn.md#the-spawn-gate). Code: `src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs`, `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs`.

## STARK proof

A proof, checked by the `nox_verify` verifier from the STARKs repository the flake pins, that a measurement and its context fill a slot of an enrolled tree. Every attestation trailer carries one beside its Merkle path, except a development image's, and a gate admits only when both the path and the proof pass. Explained in [STARK attestation](../security/stark-attestation.md#the-statement). Code: `src/security/capsule_attest/path.rs`.

## Syscall tag

Four ASCII letters packed little-endian into the 64-bit system call number by `tag4`, first letter in the lowest byte. `MkIpcSend` is `MISD`, so its number is 0x4453494D. Explained in [Syscalls](../abi/syscalls.md#numbers). Code: `src/syscall/abi/tag.rs`.

## TLB shootdown

The round in which a CPU that changed a page table makes every other CPU that may cache the old translation drop it and answer. A round still unanswered after 50 ms is sent again as an NMI, and after 2000 ms the machine stops. Explained in [Scheduler and SMP](../kernel/scheduler-and-smp.md#shootdowns-and-stopping-the-other-cpus). Code: `src/memory/paging/manager/shootdown/request.rs`, `src/memory/paging/manager/shootdown/slow.rs`.

## TPM

The TPM 2.0 chip. NONOS keeps its rollback floor there, derives machine keys and the device secret from it without storing them, and reads PCR 4 from it to check the loader. Hardened and Air-Gapped boots refuse to start without one, and a data volume keyed by the TPM stays closed without it. Explained in [Measured boot and the TPM](../security/measured-boot-and-tpm.md). Code: `src/security/tpm/mod.rs`, `nonos-bootloader/src/menu/types/mode.rs`.

## Trust anchor

The hybrid key pair, one Ed25519 and one ML-DSA-65 key, that signs every NONOS ID certificate. The kernel compiles in the trust-anchor policy built from its public halves, and a tree without that policy file does not build. Explained in [Signing and publisher keys](../userland/signing-and-publisher-keys.md#the-keys). Code: `src/security/nonos_trust_anchor/baked.rs`.

## Trust set

The public files the seal writes under `nonos-data/trust`: each capsule's NONOS ID certificate, signed manifest and STARK trailer, the attestation roots and policies, and the kernel's public keys. The kernel embeds the capsule files and the loader compiles in the kernel's keys; for a tree that lacks a capsule's files, the flake builds no kernel and writes a note naming the capsule. Explained in [Build](../build/README.md). Code: `tools/nix/image.nix`, `tools/nix/artifacts.nix`.

## ZeroState

The wipe `terminate` runs before every shutdown and restart. It stops the other CPUs and the claimed devices, wipes DMA buffers, process memory, kernel stacks, file system caches, the key vault, the RAM log and the kernel heap, then hands the machine to the firmware. A kernel panic, a forced power-off or a power cut skips it, and apart from the RAM log it does not reach kernel statics outside the heap, the data volume key among them, or a live stick's in-memory volume. In 0.9.2 only the installer's restart runs it, since the desktop cannot shut down. Explained in [Device secrets and keys](../security/device-secrets-and-keys.md#wiped-at-shutdown-and-reboot). Code: `src/security/zerostate/terminate.rs`, `src/security/hardening/memory_sanitization/api.rs`.

## See also

- [Overview](README.md)
- [Architecture](architecture.md)
- [FAQ](faq.md)
- [Kernel](../kernel/README.md)
- [Userland](../userland/README.md)
- [ABI](../abi/README.md)
