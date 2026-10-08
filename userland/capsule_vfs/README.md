# capsule_vfs

## Role

`capsule_vfs` is the desktop's file store, the `vfs_pool` service. it holds
every file the desktop and its apps see in its own heap, as a flat table of
named records, and it is the one capsule that reads and writes the capsule
store on disk. files live in RAM and vanish at power off unless they are
written to the store. the handbook page is
[Storage](../../docs/handbook/storage.md).

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

## Microkernel contract

- `MkIpcRecvFrom` (`mk_ipc_recv_from`) receives requests on
  `service:4104:vfs_pool` and hands back the sender pid. replies go to an
  ordinary caller with `MkIpcReply` (`mk_ipc_reply`), and to the kernel's own
  client (sender pid 0) with `MkIpcSend` (`mk_ipc_send`) to the reply endpoint
  `0x1_0000_0005`, declared as `reply:4105:endpoint.4294967301`.
- `MkCapCheck` (`mk_cap_check`) asks the kernel, on every request, whether the
  sender holds FileSystem.
- `MkStoreRead` and `MkStoreWrite` (`mk_store_read`, `mk_store_write`) read and
  extend the capsule store.
- `MkServiceLookup` (`mk_service_lookup`) finds the `installer` and `policy`
  services for the install and persistence gates.
- `MkPidAlive` (`mk_pid_alive`) finds handles and private files of clients
  that ended. `MkDebug` writes one status line after seeding; `MkExit` ends
  the process if the heap cannot be set up.
- the kernel mirror is `src/fs/vfs_capsule`, which embeds the signed ELF when
  built with `nonos-capsule-vfs` and spawns it with `spawn_vfs_capsule`.

## Interface contract

ops 1 to 26, in `src/protocol/types.rs`: open, close, read, write, stat,
list, healthcheck, mkdir, unlink, rename, rmdir, copy, truncate, usage,
chmod, seek, the five store ops (`OP_STORE_PERSIST`, `OP_STORE_REMOVE`,
`OP_STORE_STATUS`, `OP_STORE_INSTALL`, `OP_STORE_UNINSTALL`), dirstat,
journal touch and list, search, and generation.

every request carries the caller's pid first; it is kept only when it matches
the sender the kernel stamped, or when the sender is pid 0, the kernel's own
client. ordinary writes refuse the `/capsules` tree.

## Authority

the manifest grants `IPC` (`0x08`), `Memory` (`0x10`) and `StoreWrite`
(`0x4000000`): `CAPSULE_REQUIRED_CAPS = 0x04000018`. `Debug` is the optional
`0x100`, folded in only by a `capsule-serial-debug` build through the mirror's
`serial_debug_cap()`; with it, one line after seeding says how the store came
up. it has no Keyring (the data volume opens with the machine key), and no
driver, MMIO, IRQ, DMA, PIO, network, admin or direct block-device authority.

it answers the kernel's own client (sender pid 0) and any sender the kernel
says holds FileSystem, and refuses every other request with `EACCES` before
any handler runs (`src/server/fs_gate.rs`). the kernel is asked with
`MkCapCheck` on each request; no verdict is kept.

## Privacy and persistence

files are held in the capsule's 320 MiB heap and are gone at power off. a file
reaches the disk only through `OP_STORE_PERSIST`, which the capsule allows only
when the policy service says `Field::Persistent` is true (an all-zero payload,
which withdraws a record, is always allowed), and only for a file the caller
created. app installs reach it through `OP_STORE_INSTALL`, from the registered
`installer` only. unlinked files are wiped before their memory is freed
(`src/store/fdtable/zeroize.rs`).

## Runtime lifecycle

1. `_start` sets up the 320 MiB heap (`VFS_HEAP`), enough for the store loaded
   whole plus a replaced file's new bytes and the request buffers; the kernel
   backs a page only when it is first touched, so this reserves address space,
   not RAM a small store never uses.
2. `run` seeds the tree (`/docs`, `/tmp`, `/capsules`, `/home/nonos/...`, a
   readme and sample files), writes one `[VFS] serving, store status NN` line,
   and serves.
3. on idle polls a resumable load stages the store: header, table, then each
   payload, checking each entry's digest. a damaged entry is left out and
   counted rather than failing the whole load. staged entries appear under
   their paths, owned by pid 0.

## Failure model

- store status codes: 0 loaded, 1 no NONOS disk, 2 transport, 3 short read,
  6 over budget, 7 refused read, 8 bad request, 9 bad container or a load that
  left damaged entries out, 10 refused overwrite, 11 no memory, 12 no room. the
  desktop shell raises "capsule store corrupted" for 3, 6 and 9.
