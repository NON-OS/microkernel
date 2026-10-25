# Glossary: Rollback index

Part of [Glossary](glossary.md).

## Rollback index

The anti-rollback number signed into the kernel image together with its BLAKE3
hash, set by `rollback_index` in `nonos.toml`, 1 by default; the flake refuses a
value below 1. Raising it for a release retires every older kernel on each
machine with a TPM where the new one boots. Explained in [Rollback
protection](../security/rollback-protection.md#the-kernels-rollback-index).
Code: `nonos-bootloader/tools/sign-kernel/src/message.rs`,
`tools/nix/config.nix`, `nonos.toml`.

## Seal

The step between the reproducible build and a bootable image, run as `make seal`
or `nix run .#seal`. In six phases it signs the market inputs, gives every
capsule its certificate, manifest and trailer, enrolls the kernel and the
loader, writes the signed image with its ESP, store, USB image and ISO, and
checks what it wrote. It compiles nothing itself, and the build never holds a
key. Explained in [The seal](../build/seal.md). Code:
`tools/nonos_seal/__init__.py`.

## Secure Boot

The UEFI firmware feature that starts only a loader whose signature the
firmware's signature database, db, trusts. The Hardened boot entry needs it on,
with a platform key and a db; the hardened and airgapped build profiles, built
with the `production` loader policy, are described in `tools/nix/config.nix` as
refusing to start without Secure Boot and a TPM. When the seal has the NONOS db
key it signs `BOOTX64.EFI` with that key alone, and a release seal of a
`production` loader stops without it. Explained in
[Requirements](../install/requirements.md#secure-boot-and-the-tpm). Code:
`nonos-bootloader/src/bootmenu/ready.rs`, `tools/nix/config.nix`,
`tools/nonos_seal/chain.py`.

## Serial console

The kernel's main log, a UART that takes its tagged lines: on x86_64 the 16550
at I/O port 0x3F8, set to 115200 8N1, and on aarch64 the PL011. With no UART
present the output is dropped and the boot goes on; a capsule may write to it
only with the Debug capability, which a spawn grants only in an image built with
`capsule-serial-debug`. Explained in
[Logging](../kernel/logging.md#the-serial-console). Code:
`src/arch/x86_64/console.rs`, `src/capabilities/serial_debug.rs`.

## SMP

Symmetric multiprocessing: the kernel runs on every core, not only the boot CPU.
It starts every CPU the firmware enables in the ACPI MADT, up to 256, and every
one of them takes processes from one run queue shared by the whole machine. On
an Intel hybrid part an idle performance core is offered new work before an
efficiency core. Every build profile turns it on through the `nonos-smp` feature
unless `nonos.toml` sets `smp = false`, and a kernel built without that feature
runs on the boot CPU alone. A Linux program is still told it has one CPU.
Explained in [Scheduler and SMP](../kernel/scheduler-and-smp.md). Code:
`src/smp/init/ap_start.rs`, `src/process/scheduler/dispatch/run_queue.rs`,
`src/smp/ipi_handler.rs`.

## Spawn gate

The kernel path every capsule passes before it becomes a process,
`spawn_verified_as`. It checks, in order, that the boot profile allows the
capsule, its NONOS ID certificate, its manifest (namespace, capability ceiling,
signatures, payload hash, target, endpoints and the grant), then its attestation
trailer, and refuses the spawn at the first failure. Explained in [Processes and
capsule spawn](../kernel/processes-and-spawn.md#the-spawn-gate). Code:
`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs`,
`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs`.
