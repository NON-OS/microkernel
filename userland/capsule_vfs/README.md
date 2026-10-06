# capsule_vfs

## Role

`capsule_vfs` is the desktop's file store, the `vfs_pool` service. It holds
every file the desktop and its apps see in its own heap, as a flat table of
named records, and it is the one capsule that reads and writes the capsule
store on disk. Files live in RAM and vanish at power off unless they are
written to the store.

```text
application capsule (app_skeleton::clients::vfs, std::fs through nonos_rt)
    |
    | VFS IPC, service:4104:vfs_pool
    v
vfs_pool -- in-RAM file table, handles, journal, search
    |
    | MkStoreRead / MkStoreWrite (StoreWrite)
    v
capsule store on the NONOS disk (NONOSTR1 container at LBA 256)
```

The handbook page is [Storage](../../docs/handbook/storage.md).

## Microkernel contract

- `MkIpcRecv` receives requests on `service:4104:vfs_pool`; replies go out
  on `reply:4105:endpoint.4294967301`.
- `MkCapCheck` asks, on every request, whether the sender holds FileSystem.
- `MkStoreRead` and `MkStoreWrite` read and extend the capsule store.
- `MkServiceLookup` finds the `installer` and `policy` services for the
  install and persistence gates.
- `MkPidAlive` finds handles and private files of clients that ended.
- The kernel mirror is `src/fs/vfs_capsule`, which embeds the signed ELF
  in the kernel when it is built with `nonos-capsule-vfs` and spawns it
  with `spawn_vfs_capsule`.

## Interface contract

Ops 1 to 26, in `src/protocol/types.rs`: open, close, read, write, stat,
list, healthcheck, mkdir, unlink, rename, rmdir, copy, truncate, usage,
chmod, seek, the five store ops (`OP_STORE_PERSIST`, `OP_STORE_REMOVE`,
`OP_STORE_STATUS`, `OP_STORE_INSTALL`, `OP_STORE_UNINSTALL`), dirstat,
journal touch and list, search, and generation.

Every request carries the caller's pid first; it is kept only when it
matches the sender the kernel stamped, or when the sender is pid 0, the
kernel's own client. Ordinary writes refuse the `/capsules` tree.

## Authority

The manifest grants `IPC`, `Memory` and `StoreWrite`
(`CAPSULE_REQUIRED_CAPS = 0x04000018`), and `Debug` as the optional `0x100`, which
only a `capsule-serial-debug` build grants; with it, one line after seeding
says how the store came up. It has no driver, MMIO, IRQ, DMA, PIO, network,
admin, or direct block-device authority.

It answers the kernel's own client (sender pid 0) and any sender the kernel
says holds `FileSystem`, and refuses every other request with `EACCES` before
any handler runs (`src/server/fs_gate.rs`). The kernel is asked with `MkCapCheck` on
each request; no verdict is kept.

## Privacy and persistence

Files are held in the capsule's 192 MiB heap and are gone at power off.
A file reaches the disk only through `OP_STORE_PERSIST`, which the
capsule allows only when the policy service says `Field::Persistent` is
true (an all-zero payload, which withdraws a record, is always allowed), and
only for a file the caller created. App installs reach it through
`OP_STORE_INSTALL`, from the registered `installer` only. Unlinked files
are wiped before their memory is freed.

## Runtime lifecycle

1. `_start` sets up the 192 MiB heap; `run` seeds the tree (`/docs`, `/tmp`,
   `/capsules`, `/home/nonos/...`, a readme and sample files) and serves.
2. On idle polls a resumable load stages the store: header, table, then
   each payload, checking each entry's digest. A damaged entry is left out
   and counted rather than failing the whole load.
3. Staged entries appear under their paths, owned by pid 0.

## Failure model

- Store status codes: 0 loaded, 1 no NONOS disk, 2 transport, 3 short read,
  6 over budget, 7 refused read, 8 bad request, 9 bad container or a load
  that left damaged entries out, 10 refused overwrite, 11 no memory, 12 no
  room. The desktop shell raises "capsule store corrupted" for 3, 6 and 9.
- Store errors reach callers as `ENOSPC` (no room), `ENODEV` (no disk),
  `EIO` (a disk that failed) or `EUCLEAN` (a store that does not decode).
- File table errors map to `ENOENT`, `EBADF`, `ENOSPC`, `EACCES`, `EEXIST`,
  `ENOTEMPTY`, `EISDIR` and `EINVAL`.

## Operating rules

- Hold one owner to `PER_OWNER_FDS` open handles, half the table, so no client can refuse
  every other open on the machine.
- Hold one owner to `NAMES_PER_OWNER` names, a quarter of `MAX_FILES`, and
  every file together to `DATA_BYTES_MAX`, 160 MiB, reserved fallibly, so a
  large write is refused with `ENOSPC` instead of aborting the store
  (`src/store/fdtable/budget.rs`). Pid 0 is held to neither.
- Close the handles of a client that ended without closing them: looked for every two seconds
  while requests arrive, and at once when an open finds no room (`src/server/reap.rs`,
  `src/store/fdtable/reap.rs`). The kernel's own handles (owner 0) are never taken.
- Remove the `/linux-private/` files of a Linux run that ended without
  removing them, on the same sweep.
- Store writes are ordered so a power cut leaves the old table or the new
  one: an append places every payload before writing anything and commits
  the header sector last; a removal copies the last descriptor over the
  dropped one and lowers the count last (`src/blk/store_write.rs`,
  `src/blk/store_room.rs`, `src/blk/store_drop.rs`).

## Explicit non-goals today

- The store appends new names and replaces a payload only with one of the
  same length; a changed length under the same name gets `EEXIST`.
- A removal frees no disk space; only a later replacement reuses gaps.
- The store holds 512 entries and 96 MiB loaded whole into the heap, and
  20 MiB streamed from the device (the wallpaper collection).
- No journaling filesystem, no encryption at rest in this capsule. The
  sealed data volume is the kernel's (`src/fs/blockfs_volume`).

## Verification

- Build: `make nonos-mk-vfs`; sign and verify: `nonos-mk-vfs-sign`,
  `nonos-mk-vfs-verify`.
- Host proofs: `userland/fs_proofs` runs the real store, block layer,
  protocol and handlers, including the per-owner shares, the budget, the
  FileSystem gate, reaping, and appends and removals cut short.
- `userland/nonos_disk_map` holds the store's layout and its tests.
