# capsule_linux_proofs

Host test crate for the pure parts of the Linux personality in `userland/capsule_linux`. It
compiles shipped source files through `#[path]` and runs them against vectors in `vectors/`:
Alpine, Debian and Kali metadata, a dynamic ELF, tar archives and gzip input. The
Cargo package is `nonos_capsule_linux_proofs`; it links the same `nonos_inflate`,
`nonos_hash`, `nonos_xz`, `nonos_zstd` and `sha1` crates the capsule does, and uses `rsa` and
`nonos_openpgp` as dev-dependencies (`Cargo.toml`).

## What is under test

Mounted from `../../capsule_linux/src/linux/` (see `src/lib.rs` and the `mod.rs` files under
`src/calls.rs`, `src/install/`, `src/image/`, `src/console/`, `src/pinned/`):

- Wayland: `wayland/wire.rs` (with the walk over a batch of requests), `wayland/args.rs`,
  and the bytes a commit copies (`wayland/frame_len.rs`).
- Filesystem view: `file/root.rs`, `file/resolve.rs`, `file/family.rs`, `file/dirent.rs`,
  `file/dir_children.rs`, `file/meta/statbuf/mod.rs`, `file/models/*`.
- Process and signals: `call/sigframe*.rs`, `call/spawn/exec_shebang.rs`,
  `guest/sigtimer*.rs`, `image/elf.rs`, `image/phdr.rs`, `image/stack_words.rs`.
- Network: `net/host_body.rs`, `net/route.rs`; the path a guest's stream outside the family
  takes (`net/guest_route.rs`, over `nonos_route_link/src/pick.rs` as it ships); what is
  said to net.anon's handle front and read back (`net/anon_ops.rs`, `net/anon_wire.rs`,
  `net/anon_answer.rs`); and the part of the socket table such a stream goes through
  (`net/sock/{backend,anon_rx,adopt,shut_anon,free,table,types,...}.rs`, under
  `src/linux/net/sock/`), with net.sockets' and net.anon's close calls replaced by the
  recording doubles in `src/linux/net/stream.rs` and `src/linux/net/anon_stream.rs`; and
  what a waiting datagram is charged against its receiver's buffer (`net/sock/gram_room.rs`).
- Installer: `install/tar*.rs`, `install/index.rs`, `install/auth/*`, `install/deb/*`,
  `install/pacman/desc.rs`, `install/pgp/*`, `install/http_reply.rs`, `install/apps*.rs`.
- Console and run requests: `console/queue*.rs`, `console/wipe.rs`, `console/winsize.rs`,
  `run_mode.rs`.
- The call rules, under `src/linux/`, mounted at the crate paths they name in the capsule
  (`crate::linux::abi::errno`, `crate::linux::guest`) so each compiles as it ships: the
  argument checks of mmap, mprotect, mremap and munmap (`call/mem/*_args.rs`), futex's decoder
  and waiter choice (`call/futex_op.rs`, `guest/futex_pick.rs`), clone's flags
  (`call/spawn/clone_flags.rs`), the family's RLIMIT_NPROC (`call/spawn/tasks.rs`), wait4's
  and waitid's options (`call/spawn/wait_opts.rs`),
  the signal rules (`guest/sigalt.rs`, `guest/sigrestart.rs`,
  `call/signal_act.rs`), ioctl's request table (`call/ioctl_req.rs`), getrandom's flags, the
  iovec reader (`call/iovec.rs`), poll's and select's counts and sets (`net/ready_sets.rs`),
  epoll's rules (`file/epoll_rules.rs`), the span fallocate's PUNCH_HOLE zeroes
  (`file/calls/falloc/punch.rs`), the private directories' quota of bytes and names
  (`file/system/space/quota.rs`, mounted beside `declared` under `src/load/`), the mounts a
  hard link may not cross (`file/link/across.rs` over `made/proc/mounts/table.rs`), the
  resolver's limits and DNS name and answer
  (`net/dns/*`), the SCM_RIGHTS walk (`unix/cmsg.rs`), the display connection's two
  bounded queues (`unix/conn.rs`), the lock table's splitting and ceiling
  (`file/locks/lock/{apply,room,table}.rs`), the descriptor-table slots
  (`guest/slots.rs`), the link ceiling, the timerfd timer, the Wayland object table, and
  the record /proc keeps of each process's image (`file/made/exe/table.rs`).
  The calls that read guest memory (`sigaltstack`, the wait masks, the iovec reader, the
  string and path readers `file/cstr.rs` and `file/path.rs`, execve's argv and envp reader
  `call/spawn/exec_args.rs`) are written over `guest/memory.rs`'s `Memory` trait and run
  here over `tests/fake_memory.rs`.

