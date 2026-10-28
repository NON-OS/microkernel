# Design: SERVE_ONE, one guest port for one capsule

Status: approved by the owner on 28 October, in code (S1). 0.9.3 if it is done
properly, else 0.9.4. For S2 letter 11 and S3's answer
(`HANDOFF/letters/2026-10-27-s2-to-s3-s1-simplex-split.md`,
`2026-10-27-s3-to-s2-simplex-core-answers.md:30-40`). Cited at main
8f2d183dda. Bare paths are under `userland/capsule_linux/src/linux/`;
`policy/` is `userland/nonos_linux_policy/src/`, `ipc/`
`src/syscall/microkernel/ipc/`, and `registry/` `src/services/registry/`.

## The promise

The SimpleX core, a Linux guest in its own box, serves its JSON-over-WebSocket
API on 127.0.0.1. One native capsule reaches it: the SimpleX window, known by
the endpoint the kernel registered for it at spawn. No other capsule, Linux
run or network peer does; the kernel refuses them, not only the personality.
The guest gains one listener and accept; its streams out stay on Anyone or Nym.

## What is there

- **The box.** 256 canonical bytes, zeros from byte 217
  (`policy/layout.rs:19-34`). Decode refuses a non-zero pad or an unnamed bit
  (`policy/decode.rs:33-41`, `policy/mask.rs:21-55`). No field holds a name:
  destinations ride as a hash (`policy/dests/mod.rs:17-25`), and "Market
  releases carry no list yet" (`plan/33-net-dests.md:40-42`). Listed args
  follow the policy, long options only (`policy/args/word.rs:51-63`); the
  market reads only header and ids (`userland/marketplace_abi/src/verdict/policy.rs:51-59`).
  The install role vouches for it (`install/market/check.rs:29-42`,
  `install/market/bind.rs:31-43`); each start checks it (`app/bind.rs:59-72`).
- **The refusal S2 meets.** AF_INET needs NET_ANON or NET_LISTEN_LO
  (`net/socket.rs:37-43`), and AF_INET6 is EAFNOSUPPORT (`net/socket.rs:46`).
  Without NET_LISTEN_LO, bind and listen are EACCES (`net/bind.rs:47-49`,
  `net/listen.rs:35-37`), and outside 127.0.0.0/8 always (`net/bind.rs:50-52`).
  With it the listener is the family's: one table per personality process
  (`net/sock/cell.rs:17-19`, `net/connect/lo.rs:24-68`), and each run role
  is its own process (`src/userspace/capsule_linux/roles.rs:21-22,51-59`).
- **The server path to reuse.** `join` queues a server end that nobody holds
  until accept (`net/sock/link.rs:36-67`, `net/accept.rs:36-53`). Readiness
  is the listener's queue and the peer's room (`net/sock/ready.rs:33-36,46-48`).
  A write takes what fits in the peer's 131,072 bytes, then EAGAIN
  (`net/sock/put.rs:49-55`, `net/sock/opts.rs:47-48`). Parked calls retry
  each turn (`serve/loop_impl.rs:45`), which waits up to 250 ms for a trap
  (`serve/loop_impl.rs:26-36`); IPC does not cut it short (`src/process/foreign/wait.rs:41-56`).
- **The display, the precedent.** A stream connect to a path ending in
  `wayland-0`, with WAYLAND, is answered by the personality itself
  (`unix/path.rs:45-49`, `net/named/connect.rs:42-48`): one a guest, not
  inherited, 1 MiB each way then EAGAIN (`guest/handle.rs:62-63`,
  `guest/fork_state.rs:24`, `unix/conn.rs:20-34`). Input comes to its inbox,
  believed from the router's pid only (`wayland/input/pump.rs:53,59-61`).
