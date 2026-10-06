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

## Every page

### What NONOS is

- [The NONOS overview](overview/README.md): NONOS in four sentences, and the order to read the overview pages in.
- [Mission](overview/mission.md): what NONOS is for, who it serves, and what it does not try to be.
- [Architecture](overview/architecture.md): the whole system in one diagram, then each part with links to its pages.
- [Design principles](overview/design-principles.md): the rules the code follows, the check that holds each one, and where one does not hold yet.
- [Threat model](overview/threat-model.md): what NONOS protects, from whom, what it relies on and what it leaves out.
- [Glossary](overview/glossary.md): the terms these pages use, each with the file that defines it.
- [FAQ](overview/faq.md): short answers to the questions people ask first.

### Install

- [Installing NONOS](install/README.md): the path from a bare machine to NONOS on a stick and then on a disk.
- [Requirements](install/requirements.md): what a machine needs to boot from a stick and to install.
- [Get an image](install/get-an-image.md): build, seal and check an image, and the build profiles to pick from.
- [Write a USB stick](install/usb-stick.md): write the image, check the write, and boot from the stick.
- [Boot modes](install/boot-modes.md): the seven entries of the boot menu and what each one changes.
- [First boot](install/first-boot.md): the thirteen steps of setup, what each answer changes and where it is kept.
- [Install to disk](install/install-to-disk.md): how the installer picks a disk, what it writes, and what it refuses.
- [Update](install/update.md): moving an installed system to a newer release, and what the TPM changes on the way.
- [Recovery](install/recovery.md): the Recovery boot mode, reading a disk, and starting over.
- [Troubleshooting](install/troubleshooting.md): what each stop and error message means, and what to do.

### Using NONOS

- [Using NONOS](using/README.md): the guide for everyday use, and a first hour on the desktop.
- [The desktop](using/desktop.md): the menu bar, the dock, the Launchpad, windows and the apps that ship.
- [Terminal](using/terminal.md): tabs, line editing, pipes, jobs, every built-in command and git over HTTPS.
- [Files](using/files.md): where files live, the Files app, and exactly what is kept at power off.
- [Settings](using/settings.md): every Settings panel, what each row changes and how long a change lasts.
- [Keyboard layouts](using/keyboard-layouts.md): the layouts, choosing one at first boot and switching while you type.
- [Sound and media](using/audio.md): music, video, the volume, and why a machine may stay silent.
- [Wi-Fi and networking](using/wifi-and-networking.md): joining Wi-Fi, plugging in a cable, and the join errors.
- [Privacy networks](using/privacy-network.md): Nym, Anyone and Direct, and what each one hides and does not hide.
- [Linux programs](using/linux-programs.md): running a shell, Python, SQLite and the other Linux tools, and what they can reach.
- [Local model](using/local-ai.md): a Qwen language model on your own machine, offline, and how to pick its tier.
- [Marketplace](using/marketplace.md): what can be installed, how a listing is checked, and what needs a network.
- [Wallet](using/wallet.md): an Ethereum account, with its private key kept by the keyring rather than the wallet window.

### Hardware

- [Hardware support matrix](hardware/MATRIX.md): every device class and chip, its state, and what that state rests on.
- [Reporting a machine](hardware/report.md): how to tell the NONOS team what NONOS did on your machine.

### Kernel

- [The kernel](kernel/README.md): what runs in ring 0, what is left to capsules, and where each part lives in `src/`.
- [Boot handoff and kernel init](kernel/boot-handoff.md): how the loader hands over, what the kernel checks, and its start order.
- [Memory and paging](kernel/memory-and-paging.md): the address space, page tables, hardware protections and user copies.
- [Frame allocator](kernel/frame-allocator.md): which physical memory the kernel uses and how it hands out frames.
- [Scheduler and SMP](kernel/scheduler-and-smp.md): starting every CPU, per CPU state, and picking the next process.
- [Futex](kernel/futex.md): the two calls that let threads sleep on a word and wake each other.
- [Timers and clocks](kernel/timers.md): the counters the kernel reads, the scheduler tick and the time calls.
- [Kernel logging](kernel/logging.md): where kernel messages go, their tags, and how to read them.
- [Panic and boot stop](kernel/panic-and-boot-stop.md): what the kernel does when it cannot go on, and what you can do next.
- [System calls](kernel/syscalls.md): how a call enters the kernel, the dispatch, the capability check and the count.
- [Capabilities](kernel/capabilities.md): the capability bits, each process's token, the check, and grant and revoke.
- [IPC](kernel/ipc.md): endpoints, inboxes, size limits, who may send to whom, and timeouts.
- [Processes and capsule spawn](kernel/processes-and-spawn.md): the spawn gate, the ELF loader, stacks, and process exit.
- [The hardware broker](kernel/hardware-broker.md): the device table, claims, register windows, DMA, interrupts and release.
- [IOMMU](kernel/iommu.md): which devices the kernel confines with DMA remapping, and what happens when it cannot.
- [PCI and ACPI](kernel/pci-and-acpi.md): the ACPI tables the kernel reads and how it scans PCI on x86_64.

