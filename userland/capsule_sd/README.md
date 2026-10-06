# capsule_sd

`sd` 1.0.0, the find-and-replace tool from crates.io, built for NONOS and signed as a capsule.
This directory holds only the capsule declaration (`Capsule.mk`); the source is vendored
byte-identical under `userland/upstream-src/sd-1.0.0`.

## Build

`mk/20-build.mk` builds it with `cargo install --path . --locked` from the vendored source for
the `x86_64-nonos-user` target, with `-Zbuild-std=std,panic_abort` and the nonos-rt start
object linked in. The result is `target/upstream-sd/sd` (`make nonos-mk-upstream-sd`), which
`CAPSULE_PREBUILT_BIN` hands to `nonos-mk/capsule.mk`. It is built from a path because cargo
honours `[patch]` only there: `sd-1.0.0/.cargo/config.toml` replaces is-terminal with
`userland/vendor/is-terminal` and errno with `userland/upstream-src/errno-0.3.14`. The program
source itself is unmodified. `nix build` makes the same binary from the same lock with
`--frozen` (`tools/nix/capsules.nix`).

## Capabilities and endpoints

`CAPSULE_REQUIRED_CAPS := 0x59`, with the same ceiling: CoreExec (0x1), IPC (0x8), Memory (0x10)
and FileSystem (0x40), the crates.io tool sandbox (`SANDBOX_CAPS` in
`src/userspace/tool_capsules/spec.rs`). FileSystem lets it read and rewrite the files it is
given through vfs. No Network, Debug, graphics or device bit.

Service `service:4822:tool.sd`, reply `reply:4823:endpoint.tool.sd.reply`.

## How it runs

The kernel mirror `src/userspace/capsule_sd` embeds the ELF, certificate, manifest and trailer
under the kernel feature `nonos-capsule-sd`, and init spawns it once at boot with that feature
(`src/userspace/init/entry.rs`). Typed at the terminal, `sd` runs from the vfs store through
`STORE_TOOLS` in `userland/capsule_terminal/src/jobs/classify.rs`, not from the tool table.

Handbook: [libc and std](../../docs/handbook/userland/libc-and-std.md),
[capsule catalogue](../../docs/handbook/apps/capsule-catalog.md).
