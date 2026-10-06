# capsule_ramfs

## Role

`capsule_ramfs` is the volatile in-memory file capsule. It provides a small
file namespace to other capsules without putting filesystem policy back into
the kernel and without touching persistent storage. In the kernel,
`is_capsule_path` routes `/ram` and below to it, so a process's `/ram` files
are opened through the kernel's own client (owner 0). Each file is sealed
in memory under its own key. The handbook page is
[Storage](../../docs/handbook/storage.md).

```text
client capsule
    |
    | file IPC
    v
ramfs -- in-memory store --> volatile file bytes
    |
    `-- reply over IPC
```

## Microkernel contract

The capsule is a normal IPC service:

- `MkIpcRecv` receives requests on `service:4096:ramfs`.
- `MkIpcSend` replies on the caller's reply path.
- `CryptoRandom` draws each file's key and nonce; `CryptoEncrypt` and
  `CryptoDecrypt` seal and open its bytes with ChaCha20-Poly1305.
- `MkPidAlive` finds handles of callers that ended.
- `MkExit` is used for fatal startup failure.
- The kernel-side mirror is `src/fs/ramfs_capsule`, which embeds the signed
  ELF in the kernel and spawns it with IPC, Memory and Crypto.

The kernel routes IPC and schedules the process. It does not keep file tables,
file bytes, directory policy, or ramfs mutation logic.

## Interface contract

| Surface | Purpose |
|---|---|
| file protocol | five ops, `OP_OPEN` (flags `OPEN_FLAG_CREATE`, `OPEN_FLAG_TRUNCATE`), `OP_CLOSE`, `OP_READ`, `OP_WRITE`, `OP_TRUNCATE` |
| handle table | runtime mapping of caller handles to in-memory records |
| IPC reply | returns bytes or deterministic errno values |

## Authority

The manifest grants `IPC`, `Memory` and `Crypto`
(`CAPSULE_REQUIRED_CAPS = 0x38`). Crypto is for the random keys and nonces and
the AEAD calls. It has no driver, MMIO, IRQ, DMA, PIO, network, admin, debug,
or persistent-storage authority.

## Privacy and persistence

All file bytes are memory-resident, sealed per file with a fresh random
key; every write opens the whole file, patches it, and seals it again under a
fresh nonce. They vanish when the capsule exits or the
system reboots. The capsule does not write to disk. Clients must treat the
service as volatile scratch state, not durable storage.

## Runtime lifecycle

The capsule starts empty, serves file requests from memory, updates its handle
and content tables, and drops all state when the process exits.

## Failure model

Invalid handle, missing file and invalid payloads return protocol errors
(`EINVAL`, `ENOENT`, `EACCES`, `EMFILE`, `EIO`). A write or truncate past
`MAX_FILE_BYTES`, 2 MiB, gets `EFBIG`; one that would take every file past
`MAX_STORE_BYTES`, 8 MiB, or a create past `MAX_FILES`, 1024, gets `ENOSPC`.
Both are checked before anything is allocated, so a caller-chosen offset
cannot run the 16 MiB heap out (`src/store/limits.rs`). A frame the decoder
refuses is answered with `EINVAL` under sequence number 0. No kernel fallback
filesystem is invoked.

## Current implemented surface

- Maintains an in-memory file store.
- Handles open/read/write/truncate-style operations through its protocol.
- Keeps handles and file content in capsule-owned memory.
- Has a kernel mirror that embeds and spawns it.

## Wire format

The ramfs wire protocol is an IPC request/reply envelope carrying operation id,
handle/path metadata, byte ranges, and payload bytes. Replies return either
data bytes or a deterministic errno value. The exact layout lives in
`src/protocol`.

## State ownership

The capsule owns file records, handle records, and file bytes. VFS owns
descriptor routing above it. The kernel owns no ramfs bytes and no ramfs handle
table.

## Operating rules

- Treat all state as volatile.
- Bound memory growth and reject requests that exceed capacity.
- Return explicit errors for invalid handles and malformed requests.
- Hold one caller to `PER_OWNER` handles, half the table, so no caller can leave the kernel
  unable to open any process's /ram file; past it an open answers `EMFILE`.
- Close the handles of a caller that ended without closing them: looked for every two seconds
  while requests arrive, and before any request when the table is full (`src/handles.rs`,
  `src/server/reap.rs`). The kernel's own handles (owner 0) are never taken.
- Never add block-device persistence here.

## Release target

The finished ramfs capsule has bounded memory accounting, deterministic handle
lifetime, complete error mapping, validation coverage for create/read/write/truncate,
and clear teardown semantics. It remains volatile by design and never becomes
a disk filesystem.

## Release evidence

Release evidence is the ramfs validation check covering create, read, write,
truncate, invalid handle, and teardown with no kernel-resident file state.

## Release checklist

- Validation covers create/read/write/truncate/delete or equivalent flows.
- Invalid-handle and malformed-request errors are covered.
- Memory bounds are documented and enforced.
- Kernel has no ramfs byte store.

## Explicit non-goals today

No disk backing, journaling, fsck, permissions database, encryption-at-rest,
mount stack, block driver, or persistent namespace is implemented here.

## Verification

- Build: `make -B nonos-mk-ramfs`; sign: `nonos-mk-ramfs-sign`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Host proofs: `userland/fs_proofs` (`src/ramfs/tests_handles.rs` holds the per-caller share and
  the closing of ended callers' handles on the real table; `tests_limits.rs` the size
  bounds; `tests_wire.rs` the request decode).
- Architecture check: filesystem mutation logic must stay in userland.
