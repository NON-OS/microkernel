# nonos_git

`nonos_git` is git for the NONOS terminal, written from scratch and `no_std`:
the object model named by SHA-1, the index, refs, packfiles in both
directions, the smart HTTP wire protocol, and clone, fetch and push. It is a
library with no capsule, service or capability word of its own; its one user
is `capsule_terminal` (`Cargo.toml` there), and it runs with the terminal's
authority. The terminal's `git` command is described in
[docs/handbook/apps/terminal.md](../../docs/handbook/apps/terminal.md).

## What is in it

- `object`, `oid`, `sha1`, `zlib`: framing an object, naming it by the SHA-1
  of its framed bytes, and the zlib streams git stores them in.
- `odb`, `tree`, `commit`, `index`, `refs`, `config`: the loose object store,
  trees, commits, the index file, `HEAD` and refs, and the remote URL.
- `pack`: reading and writing packfiles and their index, with deltas.
- `wire`, `remote`: pkt-line framing, ref discovery, and `clone`, `fetch` and
  `push` over the smart HTTP protocol.
- `repo`: the porcelain the terminal calls (`init`, `add`, `commit`, `log`,
  `checkout`, `clone_into`).

The crate opens no file and no socket. It asks a `Storage` to read and write
paths and a `Transport` to answer a GET or a POST. The terminal passes a
`Storage` backed by vfs and a `Transport` that speaks HTTPS over the network
the person chose (`capsule_terminal/src/git/`); the tests pass a real
directory, replayed server bytes, or a local `git upload-pack` and
`git receive-pack`.

## Tests

73 `#[test]` functions under `tests/`. Where `git` is installed on the host,
the tests run real git against what this crate wrote (a repository, an index,
a pack) and against a `git receive-pack` this crate pushes to, so the format
is checked against git rather than against itself. Other tests cover damaged
and hostile input: truncated or corrupt packs and indexes, delta and inflate
limits, rejected trees and refused pushes.

```sh
cd userland/nonos_git
cargo test --release
```

No CI job names this crate.

## What it does not do

- Only HTTPS remotes through the terminal's transport; no SSH and no `git://`.
- No merge and no rebase.
