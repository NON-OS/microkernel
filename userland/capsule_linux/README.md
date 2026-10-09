# capsule_linux

`capsule_linux` is the Linux personality. It runs unmodified x86_64 Linux binaries in
processes that hold no NONOS capabilities and answers every syscall they make. Per
`Cargo.toml`, the kernel holds no Linux knowledge: syscall numbers, structs, errnos and the
filesystem view all live in this capsule. It also installs Alpine, Debian/Kali and pacman
packages (`src/linux/install/`). The handbook page is
[`docs/handbook/linux/personality.md`](../../docs/handbook/linux/personality.md).

## Role

`src/linux/start.rs` either serves an `install <name> <hash>` request (`src/linux/request.rs`)
or picks a program, calls `mk_foreign_spawn`, prepares the family's private directories, loads
the image (`src/linux/image/`), and enters the serve loop. Main modules under `src/linux/`:

- `abi/`: numbers, errnos and names. `call/`: one file per family of calls.
- `serve/`: the loop, dispatch and per-area tables (`table_file.rs`, `table_net.rs`, ...).
- `guest/`: a hosted process: descriptors, memory, signals, links.
- `file/`: the guest's filesystem view over the store, path walk, epoll, eventfd, timerfd.
- `net/`: family-local sockets, and streams carried by `net.sockets` or `net.anon`.
- `unix/`, `wayland/`: the display connection and a Wayland server for guest windows.
- `console/`, `terminal*.rs`: terminal runs. `install/`: fetch, authenticate, unpack.
- `attest*.rs`, `origin.rs`: proving a program before it runs.

## How a guest syscall is served

`serve/loop_impl.rs` blocks in `mk_foreign_wait` (up to 250 ms) for a `ForeignFrame`. The
family (`serve/family.rs`) finds the owning guest and calls `serve/dispatch.rs::answer`, which
routes process, futex, sleep and blocking I/O calls first and sends the rest to
`serve/table.rs::plain`. The result is either `Answer::Reply(value)`, returned through
`mk_foreign_reply` unless a caught signal is delivered in its place (`serve/deliver.rs`), or
`Answer::Park`, which leaves the thread inside its syscall until a later
settle pass wakes it. A number nothing serves goes to `serve/unserved.rs`, which logs its name
and returns `ENOSYS`. Guest memory is read and written with `mk_peer_read` and
`mk_peer_write` (`guest/mem_copy.rs`).

## Capabilities

From `Capsule.mk`:

```make
# = CoreExec 0x1 | IPC 0x8 | Memory 0x10 | Crypto 0x20 | FileSystem 0x40
#   | Debug 0x100 | GfxQuery 0x800 | GfxCreate 0x1000 | StoreWrite 0x4000000
#   | ForeignExec 0x100000000 | LocalSign 0x200000000 = 0x304001979
CAPSULE_REQUIRED_CAPS    := 0x304001979
CAPSULE_OPTIONAL_CAPS    := 0x4
```

The comment says no other capsule holds ForeignExec, and that Network (0x4) is optional and
requested only by the install role. Extra instance endpoints for install, run and terminal
roles are listed in `CAPSULE_INSTANCE_ENDPOINTS`.

## Interface

Service `service:4936:app.linux`, reply `reply:4937:endpoint.app.linux.reply`, plus the
instance endpoints above. Arguments: `install <name> <hash>` or `run <name> [cli]`
(`src/linux/run_mode.rs`).

The installer's exit code is its reason (`install/why.rs`), which the store shows in words.

## Network

A guest's sockets stay inside the family, except a stream to an address outside
127.0.0.0/8. That stream follows the network the person chose, read for each connection the
way every capsule holding Network reads it: `nonos_route_link`'s pick over the policy store's
default network and the anonymity networks that run (`net/guest_route.rs`).

- Anyone: a net.anon stream, opened through its handle front (`net/connect_anon.rs`,
  `net/anon_*.rs`). A name the guest looked up goes to the exit unresolved, a literal address
  as its dotted quad, so nothing here looks a name up. With net.anon not running the connect
  is ENETUNREACH, the serial log says `[LINUX] refused connect <ip>:<port>: the Anyone
  network, the default, is not running`, and nothing else is tried.
- Nym, or a default that cannot be read: a net.sockets mixnet socket, as before. A mixnet
  with no gateway is ENETUNREACH.
- Direct: the mixnet as well. A guest never reaches the network directly, whatever the
  default. That is deliberate (`net/connect_out.rs`, `design/install-network.md`).

