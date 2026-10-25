# NONOS documentation: Where to start

Part of [NONOS documentation](README.md).

## Where to start

### A person with a laptop

You want NONOS running on your own machine, first from a USB stick, then perhaps
installed.

1. [Mission](overview/mission.md): what NONOS is for, and what it is not.
2. [Requirements](install/requirements.md): the processor, firmware, memory,
   stick and disk it needs.
3. [Hardware support matrix](hardware/MATRIX.md): whether your chips have a
   driver, and what that rests on.
4. [Get an image](install/get-an-image.md): this repository names no download,
   so you build and seal an image yourself.
5. [Write a USB stick](install/usb-stick.md), then [Boot
   modes](install/boot-modes.md) and [First boot](install/first-boot.md). Once
   the desktop is up, [Using NONOS](using/README.md) takes over, with [Wi-Fi and
   networking](using/wifi-and-networking.md) for getting online.
6. [Install to disk](install/install-to-disk.md), only if you want the machine
   to keep anything.
7. [Troubleshooting](install/troubleshooting.md) when a step stops, and
   [Reporting a machine](hardware/report.md) to tell the NONOS team what
   happened.

### A user of a running system

NONOS is booted and you want to get work done.

1. [Using NONOS](using/README.md): what is kept at power off, and where each
   program stops.
2. [The desktop](using/desktop.md), then [Files](using/files.md), [Everyday
   apps](using/apps.md) and [Settings](using/settings.md).
3. [Wi-Fi and networking](using/wifi-and-networking.md), [Privacy
   networks](using/privacy-network.md) and [The Browser](using/browser.md):
   getting online, and how your connections leave the machine.
4. [Terminal](using/terminal.md), [Command-line
   tools](using/command-line-tools.md), [Linux
   programs](using/linux-programs.md) and [Local model](using/local-ai.md).
5. [Wallet](using/wallet.md), [Marketplace](using/marketplace.md), [Sound and
   media](using/audio.md) and [Keyboard layouts](using/keyboard-layouts.md) as
   you need them.

### An OS developer

You want to know how the system is built, from the boot to a running capsule.

1. [Architecture](overview/architecture.md), then [Design
   principles](overview/design-principles.md).
2. [The kernel](kernel/README.md), then [Boot handoff](kernel/boot-handoff.md),
   [Memory and paging](kernel/memory-and-paging.md), [Scheduler and
   SMP](kernel/scheduler-and-smp.md) and [System calls](kernel/syscalls.md).
3. [Capabilities](kernel/capabilities.md), [IPC](kernel/ipc.md) and [Processes
   and capsule spawn](kernel/processes-and-spawn.md).
4. [Userland](userland/README.md), then [Manifests and
   capabilities](userland/manifests-and-capabilities.md) and [libc and the Rust
   runtimes](userland/libc.md).
5. [Writing an app](userland/writing-an-app.md), then [Shipping an
   app](userland/shipping-an-app.md): a windowed app from its crate to a tile on
   the dock.
6. [The ABI](abi/README.md) when you need exact numbers and layouts.
7. [Architectures](architectures/README.md) and [Build NONOS](build/README.md).
8. [Kernel internals: the source map](internals/README.md) when you want the
   `src/` tree itself — one page per module, with the key types and functions
   and where each lives.
