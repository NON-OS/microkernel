# capsule_ripgrep

`rg`, ripgrep 14.1.1 from crates.io, built unmodified for NONOS and signed as a capsule. This
directory holds only the capsule declaration (`Capsule.mk`); no source of ripgrep lives in the
tree.

## Build

`mk/20-build.mk` builds it with `cargo install ripgrep --version 14.1.1 --locked` for the
`x86_64-nonos-user` target, with `-Zbuild-std=std,panic_abort` and the nonos-rt start object
linked in, so the program's own `main` runs on the NONOS std layer. The result is
`target/upstream-ripgrep/rg` (`make nonos-mk-upstream-ripgrep`), which `CAPSULE_PREBUILT_BIN`
hands to `nonos-mk/capsule.mk`. `nix build` fetches the same crate by its sha256, checks that its
`Cargo.lock` equals `tools/nix/locks/ripgrep-14.1.1.Cargo.lock`, and builds with `--frozen`
(`tools/nix/capsules.nix`).

## Capabilities and endpoints

`CAPSULE_REQUIRED_CAPS := 0x59`, with the same ceiling: CoreExec (0x1), IPC (0x8), Memory (0x10)
and FileSystem (0x40). That is the crates.io tool sandbox, `SANDBOX_CAPS` in
`src/userspace/tool_capsules/spec.rs`. FileSystem is for the files it searches, which the std
layer opens through vfs, and vfs serves only a holder of it. No Network, Debug, graphics or
device bit.

Service `service:4820:tool.ripgrep`, reply `reply:4821:endpoint.tool.ripgrep.reply`.

## How it runs

The kernel mirror `src/userspace/capsule_ripgrep` embeds the ELF, certificate, manifest and
trailer under the kernel feature `nonos-capsule-ripgrep`. With that feature, init spawns it
once at boot through the verified path with the sandbox set (`src/userspace/init/entry.rs`).
The terminal's tool table does not list `rg`.

Handbook: [libc and std](../../docs/handbook/userland/libc-and-std.md),
[capsule catalogue](../../docs/handbook/apps/capsule-catalog.md).