Datagrams outside the family stay ENETUNREACH, binds and listens outside loopback EACCES, a
raw internet socket EPERM (`net/policy.rs`), and any family but AF_INET and AF_UNIX, packet
sockets among them, EAFNOSUPPORT (`net/socket.rs`). net.anon has no half-close, so shutting only
the writing side of an Anyone stream is EOPNOTSUPP, as it is on the mixnet; shutting both
sides closes it. A connect over Anyone returns once net.anon has sent the exit its BEGIN, so
a far end that refuses is reported by the first read, as ECONNREFUSED. net.anon gives one
caller at most 16 streams and this capsule is one caller for all its guests: a connect past
that is ENOBUFS. net.anon cannot be asked whether bytes wait without taking them, so a poll
or read takes them into the socket's own buffer, which never holds more than one 32 KiB reply
(`net/sock/anon_rx.rs`).

net.sockets and net.anon both take Network to reach. Only the install role is spawned with it
(`CAPSULE_OPTIONAL_CAPS := 0x4`, `src/userspace/capsule_linux/roles.rs`); in the run and
terminal roles neither service answers a guest.

## Installing a shipped Qwen tier

A name in the `qwen-` namespace is never looked up in a package index
(`install/family.rs`, `apps.rs::wanted`). A shipped tier installs its program from the store
(BLAKE3 equal to the market's pin) and treats its model as a dependency
(`install/model_dep.rs`): a file the data volume holds under its pin is kept, one it lacks
is imported from the disk plan when the disk carries it, and otherwise the tier word
(`App::tier`) is handed to `tool.model-fetch` as `get <tier>` (`install/fetcher.rs`), its
output drained and never logged. The fetcher's exit status (`capsule_model_fetch/src/exit.rs`,
included by `#[path]`) and then each file's import record, which must be its pin, decide
the result: reasons 10 to 18 (no data volume, locked, no network, unknown tier, SHA-256
mismatch, download stopped, no room, another download running, file names too long for the
volume). A name in the namespace that is no shipped tier is reason 13. Proofs:
`userland/model_fetch_proofs`.

## State and privacy

- Guest paths are confined under the family root (`file/root.rs`); the shared tree is written
  by installs only, and each family has private directories cleared on exit (`start.rs`).
  They live in the store every capsule shares, so together they hold at most 16 MiB and 128
  names (`file/system/declared/sizes.rs`), ENOSPC past either (`file/system/space/quota.rs`).
- `net/offline.rs` refuses a new internet socket once a model is held, and refuses to open a
  model while an internet socket is open.
- A program read from the store is checked by `prove` in `start_guest.rs` before it runs, and
  refused if the check fails (`origin.rs`, `attest.rs`).
- Package keyrings are pinned at build time by `build.rs`; see `design/package-trust.md`.

## What a guest can make this capsule hold

Everything a guest buffers is this capsule's memory, and a guest is code nobody here wrote,
so each table and queue it can grow has a ceiling, refused with the errno Linux uses there:

- Tasks: 512 processes, threads and unwaited children per family, EAGAIN from fork and clone
  past it, reported as RLIMIT_NPROC (`call/spawn/tasks.rs`).
- Files being written: the family's copies together stay within the private quota, ENOSPC
  past it (`file/system/space/quota.rs`); a hard link never crosses mounts (EXDEV).
- Locks: 4096 records per family, ENOLCK past it (`file/locks/lock/room.rs`).
- Datagrams: each waiting one is charged its payload and an sk_buff's 576 bytes against
  SO_RCVBUF, as Linux charges it (`net/sock/gram_room.rs`).
- The display connection: past 1 MiB of unread events the server stops serving the client's
  requests, and past 1 MiB of waiting requests a write is EAGAIN (`unix/conn.rs`); a
  committed frame is at most 16 MiB and is asked of the heap, not demanded
  (`wayland/frame_len.rs`).

These bound each structure; together they still exceed the 64 MiB heap a run gets, so a guest
that fills many of them at once can end its own family.

## Build and test

- Build: `make nonos-mk-linux` (`nonos-mk/capsule.mk`); `build.rs` reads `NONOS_PACMAN_*` and
  `NONOS_DEB_*` settings.
- Host tests: `userland/capsule_linux_proofs`, and `userland/model_fetch_proofs` for the Qwen
  tier install. `nix flake check` runs each as a check (`tools/nix/checks.nix`), and
  `.github/workflows/verify.yml` runs `nix flake check`.

## Not done yet

- Uncovered calls return `ENOSYS`; `abi/x86_64-syscalls.txt` is the coverage denominator.
- `abi/wayland-wanted.txt` says its list is "not checked against a running client yet".
- Per `build.rs`, the pacman keyring is empty unless `NONOS_PACMAN_KEYRING` is set, so every
  pacman install is refused by default.
