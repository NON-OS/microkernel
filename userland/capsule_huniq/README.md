# capsule_huniq

`huniq` 2.7.0 from crates.io (https://github.com/koraa/huniq) filters duplicate lines out of its
input, as a faster `sort | uniq`. This directory holds only the capsule declaration that packages
the upstream binary for NONOS; the source lives in `userland/upstream-src/huniq`. It is for someone
at the terminal cleaning up line-oriented output, who reaches it from the terminal or the desktop
launchpad.

## Role

A command-line tool capsule. `mk/20-build.mk` lists `huniq` in `NONOS_TOOL_BINS` and builds it with
`cargo install --path . --locked` from `userland/upstream-src/huniq` for the `x86_64-nonos-user` target, with
`-Zbuild-std=std,panic_abort`, the nonos-rt start object linked in, and `--no-default-features`. The
result lands in `target/upstream-huniq/bin/huniq`, which `CAPSULE_PREBUILT_BIN` in `Capsule.mk`
hands to `nonos-mk/capsule.mk`. The vendored `Cargo.toml` ends with a `[patch.crates-io]` block that
replaces atty and os_str_bytes with the copies in `userland/upstream-src/atty-0.2.14` and
`userland/upstream-src/os_str_bytes-6.6.1`. The program source itself is the crates.io source.

`--locked` holds every dependency to the committed `Cargo.lock`. `nix build` makes the same
binary from the same lock with `--frozen` (`tools/nix/capsules.nix`).

The kernel embeds the ELF, certificate, manifest and attestation trailer through the generated block
in `src/userspace/tool_capsules/registry.rs` (feature `nonos-tool-capsules`). `run_named` in
`src/userspace/tool_capsules/run.rs` spawns it on request, parented to the caller, with the argument
vector the caller passed (`src/userspace/tool_capsules/spec.rs`).

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x59`, with `CAPSULE_CAPS_CEILING := 0x59`:

- `0x01` CoreExec: run at all.
- `0x08` IPC: reach other services, including the vfs capsule for any file it opens.
- `0x10` Memory: the std heap.
- `0x40` FileSystem: any file it opens; the std layer asks vfs, which serves only a holder
  of FileSystem.

This is `SANDBOX_CAPS` in `src/userspace/tool_capsules/spec.rs`, the same set for every crates.io
tool. No Network, Debug or graphics bit is declared.

Endpoints: service `service:4912:tool.huniq`, reply `reply:4913:endpoint.tool.huniq.reply`.

## Interface

There is no IPC protocol of its own: the capsule runs upstream `main` with an argv. The terminal
maps the typed name `huniq` to service `tool.huniq`
(`userland/capsule_terminal/src/command/builtin/tool.rs`), calls `mk_tool_run`, and attaches the
child to its tty so it feeds stdin and drains stdout. The desktop launchpad lists the same service
in `userland/capsule_desktop_shell/src/state/tool_apps.rs`. Files the tool opens go through the std
layer in `toolchain/nonos-std/sys/fs/nonos`, which asks the vfs capsule over IPC.

## State and privacy

The capsule keeps nothing between runs: each invocation is a fresh process that exits when upstream
`main` returns. It reads and writes only what its arguments and stdin point it at. It holds no
network, device, graphics or key material authority.

## Build and test

- Upstream binary: `make nonos-mk-upstream-tools` (or the file target
  `target/upstream-huniq/bin/huniq`).
- Capsule ELF: `make nonos-mk-huniq` (the include sits in `mk/20-build.mk`); certificate, manifest
  and trailer: `make nonos-mk-huniq-sign`.
- Boot self-test: the kernel feature `nonos-tool-selftest` runs `huniq --version` once at boot and
  frames its output with `TOOL-SELFTEST` markers (`src/userspace/init/entry.rs`).

No proof crate covers this capsule.

## Not done yet

- No test in the repository runs `huniq` beyond the single `--version` call of the boot self-test.
- The kernel registry and the desktop `TOOL_APPS` table are generated from `userland/apps.list`, but
  the terminal `TOOLS` table (`userland/capsule_terminal/src/command/builtin/tool.rs`) and the
  self-test list in `src/userspace/init/entry.rs` name `huniq` by hand.

Handbook: [libc and std](../../docs/handbook/userland/libc-and-std.md),
[capsule catalogue](../../docs/handbook/apps/capsule-catalog.md).