- store errors reach callers as `ENOSPC` (no room), `ENODEV` (no disk), `EIO`
  (a disk that failed) or `EUCLEAN` (a store that does not decode).
- file-table errors map to `ENOENT`, `EBADF`, `ENOSPC`, `EACCES`, `EEXIST`,
  `ENOTEMPTY`, `EISDIR` and `EINVAL`; a request longer than the receive buffer
  gets `EMSGSIZE`.

## Current implemented surface

the 26 ops, the FileSystem gate, the in-RAM file table with per-owner shares
(`src/store/fdtable`), the resumable store loader and ordered store writer
(`src/blk`), the handle reaper, the journal, search and the change-generation
counter. a build with the `seed-audio-store` feature also seeds `/audio` with
the test tone; no audio ships in the image otherwise.

## Wire format

magic `0x4E4F_5646` ("NOVF"), version 1, a 20-byte little-endian header: magic
`u32`, version `u16`, op `u16`, flags `u16`, two pad bytes, `request_id` `u32`,
`payload_len` `u32` (which is `4 + body`). a reply reuses the header, then a
4-byte `i32` status, then the body (`src/protocol/encode.rs`). a reply record
name carries a `u8` length prefix, so a name normalized to 256 bytes
(`MAX_WIRE_NAME` is 255) is skipped rather than truncated into a desync.

## State ownership

the file table and the open-handle table are the capsule's alone, in its heap;
no other process touches them. every file is owned by the pid that created it,
except the seed, the kernel's files and the packages staged from disk, which
are owned by pid 0. the on-disk store is the capsule's to write; the sealed
data volume is the kernel's (`src/fs/blockfs_volume`), not this capsule's.

## Operating rules

- hold one owner to `PER_OWNER_FDS` (128, half of `MAX_OPEN_FDS` 256) open
  handles, so no client can refuse every other open on the machine.
- hold one owner to `NAMES_PER_OWNER` (512, a quarter of `MAX_FILES` 2048)
  names, one file to `MAX_FILE_BYTES` (64 MiB), and every file together to
  `DATA_BYTES_MAX` (160 MiB), reserved fallibly so a large write is refused
  with `ENOSPC` instead of aborting the store (`src/store/fdtable/budget.rs`).
  pid 0 is held to none of these.
- close the handles of a client that ended without closing them: looked for
  every two seconds while requests arrive, and at once when an open finds no
  room (`src/server/reap.rs`, `src/store/fdtable/reap.rs`), using `MkPidAlive`.
  the kernel's own handles (owner 0) are never taken.
- remove the `/linux-private/` files of a Linux run that ended without
  removing them, on the same sweep.
- store writes are ordered so a power cut leaves the old table or the new one:
  an append places every payload (`MAX_WRITE_BYTES` 8192 at a time) before
  writing anything and commits the header sector last; a removal copies the
  last descriptor over the dropped one and lowers the count last
  (`src/blk/store_write.rs`, `src/blk/store_room.rs`, `src/blk/store_drop.rs`).

## Release target

0.9.2.

## Release evidence

`userland/fs_proofs` runs the real store, block layer, protocol and handlers on
the host, including the per-owner shares, the budget, the FileSystem gate,
reaping, and appends and removals cut short. `userland/nonos_disk_map` holds
the store's on-disk layout and its tests. build and sign with
`make nonos-mk-vfs`, `nonos-mk-vfs-sign`, `nonos-mk-vfs-verify`.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x04000018`, `Debug` optional only.
- [ ] the FileSystem gate refuses a sender without FileSystem that is not pid 0.
- [ ] `userland/fs_proofs` passes, including budget, reaping and torn writes.
- [ ] store append and removal leave the old or the new table on a power cut,
      never a half-written one.

## Explicit non-goals today

- the store appends new names and replaces a payload only with one of the same
  length; a changed length under the same name gets `EEXIST`.
- a removal frees no disk space; only a later replacement reuses gaps.
- the on-disk store holds `MAX_ENTRIES` (512) entries, `MAX_TOTAL_BYTES`
  (96 MiB) loaded whole into the heap, and `STREAMED_MAX_BYTES` (20 MiB)
  streamed from the device (the wallpaper collection).
- no journaling filesystem and no encryption at rest in this capsule; the
  sealed data volume is the kernel's.

## Verification

`src/protocol/types.rs` fixes the ops, magic and limits; `src/protocol/encode.rs`
fixes the reply layout; `src/store/fdtable/budget.rs` fixes the shares and the
160 MiB cap; `userland/nonos_disk_map/src/container.rs` fixes the on-disk
container (`NONOSTR1`, LBA 256, 512 entries, 96 and 20 MiB). every number,
op, endpoint and cap above is set in those files, and `userland/fs_proofs`
exercises the store, the gate and the torn-write ordering end to end on the
host.