### Security

- [Security](security/README.md): what NONOS trusts, where each check lives, and which page answers which question.
- [Protections and limits](security/protections-and-limits.md): what NONOS protects against with the code that does it, and what it does not.
- [Boot chain and signatures](security/boot-chain-and-signatures.md): the checks at each stage, from the firmware to every capsule.
- [STARK attestation](security/stark-attestation.md): how a kernel, a loader or a capsule proves it belongs to the committed set.
- [Rollback protection](security/rollback-protection.md): the floors that keep older signed kernels, loaders and certificates out.
- [Measured boot and the TPM](security/measured-boot-and-tpm.md): what the TPM records and derives, and what is lost without one.
- [Capsule isolation](security/capsule-isolation.md): address spaces, the syscall check, send rules, confined drivers and the Linux sandbox.
- [Device secrets and keys](security/device-secrets-and-keys.md): where secrets live, what protects each, and what shutdown wipes.
- [Reporting a vulnerability](security/reporting-a-vulnerability.md): how to report a security bug in private, and what to include.

### Drivers

- [Drivers](drivers/README.md): how a device is found, how its driver starts and is confined, and every driver capsule.
- [Broker API](drivers/broker-api.md): the calls a driver makes to find, claim, map and release a device.
- [Writing a driver](drivers/writing-a-driver.md): a driver capsule from its manifest to its proof crate, built on the virtio-rng driver.
- [Platform](drivers/platform.md): processor cores, the IOMMU, the TPM, the ACPI buttons and GPIO.
- [Display](drivers/display.md): the UEFI GOP framebuffer, virtio-gpu, and what NONOS cannot drive.
- [Audio: Intel HD Audio](drivers/audio.md): the HD Audio driver, the audio server, the volume keys, and where it cannot play.
- [Input drivers](drivers/input/README.md): how keys, mouse movement and touches reach an app.
- [PS/2 keyboard and mouse](drivers/input/ps2.md): the i8042 driver for keyboards, mice and touchpads on the aux port.
- [I2C-HID touchpads](drivers/input/i2c-hid.md): the I2C controllers NONOS finds, binding the pad, and its gestures.
- [Storage drivers](drivers/storage/README.md): which disks NONOS reads and writes, and how a disk becomes the store and the data volume.
- [NVMe](drivers/storage/nvme.md): the NVMe driver, its limits, and how it was verified.
- [AHCI SATA and Intel RST](drivers/storage/ahci-and-rst.md): the SATA driver, RST mode, and what it refuses.
- [Intel VMD](drivers/storage/vmd.md): drives hidden behind VMD, what is untested, and the firmware setting to change.
- [SD cards and eMMC](drivers/storage/sd-and-emmc.md): soldered eMMC storage and SD card readers in this release.
- [USB mass storage](drivers/storage/usb-mass-storage.md): USB sticks and disks, and what is not supported.
- [USB and the xHCI host controller](drivers/usb/README.md): the xHCI driver and the class drivers above it.
- [USB keyboards and mice](drivers/usb/hid.md): what the USB HID driver binds and where its events go.
- [USB hubs](drivers/usb/hubs.md): a hub comes up, and the devices behind it are not reached yet.
- [Wi-Fi drivers](drivers/wifi/README.md): the two Wi-Fi drivers, a scan and a join from Settings to the radio, and what is refused.
- [Realtek RTL8821CE](drivers/wifi/rtl8821ce.md): the 2.4 GHz driver that scans, joins WPA2 and WPA3 networks and carries traffic.
- [Intel Wi-Fi (iwlwifi)](drivers/wifi/iwlwifi.md): the Intel cards it finds, and the few it can join on, so far only against a modelled device.
- [Wi-Fi chips with no driver](drivers/wifi/not-supported.md): how to tell whether your chip has a driver, and what to use instead.
- [Ethernet drivers](drivers/ethernet/README.md): every Ethernet driver, which are in the image, and the receive fault of the PCI drivers.
- [Intel Ethernet](drivers/ethernet/intel.md): e1000 in the image, and e1000e and igc written but not built.
- [Realtek Ethernet](drivers/ethernet/realtek.md): the RTL8139 and RTL8169 family drivers.
- [USB networking](drivers/ethernet/usb-net.md): five USB network drivers, and why none of them runs in 0.9.2.

