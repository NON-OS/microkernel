# fs_proofs

Host-runnable proofs for the filesystem logic, the largest proof crate.
Each `#[path]` include pulls in the real source: the vfs capsule's store,
block layer, protocol and handlers (`../capsule_vfs/src/`), the file
manager's logic and formatting, app_skeleton's sidecar codec, the desktop
shell's vfs listing helpers, the kernel's blockfs, blockfs volume and
symmetric crypto (`src/fs/`, `src/crypto/`), and `capsule_ramfs`, built as
a crate of its own in `ramfs_host/`.

`libc_shim/` stands in for `nonos_libc`: its store calls answer as the
kernel's do, over a disk held in memory, so the vfs block layer runs
unchanged. `reference/` holds the Python Argon2 reference the vectors come
from.

What the tests cover, by area:

- vfs store: paths, budgets (`vfs_budget_tests`), per-client shares
  (`vfs_fd_share_tests`, `store_owner_tests`), the FileSystem gate
  (`vfs_gate_tests`), reaping of ended Linux runs' private files
  (`vfs_private_reap_tests`), the journal, search, tags, favourites and
  sidecars.
- The capsule store on disk (`vfs_disk/`, `store_patch_tests`,
  `store_replace_tests`): load, append, remove, damaged entries left out,
  corrupt tables, names, and removals and appends cut short.
- The kernel's blockfs (`blockfs_tree/`, `blockfs_dir/`) and the disk plan
  (`data_plan/`).
- The crypto the sealed volume uses, against vectors and an earlier
  ChaCha20-Poly1305 (`crypto/`).
- ramfs through its request handlers (`ramfs/`).
- `fuzz_tests` for the decoders. `kani_proofs.rs` holds eight Kani
  proofs, run by the `kani` job in `.github/workflows/verify.yml`.

Run: `cargo test` in this directory, and `cargo kani` for the Kani proofs.
Storage is described in [Storage](../../docs/handbook/storage.md).