- **Endpoints and calls.** An endpoint is a name, port, owner pid and caps
  (`registry/endpoint.rs:21-27`), dropped at exit
  (`src/process/exit/finalize.rs:59`). A spawn registers the capsule's name
  (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:104-106`);
  at run time only five net names can be claimed (`ipc/register_allowed.rs:24-43`).
  A baked name, and names under it, is its namespace's; an empty namespace
  means no one's (`src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/baked_names.rs:17-23`).
  A call checks caps, peer list and the owner's inbox (`ipc/send_caps.rs:36-62`,
  `ipc/send_address.rs:28-40,53-55`), 8 waiting a caller
  (`ipc/pending_reply/share.rs:31-32`). `mk_ipc_recv_req` gives the sender's
  pid and a request id to `mk_ipc_reply_to` (`userland/libc/src/ipc/recv_req.rs:21-59`).
- **Identity and allow-lists.** Identity is owning a kernel-registered
  endpoint (`registry/peers_check.rs:43-49`). MkAppLaunch admits AppInstall,
  or the shell by `caller_owns("desktop_shell")` (`src/syscall/contract/cap_table/mk_trust.rs:52-58`).
  It queues the package, not who asked (`src/syscall/microkernel/app_launch.rs:23-44`),
  one run at a time (`src/userspace/capsule_linux/run_owner.rs:25-36`).
  MkLinuxRunNet admits only the run role, for its own package
  (`src/syscall/microkernel/linux_run_net.rs:17-28`). Allow-lists are image
  tables: `PEERS` by caller (`registry/peers.rs:26-41`), `HELD` by endpoint,
  admitting a caller that owns a listed endpoint (`registry/held.rs:47-61`).
  A send by pid passes every gate its target serves (`registry/held.rs:63-79`,
  `registry/addressed.rs:17-39`). Run-time holds: not present.

## The design

**The policy form.** `SERVE_ONE = 1 << 9`. The pad at byte 217 takes
`port u16 LE | name [u8; 32]`: 1 to 32 bytes of `[a-z0-9._-]`, starting with
a letter, zero padded; the pad then starts at 251. The bit is set exactly
when the field is non-zero, else `PolicyError::Serve`. No committed hash
moves; older decoders refuse the new form. `market_ready` refuses SERVE_ONE
with WAYLAND: the hold covers the whole inbox, and input comes by
MkIpcSendToPid (`userland/capsule_input_router/src/route/deliver.rs:33`). An
exec keeps the bit only into the same field. S3's listing: `serve:
app.simplex:5225`, NET_ANON | FILES_HOME, args `--chat-server-port=5225
--yes-migrate` (S3 to confirm the long spelling).

**The pairing and the endpoint (kernel).** `SERVES: &[(capsule, package)]`
in `registry/serves.rs`, measured with the image as PEERS is, pairs
`app.simplex` with `nonos-simplex-chat` and `deb:nonos-simplex-chat`
(`src/userspace/capsule_linux/family.rs:32-39`); a host test holds each
capsule to a baked name. A new call, `MkServeOne(name_ptr, name_len)`,
returns a port, with a gate like MkLinuxRunNet's. The caller holds
ForeignExec, which only capsule_linux's spawn grants
(`src/userspace/capsule_linux/spawn.rs:47`); it is the process registered as
app.linux.run or app.linux.run.net, argv `run <package>`
(`src/userspace/capsule_linux/run_owner.rs:38-51`, `run_net.rs:41-45` beside it);
and `(name, package)` is in SERVES. So another listing claiming SERVE_ONE
app.simplex is refused: the window never reaches an impostor core. The
kernel registers `linux.serve.<name>` for the caller: IPC to call, a port
from a reserved range (as `registry/reserved.rs:24-28`), held to `name`; one
per process, gone with it. `("linux.serve", "")` joins CORE
(`baked_names.rs:31-63`), so no loaded capsule declares the name.

**The caller rule.** `ServiceEndpoint` gains `held_to: Option<String>`, and
`endpoint_admits` (`registry/held.rs:59-61`) takes it: a static list first,
else a held name admits only its owner (`registry/peers_check.rs:43-49`),
else today's answer. `caller_may_reach` and `caller_may_write_inbox`
(`registry/peers_check.rs:54-69`) pass it, so every way in is checked: name,
port, the role's own name (`ipc/send_address.rs:28-40,53-55`), pid
(`ipc/send_to_pid.rs:47-50`). A refusal is EPERM, logged `[IPC-DENY]`;
replies to the personality's own calls are unaffected (`ipc/send.rs:53-58`).

**The guest side.** The listen taken is a stream bind to exactly 127.0.0.1
on the field's port, without NET_LISTEN_LO; `inet_allowed` counts SERVE_ONE
(`app/rules.rs:33-35`). The first listen calls MkServeOne before the socket
is marked listening. accept, read, write, close, poll and select stay the
family's own: the window's OPEN joins a door end, which the personality
holds, to the listener, and `autobind` gives it 127.0.0.1 and an ephemeral
port (`net/sock/bind.rs:31-40`), so the guest accepts a loopback client.
Upstream serves one client (S3's API letter §2): one connection at a time.

**The wire.** The window calls `linux.serve.app.simplex` with
`mk_ipc_call_timeout`. The personality takes frames from its inbox with
`mk_ipc_recv_req`, as `wayland/input/pump.rs:53` reads it, and answers with
`mk_ipc_reply_to`. Every frame, asked or answered, is the header `"NSO1" | op u8 |
status u8 | conn u16 LE | len u32 LE`, then `len` bytes; ops OPEN 1, WRITE 2,
READ 3, CLOSE 4; a question's status is 0, an answer's OK 0, CLOSED 1 or a Linux
errno. The numbers are `userland/nonos_serve_wire`, which both sides import.

| Op | Body | Answer |
|---|---|---|
| OPEN | none | OK and the conn id, once the door end is queued |
| WRITE | bytes for the core, 64 KiB at most | OK and the count the guest's buffer took; 0 when full |
| READ | want u32, wait_ms u32 (5000 at most) | DATA once the core has written, empty after wait_ms; CLOSED after its close |
| CLOSE | none | OK; the guest reads end of file |

A frame counts only from the held name's owner (as `pump.rs:59` checks the
router) and the pid that opened the conn. Backpressure is the family's: a
WRITE that took 0 is retried; the core's write parks on a full door end.
Bounds: one conn, one held READ, 64 KiB a frame, 128 KiB each way, 8 calls.
The window speaks WebSocket over the bytes; the personality does not parse
it. While a conn is open the loop waits 20 ms, not 250, until unit 7.

**Who calls what.** (1) The window, owning `app.simplex`, calls
MkAppLaunch(`linux.kali.nonos-simplex-chat`); a new arm admits a SERVES capsule
for its own listing (`mk_trust.rs:61`, `src/syscall/microkernel/app_launch_rule.rs`).
(2) Init starts the run, which restarts as app.linux.run.net
(`src/userspace/capsule_linux/run_net.rs:17-22`), held to the launcher's pid
(`src/userspace/capsule_linux/served_run.rs`). (3) The core listens; the
personality calls `MkServeOne("app.simplex")`. (4) The window looks up
`linux.serve.app.simplex` (`ipc/lookup.rs:54-56`), retrying 30 s on ENOENT,
then sends OPEN, the upgrade and `/_get chats`. (5) The window exits; the run
ends, as a terminal's runs do (`src/userspace/capsule_linux/terminal/exit.rs:21-29`).

## Refusals

| Case | Answer |
|---|---|
| Bit and field apart, port 0, a bad name; a build without SERVE_ONE | install: "its policy is malformed" (`install/market/check.rs:30`) |
| SERVE_ONE with WAYLAND | "its policy is not market ready" (`install/market/check.rs:31-33`) |
| Bind to another port, 127.0.0.2 or 0.0.0.0, or a datagram; a second listener | EACCES, as today; `::1` EAFNOSUPPORT; EADDRINUSE |
| MkServeOne without ForeignExec, outside a run role, or a pair not in SERVES | EPERM; the listen is EACCES, the box report naming SERVE_ONE |
| A bad name; a name already served, or a second call; a full registry | EINVAL; EBUSY (listen EADDRINUSE); ENOMEM (listen ENOBUFS) |
| A caller not owning `app.simplex`, by name, port, role name or pid | EPERM, `[IPC-DENY]` |
| A ninth call in flight; no core yet, or gone | EBUSY (`ipc/call/sys_ipc_call.rs:50-53`); lookup ENOENT; a stale port EPERM (`ipc/send_caps.rs:33-42`) |
| Bad magic, `len` wrong or over 64 KiB, unknown op | ERR EINVAL, nothing delivered |
| OPEN with a conn open; with no listener; with a full queue | ERR EBUSY; ECONNREFUSED; EAGAIN |
| A conn not open, or another pid's frame; a second held READ; WRITE after the core closed | ERR EBADF; ERR EBUSY; CLOSED |
| MkAppLaunch by a capsule not in SERVES, or for another listing | EPERM |

## What it does not do

- The personality sees every byte, as it does the display's, and the kernel carries them.
- The window holds the whole API: it can read, export and delete every chat.
  SERVE_ONE says who connects, not what they ask, and the API has no
  authentication of its own (S3's API letter §1).
- No port forward: one capsule, one port, one stream, store runs only; the
  core's own family still reaches it. The pairing is by package name, not
  publisher key. While the core runs no other store Linux app starts.

## Proofs

| # | Rule | Mutant that must fail a test |
|---|---|---|
| P.1 | decode refuses bit and field apart, port 0, a 33-byte or upper-case name, a NUL inside, bytes past the pad | skip the pad check |
| P.2 | round trip; every committed policy hash unchanged | write the field while the bit is clear |
| P.3 | `market_ready` refuses SERVE_ONE with WAYLAND | drop the check |
| P.4 | only a stream bind to exactly 127.0.0.1 and the port is taken | match 127.0.0.0/8 |
| P.5 | SERVE_ONE alone admits AF_INET; another bind stays EACCES | let SERVE_ONE imply NET_LISTEN_LO |
| P.6 | OPEN queues one end; the guest accepts it; the listener polls readable | queue the door end twice |
| P.7 | WRITE takes only the room; a guest write to a full door end is EAGAIN | an unbounded extend |
| P.8 | CLOSE gives end of file; the guest's close answers READ with CLOSED | free without telling the peer |
| P.9 | the codec refuses bad magic, `len`, size or op; a fuzz target | trust `len` |
| P.10 | one conn; another pid's frame EBADF; one held READ | key conns by id alone |

kernel_proofs, beside `userland/kernel_proofs/src/ipc_held_tests/gates.rs`,
mutant in brackets: K.1 `endpoint_admits` admits only `held_to`'s owner (any
endpoint's owner). K.2 `inbox_admits` refuses a send by pid to the holder
(skip `held_to`). K.3 MkServeOne's rule; `admit_loaded` refuses `linux.serve.*`
(drop the CORE line). K.4 one a process (two). K.5 the launch arm, its listing only (any).

## KVM lane

Logs under `docs/release-evidence/0.9.4/`, after S1's guest-connect fix
(`HANDOFF/letters/2026-10-27-s3-to-s1-guest-connect-cause.md`).
1. The window starts the core, upgrades, and sends `/user` and
   `/_get chats 1 pcc=on`. Its transcript matches S2's by `corrId` and `type`.
2. `test.attack` (`baked_names.rs:56`) calls by name and port, sends by pid,
   and addresses `app.linux.run.net`. Each is EPERM, with `[IPC-DENY]`.
3. `nc 127.0.0.1 5225` from a Terminal `linux` guest: EACCES without an internet
   grant, else ECONNREFUSED (`net/connect/lo.rs:65`).
4. Closing the window ends the core; the lookup is then ENOENT.

## Units of work, in landing order

1. S1, policy crate, about 250 lines: bit, field, codec, market rule, P.1-P.3.
2. S1, kernel, about 400 lines: `held_to`, SERVES, MkServeOne, K.1-K.4; owner review.
3. S1, kernel, about 200 lines: the launch arm, the launcher's pid, K.5.
4. S1, personality, about 700 lines: door, wire, gates, P.4-P.10, fuzz target.
5. S3, about 120 lines: listing, args, pack tool (`tools/nonos_linux_pack/policy.py:23-24`).
6. S2: `app.simplex` in APPS with its manifest, the client, the transcript test.
7. S1, kernel, about 80 lines: MkForeignWait returns on an inbox arrival.
8. S1 with S2 and S3: the KVM lane.

## Open questions for the owner

1. `app.simplex` in APPS, or `simplex.ui` added? Today a publisher under namespace
   `simplex` could declare `simplex.ui` (`name_rule.rs:69-72`, beside `baked_names.rs`).
2. The field in version 1, or a version 2 the market refuses until updated
   (`userland/marketplace_abi/src/verdict/policy.rs:57`)?
3. SERVES needs a kernel release per pairing: a signed listing field later?
4. Start by the launch arm or the shell? Stop with the window? One connection?
5. One run at a time, or a role of its own for served runs?
6. Does the core bind AF_INET 127.0.0.1 (the lane checks)? 20 ms until unit 7?
7. Pair by package name, or by listing key, the personality passing its `listing_id`?

## The owner's decisions (28 October), and the interfaces they fix

1. The window is `app.simplex`, in the image's APPS: no publisher can claim it.
2. The field rides in policy **version 2**. A version 1 policy is byte for
   byte as today; a decoder that knows only version 1 refuses version 2, and
   so does the market's verdict until it is updated.
3. SERVES is a kernel table now; a signed listing field may replace it later.
4. The shell starts the window; the window starts the core by MkAppLaunch;
   the core stops with the window; one connection.
5. One served run at a time.
6. The loop polls at 20 ms while a conn is open, until unit 7.
7. Pairing by **listing key**, not package name: SERVES is
   `&[(capsule, listing_key: [u8; 32])]`, and the personality passes the
   listing key it bound and checked at start (`app/bind.rs`). The kernel admits
   MkServeOne only when `(name, listing_key)` is in SERVES and the caller is
   the run role.

Interfaces the units share, so they can be written apart:
- policy crate: `Policy::serve_one(&self) -> Option<ServeOne>` with
  `ServeOne { port: u16, name: [u8; 32], name_len: u8 }` and
  `ServeOne::name(&self) -> &str`; `mask::SERVE_ONE = 1 << 9`; version 2 is
  `VERSION_SERVE = 2`.
- kernel call `MkServeOne(name_ptr, name_len, key_ptr, key_len=32)`, returning
  the endpoint's port (> 0) or a negative errno; libc
  `mk_serve_one(name: &[u8], listing_key: &[u8; 32]) -> i64`. The endpoint is
  named `linux.serve.<name>` and held to `name`.
- wire: as in "The wire" above, magic `NSO1`.

As built, units 3 and 7:
- The launch arm. The kernel cannot read an installed package's key when it
  queues a launch: the key is in the policy the install role wrote to the
  personality's store, which only the run reads and checks at start. So
  `LISTINGS` beside SERVES names the listing id each capsule may launch, the
  id its key is made from (market_proofs holds the two together); the launch
  is held to that id, EPERM otherwise, and MkServeOne still binds the key, so
  a same-named listing of another publisher starts but is never served.
- The run is noted as the launcher's process when queued, follows it into
  app.linux.run.net, and is ended with the launcher's teardown; a start still
  queued then ends as it starts. Any other launch forgets the note.
- MkForeignWait returns 0, nothing written, once a message comes to the
  caller's own inbox, told once a count so a message left unread does not
  wake it again. A frame's size and ETIMEDOUT are as before.