It also mounts `src/userspace/capsule_linux/family.rs` and, for `tier_word_tests.rs`,
`src/userspace/capsule_linux/terminal/tier.rs` from the kernel tree; four
`capsule_net_sockets` handler files and its `protocol/errno.rs`, so the capsule's
connect-by-host encoder is checked against that service's parser; and net.anon's `protocol/ops.rs`, `protocol/errno.rs` and
`stream/end.rs`, so the capsule's client of its handle front uses net.anon's own values. `src/host_doubles.rs` replaces the private-directory and clamp
modules that make syscalls.

## What the tests check

Modules are listed in `src/tests.rs`. Examples:

- `resolve_tests.rs`: guest paths become store keys
  (`a_relative_path_hangs_off_the_working_directory`).
- `auth_tests.rs`, `auth_refusals.rs`: Alpine index and package signatures, and refusal of
  changed indexes or untrusted keys
  (`an_index_signed_under_a_key_not_trusted_here_is_refused`).
- `deb_chain_tests.rs`, `kali_anchor_tests.rs`: Release.gpg over Release over Packages over
  each .deb, and the pinned Kali key against a fetched kali-rolling Release
  (`kali_rolling_release_verifies_under_the_pin`).
- `sigframe_tests.rs`, `sigframe_layout_tests.rs`: a built signal frame reads back identically
  and matches `struct rt_sigframe` (`the_frame_is_linux_rt_sigframe_byte_for_byte`).
- `elf_tests.rs`, `tar_tests.rs`, `tar_link_tests.rs`, `wire_tests.rs`, `route_tests.rs`
  (`private_and_link_local_mirrors_go_direct`).
- `mutation_tests.rs`: seeded mutation of the Debian and pacman readers, which must return
  without a panic (`damaged_debs_never_panic`).
- `guest_route_tests.rs`: a guest's stream follows the chosen network over every default
  and every network up or down: Anyone to net.anon or ENETUNREACH, Nym, an unreadable
  default and Direct to the mixnet (`the_whole_table`, `direct_keeps_a_guest_on_the_mixnet`).
- `anon_backend_tests.rs`: a net.anon stream is closed exactly once whether shutdown, the
  last close or freeing comes first (`every_order_of_shutdown_close_and_free_closes_once`),
  a mixnet stream is let go as before, and the stream past net.anon's share for one caller
  is ENOBUFS.
- `anon_rx_tests.rs`: the receive buffer never holds more than one reply and poll never
  calls an empty open stream readable (`the_buffer_never_holds_more_than_one_reply`).
- `anon_reply_tests.rs`: short, cut, overlong, garbled and unknown replies from net.anon end
  in a clean errno (`hostile_replies_end_in_an_errno_and_never_a_panic`).

## Running

The handbook page is [`docs/handbook/linux/personality.md`](../../docs/handbook/linux/personality.md).

```sh
cd userland/capsule_linux_proofs
cargo test --release
```

The crate holds 295 `#[test]` functions. CI runs it through `nix flake check`, which
`.github/workflows/verify.yml` runs: every `*_proofs` crate with a `Cargo.lock` is a check
(`proofs-capsule_linux_proofs`, `tools/nix/checks.nix`) that runs `cargo test --frozen
--release` with overflow checks on and `RUST_TEST_THREADS=1`, then clippy over all targets
with `-D warnings`. `fuzz/` holds one cargo-fuzz target, `linux_inputs`, over the Wayland
wire reader, the ELF reader, the tar, ar and base64 readers and the HTTP reply reader;
`.github/workflows/fuzz.yml` runs it nightly.

## Not covered

The serve loop and dispatch (`serve/`), foreign process creation, guest memory access other
than through the `Memory` trait, the file layer beyond path resolution and stat, sockets
beyond the route decision and a stream's backend bookkeeping (the IPC calls to net.sockets
and net.anon, and the policy store read, run only on a machine), the Wayland server beyond its wire codec and object table, and network fetches are not exercised:
of the calls, the rules that decide each refusal and errno are, the I/O around them is not. The mutation pass is seeded, not coverage-guided.
