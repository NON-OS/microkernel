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
- `attest*.rs`, `origin.rs`: proving a program before it runs. `app/`: the policy it is
  bound to: the vendor's (`policies.txt`, compiled into `nonos_linux_policy::vendor`), a
  vouched file beside it, or the owner's default.

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
#   | ForeignExec 0x100000000 = 0x104001979
CAPSULE_REQUIRED_CAPS    := 0x104001979
CAPSULE_OPTIONAL_CAPS    := 0x600000004
```

The comment says no other capsule holds ForeignExec, and that Network (0x4), LocalSign
(0x200000000) and StreamImport (0x400000000) are optional: the install role asks for all three,
StreamImport so it may start the model fetcher within its own capabilities, and the run.net role
for Network alone, so no run of a guest holds the right to vouch or to feed the data volume. Extra instance endpoints for the install,
run, run.net and terminal roles are listed in `CAPSULE_INSTANCE_ENDPOINTS`.

## Interface

Service `service:4936:app.linux`, reply `reply:4937:endpoint.app.linux.reply`, plus the
instance endpoints above. Arguments: `install <name> <hash>` or `run <name> [cli]`
(`src/linux/run_mode.rs`).

The installer's exit code is its reason (`install/why.rs`), which the store shows in words.

## Boxed runs

The terminal's `linux --box <box> <program> [args]` runs the program held to a named box
(`src/linux/boxed/`, armed in `terminal_box.rs`):

- `strict`: no network; only the run's own `/tmp` is written.
- `offline`: no network; the image is read-only, and the run's own folders (`/tmp`, `/home`,
  `/root`, `/run`, `/var/tmp`, `/dev/shm`) are written.
- `allow:<folder>,...`: only the folders named, each inside those, are written. A destination
  item (`net:<IPv4 or name>:<port>`) lets the run open TCP streams to it, over the anonymity
  network, and to nothing else. Such a box runs only in the terminal role that holds Network
  (`app.linux.term.net`), which runs no other box (`boxed/terminal_run.rs`); the report names
  each destination reached (`boxed/connected.rs`, `boxed/net_report.rs`).

Each box grants a terminal and execve; every program the run execs is held to it too. The box
narrows the program's own policy (`app/identity_box.rs`) and never widens it, but for one case:
a program with no policy of its own (the owner's default) gets NET_ANON from a box naming
destinations, held to exactly that list, and so does each such program the run execs. A
program whose published policy grants no network has none in any box. Each box has one hash,
BLAKE3 under `NONOS.linux.box.v1` of its canonical form (`boxed/hash.rs`).

At the end the run sends a `BOX REPORT v1` to the terminal that started it: the box and its hash,
what the program actually had (the box narrowed by its own policy), each kind of refused action
with its count and first three examples, whether it started, and the exit status, followed by one
`[BOX] v=1 preset=... policy=... refused=N <kind>=N ... started=yes|no exit=N source=capsule_linux`
line. The terminal names the run a key ahead of its words (`--box-key=<hex>`), and every report
frame opens with `TAG_BOX` and that key (`boxed/frames.rs`), so the guest, which writes the same
channel and never sees the key, cannot forge the report; the terminal draws it after the run's
output ends and writes the `[BOX]` line to the serial log. A full inbox is waited on, not dropped.
A run that stops before its guest starts still reports why (`start_stop.rs`). A refused action is
counted once for each kind per call. The counts are this capsule's enforcement, not the kernel's
sealed histogram, which counts by errno.

The program's app_id is noted to slot 0 of the kernel's seal before its guest starts
(`seal_note.rs`, I-C6), so a family running a proved program is kept to be proved; the
capsule's own BusyBox, whose app_id no verifier takes, is not noted.

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

net.sockets and net.anon both take Network to reach. The run and terminal roles are spawned
without it, so there neither service answers a guest. A run whose program's policy has
NET_ANON asks the kernel, by `MkLinuxRunNet` naming its own pid, to be started again as
`app.linux.run.net`, which holds Network, binds the same policy again and exits unless it has
NET_ANON (`src/linux/run_net.rs`, `src/userspace/capsule_linux/run_net.rs`). That role answers
no requests: its inbox takes only input_router's events, checked by sender. The install role
holds Network as well (`src/userspace/capsule_linux/roles.rs`).

A policy whose `net_dests_hash` names a destination list reaches only what the list allows,
the address or the name the guest looked up, on the ports it names (`app/reach.rs`,
`nonos_linux_policy::dests`). After an exec it is held to every list on the way, two at most;
a third takes NET_ANON away. Only a vendor row carries its list today (`dests=` in
`policies.txt`); a policy naming a list this capsule does not hold is served without NET_ANON.

## Wayland

`src/linux/wayland/` is the server a guest window talks to. What it does, plainly:
- A window is shown whole and takes input everywhere, so a region (`region.rs`) and a
  surface's opaque and input regions, transform and scale change nothing. They are still
  objects a client can name, destroy and reuse.
- Only a toplevel's commit reaches the screen (`commit.rs`). Subsurfaces are kept as
  objects; their pixels are not composed onto the window yet.
- An XRGB8888 buffer is shown opaque (`opaque.rs`); only ARGB8888 and XRGB8888 are taken.
- NONOS carries no clipboard or drag between guests, or to the desktop, by design: a guest
  cannot read what another program copied. `data_device.rs` gives a client its sources and
  a device per seat, and cancels any selection or drag at once. The primary selection is served the
  same way (`route_select.rs`), and a cursor shape a guest asks for changes nothing: the
  cursor is the compositor's own.
- Decorations are always server-side (`decoration.rs`): the window manager frames a guest's
  window like any other.
- zxdg_output_manager_v1 says the one screen's logical place and size, as wl_output does.
  wp_viewporter makes viewports, but their source and destination change nothing: a buffer is
  shown at its own size (`route_output.rs`).
- xdg_activation_v1 issues tokens, done on commit, but activating changes nothing: a guest has
  one window and focus is the person's. A single-pixel buffer is a real wl_buffer, but only
  pixels from shared memory are shown, so a commit of one shows nothing (`route_misc.rs`).
- A popup is dismissed as soon as it is asked for (`route_rest.rs`): a guest's window is one
  surface, and a menu it opens is not shown. Moving, resizing and minimizing are the window
  manager's, which the person drives, so a guest asking does nothing. The cursor is the
  compositor's own, so a cursor surface is taken and never shown, and touch is never sent
  (`route_seat.rs`). Every object a request makes is made, so a request naming an id the
  client never made, or one already gone, is its own mistake: it is answered with
  wl_display.error (invalid_object), which ends the client, and nothing more from it is
  served (`display_error.rs`).
- Keys are sent as the evdev keys of a US board, with the xkb modifier masks (`evdev.rs`,
  `xkb_mods.rs`); buttons as BTN_LEFT onward, the wheel as axis events (`pointer_codes.rs`).
  Other layouts need the scancode in the input event. No leave events are sent: the router
  sends input only while the window has focus.

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
