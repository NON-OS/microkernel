# NONOS Userland

Everything that runs at CPL=3 is built from this tree: the capsules, the
libraries they share, the host proof crates that test kernel and capsule code,
and the Linux userland the personality runs. The kernel loads a capsule only
after its certificate, signed manifest and STARK trailer verify, and gives it
the capability word the manifest declares.

## What is here

| Kind | Where | Count |
|---|---|---|
| Capsules with a `Capsule.mk` | `capsule_*/`, `tool_install/` | 95 `Capsule.mk` files, all included by `mk/20-build.mk` |
| The enrolled set | `../tools/nix/capsules.json` | 102 entries: the 95 above and the seven Linux userland programs `linux_userland/Userland.mk` declares |
| Host proof crates | `*_proofs/` | 90 crates, each with its own `Cargo.lock` |
| The userland libc | `libc/` | the crate almost every capsule links |
| The native runtime | `nonos_runtime/`, `nonos_abi/`, `nonos_alloc/`, `nonos_ipc/`, `nonos_service/`, `nonos_cap/`, `nonos_log/`, `nonos_panic/` | a second, smaller stack; no capsule uses it yet |
| The SDK | `sdk/` | app crates on the native runtime and `nonos_std`; no capsule uses them yet |
| Shared libraries | `app_skeleton/`, `nonos_socket/`, `nonos_route_link/`, `policy_client/`, `policy_proto/`, `nonos_tls/`, `nonos_disk_map/` and the other `nonos_*` crates | code more than one capsule links |
| The Linux userland | `linux_userland/`, `linux_guests/`, `upstream-src/` | the programs the Linux personality runs, and vendored crates.io sources |
| Host tooling | `platform/` | a host CLI for packaging a capsule outside the build |
| Target specs | `x86_64-nonos-user.json`, `aarch64-nonos-user.json`, `riscv64-nonos-user.json` | |

## The target

Every x86_64 capsule builds for `x86_64-nonos-user.json`: vendor `nonos`, os
`none`, `panic-strategy` abort, SSE and SSE2 only, linked by `rust-lld` as a
static position-independent executable with `-nostdlib -pie --gc-sections` and
4 KiB pages. The vendor string is what the std platform layer in
`toolchain/nonos-std` keys on.

## Building a capsule

A capsule is declared once, in its `Capsule.mk`: slug, binary, handle,
namespace, endpoints and capability word, then `include nonos-mk/capsule.mk`.
`mk/60-nix.mk` prints those declarations as `../tools/nix/capsules.json`, and
the flake builds each capsule from it in its own derivation:
`cargo build --release` for the target with `-Zbuild-std`, offline, against the
capsule's own lock (`../tools/nix/capsules.nix`). `nix flake check` fails when
the JSON is stale; `python3 tools/nix/catalogues.py` regenerates it. The seal
then signs every capsule and enrolls the whole set under one STARK root.

[Adding a capsule](../docs/handbook/extending/capsule.md) walks the steps. The
capability bits are in `../abi/caps.toml`.

## The runtimes

- `libc/`: `no_std`, the kernel ABI as `mk_*` functions, a zero-on-free heap
  and a panic handler that logs the message and exits with 134.
- `nonos_runtime/` and its crates: a separate stack with its own syscall
  trampoline, used only by `nonos_examples/` and `sdk/`.
- `toolchain/nonos-std` (outside this tree): real `std` for programs built from
  unmodified Rust, with `toolchain/nonos-rt` as the start object.

[The libc and std page](../docs/handbook/userland/libc-and-std.md) covers all
three.

## Proof crates

Each `*_proofs` crate mounts real kernel or capsule source by `#[path]` and
tests it on the host with `cargo test`. `nix flake check` runs every one of
them, with overflow checks on and clippy, and fails a live TPM suite that
skips a test. [The proofs page](../docs/handbook/verification/proofs.md) lists
them.

## Rules for this tree

- Files stay small and do one thing; `mod.rs` declares and re-exports.
- Syscall numbers and capability bits come from the kernel (`abi/caps.toml`,
  `src/syscall/numbers`). Userland mirrors them and never assigns its own.
- A capsule gets the capabilities its signed manifest declares and nothing
  else. Asking for more at spawn than the manifest allows is refused.
