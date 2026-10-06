# capsule_dotenv-linter

`dotenv-linter` 4.0.0 from crates.io (https://github.com/dotenv-linter/dotenv-linter) checks `.env`
files for common mistakes. This directory holds only the capsule declaration that packages the
upstream binary for NONOS; the source lives in `userland/upstream-src/dotenv-linter`. It is for
someone at the terminal who keeps environment files for a project, who reaches it from the terminal
or the desktop launchpad.

## Role

A command-line tool capsule. `mk/20-build.mk` lists `dotenv-linter` in `NONOS_TOOL_BINS` and builds
it with `cargo install --path . --locked` from `userland/upstream-src/dotenv-linter` for the
`x86_64-nonos-user` target, with `-Zbuild-std=std,panic_abort`, the nonos-rt start object linked in,
and `--no-default-features`. The result lands in `target/upstream-dotenv-linter/bin/dotenv-linter`,
which `CAPSULE_PREBUILT_BIN` in `Capsule.mk` hands to `nonos-mk/capsule.mk`. The vendored
`Cargo.toml` carries no `[patch.crates-io]` block, so no dependency is swapped for a local copy.

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

Endpoints: service `service:4902:tool.dotenv-linter`, reply
`reply:4903:endpoint.tool.dotenv-linter.reply`.

## Interface

There is no IPC protocol of its own: the capsule runs upstream `main` with an argv. The terminal
maps the typed name `dotenv-linter` to service `tool.dotenv-linter`
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
  `target/upstream-dotenv-linter/bin/dotenv-linter`).
- Capsule ELF: `make nonos-mk-dotenv-linter` (the include sits in `mk/20-build.mk`); certificate,
  manifest and trailer: `make nonos-mk-dotenv-linter-sign`.
- Boot self-test: the kernel feature `nonos-tool-selftest` runs `dotenv-linter --version` once at
  boot and frames its output with `TOOL-SELFTEST` markers (`src/userspace/init/entry.rs`).

No proof crate covers this capsule.

## Not done yet

- No test in the repository runs `dotenv-linter` beyond the single `--version` call of the boot
  self-test.
- The kernel registry and the desktop `TOOL_APPS` table are generated from `userland/apps.list`, but
  the terminal `TOOLS` table (`userland/capsule_terminal/src/command/builtin/tool.rs`) and the
  self-test list in `src/userspace/init/entry.rs` name `dotenv-linter` by hand.

Handbook: [libc and std](../../docs/handbook/userland/libc-and-std.md),
[capsule catalogue](../../docs/handbook/apps/capsule-catalog.md).
