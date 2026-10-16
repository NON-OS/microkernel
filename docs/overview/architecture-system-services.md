# Architecture: System services

Part of [Architecture](architecture.md).

## System services

System services are capsules that other capsules reach by name over IPC: the
[file store](glossary.md#file-store) (`vfs`, `ramfs`), keys and randomness
(`keyring`, `entropy`, `crypto`), settings (`policy`), attestation (`attest`),
the network stack (`net.core` and `net.sockets` on the desktop images), the
anonymity transports (`net.nym`, `net.anon`, `net.socks5`), audio, input
routing, and the display (`compositor`, `wm`, `desktop_shell`). Init, which runs
in the kernel, starts them in a fixed order: ramfs, the core services, the
display core, the drivers, vfs, the network, the desktop, the market and the
apps (`run_init` in `src/userspace/init/entry.rs:20-48`). When a service on its
watch list of 30 names ends, init starts it again (`WATCHED` in
`src/userspace/init/supervisor/watch_rule.rs:17-59`), at most 8 times
(`DEFAULT_MAX_RESTARTS` in `src/services/lifecycle/state/constants.rs:17`).
Drivers, apps, setup, the installer and the Linux personality end on purpose, so
the list leaves them out. Reaching any of the 11 services that carry traffic off
the machine takes the Network bit as well as IPC (`NETWORK_SERVICES` in
`src/services/registry/policy.rs:19-45`).

[IPC services](../userland/ipc-services.md) lists them with their
[endpoints](glossary.md#endpoint), [IPC](../kernel/ipc.md) covers the kernel
side, and [How the Nym and Anyone transports are
built](../security/anonymity-transports.md) covers `net.nym` and `net.anon`.

## Apps

Apps are capsules as well: the terminal, files, the text editor, settings, the
browser, the wallet, the marketplace, the media players and the rest of the
desktop. So is each command-line tool the Terminal starts, such as `tokei` or
`csview`. At spawn, an app's [capability word](glossary.md#capability-word)
holds the required bits its [manifest](glossary.md#manifest) declares, plus
those of its optional bits the spawning code allows, and nothing else
(`install_caps` in `src/security/capsule_manifest/verify/caps_bits.rs:38-47`).
The kernel installs the word at spawn from the verified manifest
(`spawn_verified` in
`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:25-33`). On an
Air-Gapped, Safe Mode or Recovery boot, the [spawn gate](glossary.md#spawn-gate)
starts no network driver or service and takes Network from every other capsule
(`check` and `caps` in
`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:17-55`).

[Using NONOS](../using/README.md) covers the apps from the keyboard, with
[Everyday apps](../using/apps.md) and [Command-line
tools](../using/command-line-tools.md) for the small ones. [The capsule
model](../userland/README.md) and [Manifests and
capabilities](../userland/manifests-and-capabilities.md) cover them from the
code, and [Writing an app](../userland/writing-an-app.md) builds one.

## Linux personality and Linux programs

The Linux personality, `userland/capsule_linux`, runs unmodified x86_64 Linux
programs. It is the only capsule that holds `ForeignExec`, the right to create a
process the kernel has not verified, build its address space and answer the
calls it makes (`userland/capsule_linux/Capsule.mk:4-9`,
`src/capabilities/types/defs.rs:72-74`). A Linux program holds no NONOS
capabilities. When it makes a syscall, the kernel parks the call as a
`ForeignFrame` and wakes the personality to answer it (`redirect` in
`src/process/foreign/trap.rs:26-54`). A call the personality does not serve gets
ENOSYS and a log line with its number (`unserved` in
`userland/capsule_linux/src/linux/serve/unserved.rs:21-41`). Only the
personality's install role asks for Network; the run and terminal roles do not,
and every network service requires that bit (`INSTALL`, `RUN` and `TERMINAL` in
`src/userspace/capsule_linux/roles.rs:35-67`). A Linux program read from the
store runs only if the proof kept beside it verifies (`resolve` in
`userland/capsule_linux/src/linux/call/spawn/exec_resolve.rs:40-60`).

[Linux personality](../userland/linux-personality.md) says which Linux syscalls
are served and which are refused, and [Linux
programs](../using/linux-programs.md) covers running them.
