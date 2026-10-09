# upstream-src

Source of third-party crates, vendored so NONOS can build them from a path. Cargo honours a
`[patch.crates-io]` block only in a path build, so a tool that needs one of its dependencies
replaced by a NONOS-aware copy is built from here with `cargo install --path . --locked`
(`mk/20-build.mk`), and `nix build` builds the same from the same lock with `--frozen`
(`tools/nix/capsules.nix`). The programs' own sources are the crates.io sources.

## The tools built from here

| Directory | Built as | Patches it applies |
| --- | --- | --- |
| `grex` | `capsule_grex` | none |
| `dotenv-linter` | `capsule_dotenv-linter` | none |
| `jsonxf` | `capsule_jsonxf` | none |
| `pastel` | `capsule_pastel` | getrandom, atty |
| `tokei` | `capsule_tokei` | home |
| `huniq` | `capsule_huniq` | atty, os_str_bytes |
| `csview` | `capsule_csview` | errno |
| `sd-1.0.0` | `capsule_sd` | is-terminal (`../vendor/is-terminal`), errno, in `.cargo/config.toml` |
| `tokio-smoke` | the runtime gate `target/upstream-tokio-smoke/tokio-smoke` | mio, socket2, tokio (`../vendor/`), in `.cargo/config.toml` |

`NONOS_TOOL_BINS` in `mk/20-build.mk` names the first seven; one template rule builds each into
`target/upstream-<name>/bin/<name>`, linked against the nonos-rt start object, with the RDRAND
backend for getrandom. grex and tokei are built with `--features cli`, the rest with
`--no-default-features`. sd and tokio-smoke have rules of their own.

## The patched dependencies

`atty-0.2.14`, `errno-0.3.14`, `getrandom-0.2.17`, `home-0.5.12` and `os_str_bytes-6.6.1` are
crates.io releases with a NONOS arm added, the copies the patch blocks above point at.
`ctrlc-3.5.2`, `dirs-6.0.0`, `fd-lock-4.0.4` and `is-terminal-0.4.17` carry a NONOS arm as well;
no patch block in this directory names them at this commit.

## Other directories

- `tools-2048`: the game logic `userland/capsule_game_2048` depends on by path.
- `choose`: patches atty, but no make rule or flake derivation builds it.
- `softbuffer`: holds only a `Cargo.lock`.

## Updating a tool

Replace the directory with the new crates.io release, keep the patch block, regenerate the
`Cargo.lock` against the patched crates, and commit the lock: `--locked` and `--frozen` build
exactly what it names. The handbook covers the std layer these build against:
[`docs/handbook/userland/libc-and-std.md`](../../docs/handbook/userland/libc-and-std.md).