### Userland

- [Userland](userland/README.md): what a capsule is, how it goes from source to a process, and what lives under `userland/`.
- [Manifests and capabilities](userland/manifests-and-capabilities.md): from `Capsule.mk` to a signed manifest to the capability word.
- [Signing and publisher keys](userland/signing-and-publisher-keys.md): the two hybrid key pairs, enrollment, and what the kernel refuses.
- [libc and the Rust runtimes](userland/libc.md): the three layers a capsule reaches the kernel through.
- [IPC services](userland/ipc-services.md): finding a service by name, who may send there, and the message layouts.
- [The Linux personality](userland/linux-personality.md): how unmodified x86_64 Linux programs run, and which Linux calls are served.

### ABI

- [The NONOS ABI](abi/README.md): how a call is made, which number names it, which capability admits it, and what comes back.
- [Syscalls](abi/syscalls.md): all 130 native syscalls with number, capability and meaning.
- [Errors](abi/errors.md): what a failed syscall returns, and every errno the kernel defines.
- [Capabilities](abi/capabilities.md): the 36 capability bits and the syscalls each one admits.
- [Broker](abi/broker.md): the driver calls to the hardware broker, their records and constants.
- [IPC](abi/ipc.md): the IPC calls, the message envelope, the limits and the well-known service ports.

### Build

- [Build NONOS](build/README.md): from a clean machine to an image booting under QEMU.
- [Toolchain](build/toolchain.md): setting up a build machine, and the pinned version of every tool.
- [The Nix flake](build/nix-flake.md): the flake's inputs, packages, apps, shell and checks.
- [Make targets](build/make-targets.md): every `make` target and the options a QEMU boot takes.
- [Profiles](build/profiles.md): the six build profiles, the keys of `nonos.toml`, and boot modes against build profiles.
- [The seal](build/seal.md): what turns unsigned artifacts into a bootable image, and what to do without the release keys.
- [Reproducible builds](build/reproducible-builds.md): what is pinned, how to compare two builds, and what is not reproducible yet.
- [SBOM](build/sbom.md): the bill of materials and the other supply chain records.
- [CI](build/ci.md): the GitHub workflows, what `make check` covers, and how the checks stood at this commit.

### Contributing

- [Contributing to NONOS](contributing/README.md): set up a checkout, make a change, check it and send it.
- [Code style](contributing/code-style.md): the rules a change is held to, and which check enforces each one.
- [Commits](contributing/commits.md): the subject line, the body, and what a commit leaves out.
- [Tests and proofs](contributing/tests-and-proofs.md): proof crates, static checks, Kani, Lean and fuzzing, and which checks fail at this commit.
- [Review](contributing/review.md): what CI runs on a pull request, who reviews it, and what reviewers look for.

### Architectures

- [Architectures](architectures/README.md): which CPU architectures NONOS runs on and how complete each port is.
- [x86_64](architectures/x86_64.md): the release target, what it needs from the CPU, and where each part lives.
- [aarch64](architectures/aarch64.md): a preview port, how to build and boot it under QEMU, and what is missing.
- [riscv64](architectures/riscv64.md): a backend that does not build into a kernel, and what it would take.

### Release notes

- [NONOS 0.9.2](release/0.9.2.md): the release notes for this version.

## See also

- [The NONOS overview](overview/README.md)
- [Glossary](overview/glossary.md)
- [Hardware support matrix](hardware/MATRIX.md)
- [README.md](../README.md) at the root of the repository
- [SECURITY.md](../SECURITY.md)
