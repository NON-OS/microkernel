# Mission

What NONOS is for, who it serves, and what it does not try to be.

## What NONOS is

NONOS is an operating system for x86_64 computers. Its kernel is a capability microkernel written in Rust, and it runs on every core: it starts every CPU the firmware enables and schedules processes on all of them. Most of what a person would call the operating system runs above it in ring 3, each part as a [capsule](glossary.md#capsule): the drivers, the network stack, the desktop, the apps, and a [Linux personality](glossary.md#linux-personality) that runs Linux programs. Each capsule starts with the capabilities its signed [manifest](glossary.md#manifest) grants and nothing more, and the kernel checks them on every NONOS syscall. On Intel VT-d machines the [IOMMU](glossary.md#iommu) confines device DMA: each PCI device a driver claims reaches only the buffers granted to that driver (`attach` in `src/hardware/broker/confine/attach.rs:30-101`). AMD-Vi is not driven and interrupt remapping is off in this release; [IOMMU](../kernel/iommu.md) says what that leaves open. The kernel itself keeps memory, scheduling, IPC, capabilities and capsule spawn, and in this release also the [TPM](glossary.md#tpm) driver and the encrypted [data volume](glossary.md#data-volume). [Architecture](architecture.md) shows how the parts fit together.

## What it is for

Each aim below is held by code you can read.

### A machine that forgets

Every boot is [amnesic](glossary.md#amnesic-boot): nothing is kept unless the person chooses to install, in first-boot setup or from the boot menu, and an image built with `install` set to false can never keep anything (`nonos.toml:22-25`). On a live stick the data volume lives in RAM, sealed under a key made for that boot alone, and is gone at power off (`src/fs/cryptoblock/ram.rs`). Before a shutdown or restart the kernel runs the [ZeroState](glossary.md#zerostate) wipe over device buffers, process memory, kernel stacks, caches, its key vault and its heap (`zerostate_shutdown_wipe` in `src/security/hardening/memory_sanitization/api.rs:59-109`). The wipe leaves kernel statics outside the heap, the data volume key in `VOLUME` among them (`src/fs/blockfs_volume/state.rs:21-26`), and a live stick's in-memory volume. A kernel panic or a power cut skips it, and in 0.9.2 only the installer's restart runs it, because the desktop cannot shut down. [Design principles](design-principles.md#amnesic-by-default) gives the order of the wipe and the code for each step.

### Only checked code runs

Before a capsule runs, the kernel checks its certificate, then its manifest and image under a policy that requires both Ed25519 and ML-DSA-65 signatures (`NONOS_PRODUCTION_POLICY` in `src/security/nonos_id_cert/policy.rs:30-32`), then its [attestation](glossary.md#attestation). It installs only the capabilities the manifest grants. [Design principles](design-principles.md#everything-that-runs-is-signed-and-measured) walks through each step. Before any capsule, the loader of a release image refuses a kernel whose signature or STARK trailer fails, and the kernel checks the loader in turn before init (`check_bootloader` in `src/kernel_core/init/entry/microkernel_main.rs:24-27`); [Boot chain and signatures](../security/boot-chain-and-signatures.md) says what each side checks and what rests on the loader's own verification code.

### Private by default on the network

The browser, the Terminal and the wallet connect through the default network: the [Nym mixnet](glossary.md#nym-mixnet), unless the person picks the [Anyone network](glossary.md#anyone-network) or Direct (`NYM`, `ANYONE` and `DIRECT` in `userland/policy_proto/src/route.rs:25-35`). The wallet never goes direct: under a Direct default it takes Nym, or Anyone when Nym is not running (`private_only` in `userland/nonos_route_link/src/pick.rs:93-100`). When the chosen network is not running there is no route, and nothing falls back to a direct connection (`pick` in `userland/nonos_route_link/src/pick.rs:125-139`). Downloads for an install, a Qwen model or a Linux package, go over the Anyone network whatever the default (`install_route` in `userland/nonos_route_link/src/pick.rs:49-63`). A model goes direct only when the person asks for that one download, with `qwen get --direct` in the Terminal or `d` on its Marketplace card (`DIRECT_WORD` in `userland/capsule_model_fetch/src/get/run.rs:42`), and the mirror then sees this machine's address. See [Privacy network](../using/privacy-network.md).

### A local model and Linux programs

In the Terminal, `qwen get` downloads the files of a Qwen model tier, and the kernel keeps each file on the data volume only if its SHA-256 matches the pin in the signed model catalogue (`StreamImport` in `src/capabilities/types/defs.rs:77-79`, `userland/capsule_model_fetch/Capsule.mk`). The model runs under the Linux personality, and a Linux program that holds a model gets no internet socket (`refuse_inet` in `userland/capsule_linux/src/linux/net/offline.rs:36-39`). The same personality runs other x86_64 Linux programs, each only if the proof kept beside it verifies. See [Local model](../using/local-ai.md) and [Linux programs](../using/linux-programs.md).

### A wallet that does not hold its key

The keyring capsule holds the wallet's key and signs with it. The wallet keeps only a record sealed to this machine in its current boot state, and the keyring seals or opens that record only for a sender the service registry names as the wallet (`allowed` in `userland/capsule_keyring/src/server/vault_gate/mod.rs:23-30`). The raw key leaves the keyring only on an explicit export request from the process it belongs to (`wallet_export` in `userland/capsule_keyring/src/server/handlers/wallet_export.rs:21-27`). See [Wallet](../using/wallet.md).

## Who it is for

- A person who wants a laptop that keeps nothing by default and sends the system's own connections through an anonymity network. Start with [Install](../install/README.md).
- A person who holds keys and wants every program on the machine to run with only the rights it was enrolled for. Read the [threat model](threat-model.md), then [Wallet](../using/wallet.md).
- An OS developer or a security reviewer who wants each claim tied to a file and a line. Read [Architecture](architecture.md) and [Design principles](design-principles.md).
- A contributor who wants to write an app or a driver. Read [Contributing](../contributing/README.md), then [Writing an app](../userland/writing-an-app.md) or [Writing a driver](../drivers/writing-a-driver.md).

## What NONOS is not

- Not a Linux distribution. There is no Linux kernel. Linux programs run in processes that hold no NONOS capabilities, and a call the personality does not serve gets ENOSYS (`unserved` in `userland/capsule_linux/src/linux/serve/unserved.rs:21-41`).
- Not broad hardware support yet. This release builds bootable images for x86_64 only. The aarch64 kernel builds as a preview that boots under QEMU and is not a release target (`nonos-arch-preview` in `Cargo.toml`). The riscv64 backend does not build into a kernel at all: it has no kernel target file, and its entry calls a `kernel_main` that does not exist (`src/arch/riscv64/boot/entry.rs`); see [Architectures](../architectures/README.md). A Standard or Hardened image carries 18 [driver capsules](glossary.md#driver-capsule) for a fixed list of device classes and chips, which the [support matrix](../hardware/MATRIX.md) gives; the Air-Gapped image drops the six network drivers, and the QEMU image carries 11 (`profiles` in `tools/nix/config.nix:69-117`).
- Not anonymous against every observer. Direct shows this machine's address to every site it reaches. net.nym and net.anon start on every boot that starts the network stack, whatever the choice, and contact a gateway or build a circuit even when nothing uses them, so the local network can see that NONOS runs Nym and Anyone. The [threat model](threat-model.md) lists the other limits.
- Not verified as a whole. [Proof crates](glossary.md#proof-crate) test the shipped kernel and capsule source on the host, and Lean 4 models cover chosen properties. NONOS makes no claim of functional correctness for the whole kernel; [Design principles](design-principles.md#proofs-live-next-to-the-code) gives the results on this commit.
- Not independently audited. This release claims no third-party security audit.

## See also

- [Overview](README.md)
- [Architecture](architecture.md)
- [Threat model](threat-model.md)
- [FAQ](faq.md)
- [Release notes for 0.9.2](../release/0.9.2.md)
