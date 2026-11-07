# capsule_user_files

## Role

The user-files broker, the `user_files` service. It keeps the person's
Documents, Downloads, Music, Pictures and Videos on the encrypted data volume and answers every open,
read, write, rename and listing of them. The logic is
[`nonos_user_files`](../nonos_user_files/README.md); this capsule is its IPC
loop and the four kernel calls under it.

```text
native app (FileSystem)        capsule_linux (ForeignExec, for a
        |                      program whose policy grants FILES_USER)
        |  NOUF requests, service:4424:user_files
        v
capsule_user_files -- tree, chunks, commits (nonos_user_files)
        |
        | MkUserObjPut / Read / Size / Drop, objects by number
        v
data volume, /u/<low byte>/<number> (src/fs/blockfs_volume/user_obj)
```

## Microkernel contract

- `MkIpcRecvFrom` on port 4424, waiting at most 100 ms so an owed commit is
  never late by more; `MkIpcReply` answers the sender.
- `MkCapCheck` on every request: the sender must hold FileSystem or
  ForeignExec, else EACCES before its request is read. No verdict is kept.
- `MkPidAlive` every two seconds: a client that ended has its files closed.
- `MkUserObjPut`, `MkUserObjRead`, `MkUserObjSize`, `MkUserObjDrop`.
- `MkTimeMillis` for file times, `MkUptimeMs` for when a commit is due.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x10000000018`: IPC, Memory and UserFiles. No
Debug, no FileSystem, no StreamImport, no network, no Keyring. UserFiles
is this capsule's alone: it is kept out of the ambient set and out of every
nsh ceiling.

## Privacy and persistence

Everything is on the data volume: on an installed machine with keeping on,
across reboots, sealed under the machine's key; on a live boot, or one that
keeps nothing yet, in the volume's RAM copy, gone at power off. The broker
writes no log line about a file. ZeroState wipes what it holds in RAM.

## Release evidence

`userland/user_files_proofs` runs the broker's logic on the host (see its
README). The capsule's own build: `make nonos-mk-user-files`. Signing and
enrolment need its publisher keys (`user_files_publisher_ed25519`,
`user_files_publisher_mldsa65`), made at the key ceremony.
