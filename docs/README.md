# NONOS documentation

This page is the way into the NONOS documentation: what to read first for what you want to do, and one line on every page.

NONOS is an operating system for x86_64 computers, built on a capability microkernel in Rust, in which drivers, system services and apps run in ring 3 as signed [capsules](overview/glossary.md#capsule) that hold only the capabilities their manifests grant.

Every page names, in its last line, the commit of the source tree it was checked against. When a page and the code disagree, the code is what runs.

## Where to start

### A person with a laptop

You want NONOS running on your own machine, first from a USB stick, then perhaps installed.

1. [Mission](overview/mission.md): what NONOS is for, and what it is not.
2. [Requirements](install/requirements.md): the processor, firmware, memory, stick and disk it needs.
3. [Hardware support matrix](hardware/MATRIX.md): whether your chips have a driver, and what that rests on.
4. [Get an image](install/get-an-image.md): this repository names no download, so you build and seal an image yourself.
5. [Write a USB stick](install/usb-stick.md), then [Boot modes](install/boot-modes.md) and [First boot](install/first-boot.md).
6. [Install to disk](install/install-to-disk.md), only if you want the machine to keep anything.
7. [Troubleshooting](install/troubleshooting.md) when a step stops, and [Reporting a machine](hardware/report.md) to tell the NONOS team what happened.

### A user of a running system

NONOS is booted and you want to get work done.

1. [Using NONOS](using/README.md): what is kept at power off, and where each program stops.
2. [The desktop](using/desktop.md), then [Files](using/files.md) and [Settings](using/settings.md).
3. [Privacy networks](using/privacy-network.md) and [Wi-Fi and networking](using/wifi-and-networking.md): how your connections leave the machine.
4. [Terminal](using/terminal.md), [Linux programs](using/linux-programs.md) and [Local model](using/local-ai.md).
5. [Wallet](using/wallet.md), [Marketplace](using/marketplace.md), [Sound and media](using/audio.md) and [Keyboard layouts](using/keyboard-layouts.md) as you need them.

### An OS developer

You want to know how the system is built, from the boot to a running capsule.

1. [Architecture](overview/architecture.md), then [Design principles](overview/design-principles.md).
2. [The kernel](kernel/README.md), then [Boot handoff](kernel/boot-handoff.md), [Memory and paging](kernel/memory-and-paging.md), [Scheduler and SMP](kernel/scheduler-and-smp.md) and [System calls](kernel/syscalls.md).
3. [Capabilities](kernel/capabilities.md), [IPC](kernel/ipc.md) and [Processes and capsule spawn](kernel/processes-and-spawn.md).
4. [Userland](userland/README.md), then [Manifests and capabilities](userland/manifests-and-capabilities.md) and [libc and the Rust runtimes](userland/libc.md).
5. [The ABI](abi/README.md) when you need exact numbers and layouts.
6. [Architectures](architectures/README.md) and [Build NONOS](build/README.md).

### A security reviewer

You want to know what NONOS claims, what holds each claim, and where the claims stop.

1. [Threat model](overview/threat-model.md), then [Security](security/README.md).
2. [Protections and limits](security/protections-and-limits.md): every protection with its code, and every known gap with its reason.
3. [Boot chain and signatures](security/boot-chain-and-signatures.md), [STARK attestation](security/stark-attestation.md), [Rollback protection](security/rollback-protection.md) and [Measured boot and the TPM](security/measured-boot-and-tpm.md).
4. [Capsule isolation](security/capsule-isolation.md) and [Device secrets and keys](security/device-secrets-and-keys.md).
5. [Capabilities](kernel/capabilities.md), [IOMMU](kernel/iommu.md) and [Signing and publisher keys](userland/signing-and-publisher-keys.md) for the mechanisms underneath.
6. [Reproducible builds](build/reproducible-builds.md) and [SBOM](build/sbom.md) for the supply chain.
7. [Reporting a vulnerability](security/reporting-a-vulnerability.md) when you find something.

### A driver writer

You want a device to work under NONOS.

1. [Drivers](drivers/README.md): how a device is found, how its driver starts, and how it is confined.
2. [The hardware broker](kernel/hardware-broker.md) for the kernel side, then [Broker API](drivers/broker-api.md) for the driver side.
3. [Writing a driver](drivers/writing-a-driver.md): a whole driver capsule, step by step.
4. [Broker](abi/broker.md) in the ABI section for the records and constants, and [IOMMU](kernel/iommu.md) for DMA.
5. [Tests and proofs](contributing/tests-and-proofs.md): every driver has a proof crate.
6. [Hardware support matrix](hardware/MATRIX.md) and [Reporting a machine](hardware/report.md).

### A contributor

You want to send a change.

1. [CONTRIBUTING.md](../CONTRIBUTING.md), then [Contributing to NONOS](contributing/README.md).
2. [Build NONOS](build/README.md) and [Toolchain](build/toolchain.md).
3. [Code style](contributing/code-style.md) and [Commits](contributing/commits.md).
4. [Tests and proofs](contributing/tests-and-proofs.md), [CI](build/ci.md) and [Review](contributing/review.md).
