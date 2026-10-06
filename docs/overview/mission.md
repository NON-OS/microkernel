# Mission

What NONOS is for, who it serves, and what it does not try to be.

## What NONOS is

NONOS is an operating system for x86_64 computers. Its kernel is a capability microkernel written in Rust. Most of what a person would call the operating system runs above it in ring 3, each part as a [capsule](glossary.md#capsule): the drivers, the network stack, the desktop, the apps, and a [Linux personality](glossary.md#linux-personality) that runs Linux programs. Each capsule holds the capabilities its signed manifest grants and nothing more, and the kernel checks them on every syscall. The kernel itself keeps memory, scheduling, IPC, capabilities and capsule spawn, and in this release also the TPM driver and the encrypted data volume. [Architecture](architecture.md) shows how the parts fit together.

## What it is for

Each aim below is held by code you can read.

### A machine that forgets

Every boot is [amnesic](glossary.md#amnesic-boot): nothing is kept unless the person chooses to install during first-boot setup (`nonos.toml`). On a live stick the [data volume](glossary.md#data-volume) lives in RAM, sealed under a key made for that boot alone, and is gone at power off (`src/fs/cryptoblock/ram.rs`). When NONOS shuts down or restarts, the kernel stops the other CPUs and every claimed device, then wipes device buffers, process memory, kernel stacks, filesystem caches, keys, its RAM log and its heap (`zerostate_shutdown_wipe` in `src/security/hardening/memory_sanitization/api.rs:59-109`). A kernel panic halts the machine without that wipe (`panic` in `src/boot/panic/handler.rs:41-64`).

### Only checked code runs

Before a capsule runs, the kernel checks its certificate, then its manifest and image under a policy that requires both Ed25519 and ML-DSA-65 signatures (`NONOS_PRODUCTION_POLICY` in `src/security/nonos_id_cert/policy.rs:30-32`), then its attestation. It installs only the capabilities the manifest grants. [Design principles](design-principles.md#everything-that-runs-is-signed-and-measured) walks through each step.

### Private by default on the network

The browser, the Terminal and the wallet connect through the default network: the Nym mixnet, unless the person picks the Anyone onion network or Direct (`NYM`, `ANYONE` and `DIRECT` in `userland/policy_proto/src/route.rs:25-35`). When the chosen network is not running there is no route, and nothing falls back to a direct connection (`pick` in `userland/nonos_route_link/src/pick.rs:125-139`). Downloads for an install, a Qwen model or a Linux package, go over the Anyone network whatever the default (`install_route` in `userland/nonos_route_link/src/pick.rs:49-63`). See [Privacy network](../using/privacy-network.md).

### A local model and Linux programs

In the Terminal, `qwen get` downloads the files of a Qwen model tier, and the kernel keeps each file on the data volume only if its SHA-256 matches the pin in the signed model catalogue (`StreamImport` in `src/capabilities/types/defs.rs:77-79`, `userland/capsule_model_fetch/Capsule.mk`). The model runs under the Linux personality, and a Linux program that holds a model gets no internet socket (`refuse_inet` in `userland/capsule_linux/src/linux/net/offline.rs:36-39`). The same personality runs other x86_64 Linux programs, each only if the proof kept beside it verifies. See [Local model](../using/local-ai.md) and [Linux programs](../using/linux-programs.md).

### A wallet that does not hold its key

The keyring capsule holds the wallet's key and signs with it. The wallet keeps only a record sealed to this machine in its current boot state, and the keyring seals or opens that record only for a sender the service registry names as the wallet (`allowed` in `userland/capsule_keyring/src/server/vault_gate/mod.rs:23-30`). The raw key leaves the keyring only on an explicit export request from the process it belongs to (`wallet_export` in `userland/capsule_keyring/src/server/handlers/wallet_export.rs:21-27`). See [Wallet](../using/wallet.md).

## Who it is for

- A person who wants a laptop that keeps nothing by default and sends the system's own connections through an anonymity network. Start with [Install](../install/README.md).
- A person who holds keys and wants every program on the machine to run with only the rights it was enrolled for. Read the [threat model](threat-model.md), then [Wallet](../using/wallet.md).
- An OS developer or a security reviewer who wants each claim tied to a file and a line. Read [Architecture](architecture.md) and [Design principles](design-principles.md).
- A contributor who wants to write a capsule or a driver. Read [Contributing](../contributing/README.md) and [Writing a driver](../drivers/writing-a-driver.md).
