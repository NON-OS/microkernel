# nsh.front / nsh.attest: the in-guest nsh/1 host

This note describes the two NØNOS capsules that let `nsh`, the attested,
post-quantum remote shell, run **inside** a booted NØNOS guest and accept a
connection from the finished `nsh` client, how they are built and wired, how to
produce the development fixtures they need, and exactly what remains for a full
in-guest boot test. It is the companion to the nsh specification in `docs/nsh`
("What waits") and `docs/nsh/nsh-v1*.md`.

Both are release capsules, shipped by the `server` profile alone
(`nonos-server`); a sealed release admits them only through the release's
signing ceremony (`nonos-data/trust/`, `CEREMONY.md`). The release build
and the operator's steps are in [nsh-server-runbook.md](nsh-server-runbook.md).
This note covers the development side: the devkit and the development twin,
which use throwaway keys only. Nothing here signs or enrols anything with the
release keys.

## Architecture

nsh/1 attests the **host** (the NØNOS machine). The kernel forbids a TPM quote
(`MkAttestDoc`) to any process that holds `Network`
(`src/syscall/caps/checks/system.rs`: `grants(AttestRead) && !reaches_network()`).
So the host is split across two capsules:

- **`nsh.front`**, holds `Network` (and IPC, Memory, CoreExec), not
  `AttestRead`. It listens on a `net.sockets` stream socket, drives the sans-IO
  `nsh_server` state machine per connection, relays each handshake's 32-byte
  attestation challenge to `nsh.attest` over IPC, and runs a minimal in-guest
  shell behind the sealed session.
  Crate: `userland/capsule_nsh_front` (bin `nsh_front`).
- **`nsh.attest`**, holds `AttestRead`, Crypto and FileSystem (and IPC,
  Memory, CoreExec), not `Network`. It answers one measured caller,
  `nsh.front`, turning a challenge into the kernel's real NONOSATT v5
  document (`sys_attest_doc`) and the registry (`sys_attest_entries`). It
  also holds the host's hybrid keys, which the machine makes at its first
  boot and seals to its TPM, signs each digest nsh.front's server asks it
  to, and reads the machine's own enrolment, role table and revocation list
  for each connection (nsh-server-runbook.md, sections 3 to 5). Because it has no network, the quote never
  leaves the machine through it; because `nsh.front` has no `AttestRead`, it
  can never fetch a quote itself. The quote `nsh.front` presents is bound to
  that session's transcript and host keys, so it is useless to anyone else.
  Crate: `userland/capsule_nsh_attest` (bin `nsh_attest`).

The IPC between them is `userland/nsh_front_wire` (a 12-byte header + body,
shared so the format cannot drift). `nsh.attest` only answers the pid the
kernel bound the `nsh.front` service name to (`src/caller.rs`).

The session shell (`capsule_nsh_front/src/shell.rs`) is the shared NØNOS shell
(`userland/nonos_shell`), the command language the GUI terminal reads: pipes
and the filters after `|`, `if`/`for`/`while`, variables, aliases and history.
`help` (also `commands`) lists every name it runs. Each builtin asks the
machine only through `nonos_shell::host::Host`, which nsh.front answers from
its own grant (`capsule_nsh_front/src/shell_host.rs`): randomness, the clocks,
memory, the process table (`ps`), the measured-boot state (`attest`, `pcr`),
and the kernel's service registry (`service NAME...`, also `svc`, which prints
each name's port and serving pid; a name is 1 to 64 bytes of printable ASCII
and is checked before the kernel is asked). Under Network, and only while
the chosen network is Direct, `nslookup HOST` and `ping HOST`: the route is
read before either is asked (`nonos_route_link::direct_refusal`), and off
Direct the session prints why and that nothing was sent; net.core refuses a
lookup in the clear off Direct as well. A lookup goes to `net.dns` and waits
at most 6 s; `ping` sends 4 ICMP echoes through `net.ip`, each waited for at
most 1 s, so it holds the session about 4 s at most. A host name is 1 to 253
letters, digits, dots and hyphens; nothing else reaches the resolver.
A session runs inside nsh.front, so the shell itself holds a login to its
role's capabilities (`nonos_shell::session::Who::caps`): `service` runs only
if the role holds IPC, and `nslookup` and `ping` only if it holds Network,
which no shipped role does at login (nsh-v1-roles.md). A refused command says
which capability the role lacks and asks the machine nothing.
nsh.front holds no filesystem capability, so no file is read and `diff` says
so. What the terminal's other commands would need, none of which nsh.front
holds: `fs` commands and redirects to files need FileSystem; `git` needs
FileSystem and Network; `market` needs AppInstall; `nox` install needs
AppInstall and SpawnWindow; `jobs`, `bg`, `fg`, `tool` and `linux` need
SpawnWindow and every capability of the tool started; `log`, `capsules` and
`receipt` need AttestRead, which the AttestRead versus Network split keeps
from nsh.front. An escalation is refused, as no second-session launcher
exists in the guest yet.

## Ports

- **TCP listener (the network port clients dial):** `nsh.front` binds
  `0.0.0.0:2222` (nsh/1's default, as the `nshd` dev harness uses). Binding a
  non-loopback address requires the Direct route; a test or VPS guest serving nsh
  has it. **nsh.front is the first in-guest capsule to open a `net.sockets`
  listener** (OP_SOCKET/OP_BIND/OP_LISTEN/OP_ACCEPT); only net.sockets' own
  tests have exercised that server path before, so a boot test is also the
  first exercise of inbound TCP for a guest caller, verify it there.
- **One session at a time.** nsh.front serves one connection to
  completion before taking the next. While a session runs it asks the
  listener every 250 ms for a newcomer and tells each one a plaintext
  `ABORT(BUSY)`, nsh/1's answer from a host at its connection limit, so the
  client says the host is serving all the sessions it takes instead of
  waiting out its handshake timeout. Each newcomer told is held, its bytes
  read and dropped, until it closes or 2 s pass, so the close does not reset
  the connection before the answer is read; at most 4 are held, and one
  more is closed as soon as it is told. A connection the host cannot start
  a session for (its role table or bounds do not open) is told `BUSY` the
  same way (`capsule_nsh_front/src/turn_away.rs`, `newcomers.rs`).
- **Refusals on the serial line.** The client is told one reason code at
  most, and nothing for some refusals (nsh-v1-refusals.md). nsh.front
  writes the server's whole `Why` for each one, a line
  `[NSH-FRONT] refused or ended: Login(Expired)`, on a development image's
  serial console (`capsule_nsh_front/src/refused_said.rs`); a failed relay
  to nsh.attest has its own `[NSH-FRONT] attest relay` line. An image
  without capsule serial output writes neither.
- **IPC service ports (kernel endpoints, a separate namespace from TCP):**
  `nsh.attest` = service 4486 / reply 4487; `nsh.front` = service 4488 / reply
  4489. Both were free in the `44xx` net block.

## The clearnet listener and the mixnet rule

NØNOS sends its network traffic over the mixnet. nsh.front is the one
exception, and only in a development image: it binds a clearnet TCP listener
on `0.0.0.0:2222` through net.sockets' Direct route.

- **Where it is reachable.** The dev image boots under QEMU with user-mode
  networking. The guest's only interface is the emulated NIC behind QEMU's
  NAT, which accepts no inbound connection except through a host port
  forward. `nix run .#qemu -- --forward 2222:2222` adds one bound to the
  host's loopback (`hostfwd=tcp:127.0.0.1:2222-:2222`), so the listener is
  reachable from the test machine only, never from its public address. Do not
  use make's `QEMU_NET_MODE=hostfwd` for this: it binds its forwards on every
  host interface, and it already takes host port 2222 for the guest's port
  22.
- **Never in a release.** No profile in `tools/nix/config.nix` lists
  `nonos-capsule-nsh-front` or `nonos-capsule-nsh-attest`, and a kernel
  built with `nonos-release` (every profile whose loader is not the
  development one) or with `nonos-production` and no `nonos-dev-attest`
  (every `microkernel-<capsule>` production profile) refuses to compile with
  either one (`src/lib.rs`). A development twin keeps `nonos-production` from
  its desktop features and adds `nonos-dev-attest`, so it can carry them.
  `make check-nsh-release-refused` proves the refusal for each feature in
  both postures, and
  checks that it is this refusal that stops the build, not some other error.
- **The Direct route.** net.sockets opens a listener only when the chosen
  network is Direct, as it allows a name lookup. Under Nym or Anyone,
  OP_LISTEN is refused with no transport and nsh.front keeps reopening
  without ever listening. A dev image that serves nsh must be set to Direct.
- **No host-side record.** nsh.front holds no filesystem capability and keeps
  no session record in this version: the record the server hands it is
  dropped. The client receives the whole signed record at the session's end
  and keeps it with `--records` (`nsh-v1-record.md`, "Keeping records on a
  host").
- **Before a real host.** A host that accepts nsh/1 on a public network
  needs a decision on the mixnet rule: nsh over the mixnet, or a documented
  exception. That decision is not made here.

## Build

Every crate builds from committed files alone, for the guest target
(`../x86_64-nonos-user.json`, `-Zbuild-std=core,alloc`): there is no
identity directory and no `include_bytes!` in nsh.front
(`nsh_front_proofs`, `no_identity`). The kernel carries the capsules
through `nonos-server` or in a development twin; `make
check-nsh-release-refused` proves the gate (nsh-server-runbook.md,
section 1).

## Development records (the devkit)

The devkit makes a development machine's records and the client files that
pair with them, with the existing proof builders. It makes no host key: the
machine makes its own at its first boot and writes its enrolment request
(`host.public`, `enrol.nonce`, `enrol.doc`) under `/nonos/nsh`. Its keys are
its own: every run draws a key space from the OS generator and mixes it into
every builder seed, so the authority, role-table and user keys are new each
time and nobody with this source can make them. Only the proofs keep the
builders' fixed seeds. Because the keys change each run, one run writes
every file:

```
cd userland/nsh_server_proofs
cargo run --bin nsh-devkit -- <records dir> <client dir>
cargo run --bin nsh-devkit -- enrol <records dir> <client dir> \
    host.public enrol.doc LOADER_HEX
```

An optional last argument is the clock in milliseconds since the epoch; the
default is the wall clock. The code is `src/kit/devkit` in
`nsh_server_proofs`; `tests/devkit_fresh.rs` proves the keys are fresh and
that the records and the client files pair.

The machine's records, named as `/nonos/nsh` holds them:

- `roles`, `roles.anchor`: `TableB::default()` issued an hour before the
  clock, trusting a revocation list for `MAX_REVOCATION_AGE_MS` (one day,
  the most the format allows); the anchor is the 32 raw bytes.
- `revocations`: `RevB::default()` issued an hour before the clock.
- `enrol`, with `enrol`: the proof spec's certificate (`Spec::cert`) naming
  the machine read from `enrol.doc` (its AK, PCRs 0, 1, 2 and 7 and its
  other PCR 4 applications) and the host keys in `host.public`, with
  `not_after_ms` 30 days after the clock, signed by `AUTHORITY`. The fleet
  id is `[0xF1; 16]`, the host name `db-1.eu.example`.

The client files, named as `nsh` takes them:

- `user.secret` (`--key`): the `nsh_roles_proofs` `User` key in nsh-enrol
  secret-file format (`NSHA\x01\x00\x00\x00`, ed seed, ml public, ml secret).
- `authority.public` (`--authority`): the `AUTHORITY` hybrid public key,
  Ed25519 (32 B) then ML-DSA-65 (1952 B).
- `grant` (`--grant`): `GrantB::default()`, `operator` for the `User`,
  from an hour before the clock for 7 days.
- `--fleet f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1`, `--host db-1.eu.example`.

Every window opens an hour before the clock so a guest a little behind the
machine that made the files still falls inside. The revocation list is the
short one: put a fresh one on the machine within a day; the image itself
never ages, since it carries no list.

The proofs (`nsh_server_proofs`, `devkit` and `devkit_cli`) open the records
exactly as `nsh.front` does (the enrolment whole, `RoleStore::new`) with host
keys of the machine's own, serve them on loopback at the wall clock, and run
the real `nsh` command line against them with the client files: it logs in,
runs a command and exits 0. The same records served a day later are refused.

## The development twin

`tools/nonos-dev-image --nsh` (or `--profile server`) builds a development
twin with both capsules, in the throwaway copy under `target/dev/tree`,
sealed with CI's scratch keys; with `--nsh` it also runs the devkit, writing
the records to `target/dev/nsh-records` and the client files to
`target/dev/nsh-client`, outside the copy. Nothing is baked into the image.
Boot it with a vTPM (nsh-server-runbook.md, section 6).

## Running the boot test

Inside the booted guest, `nsh.attest` and `nsh.front` come up in the network
spawn plan (attest first). `nsh.front` opens `0.0.0.0:2222`. From a machine that
can reach the guest, with the client files generated above:

```
nsh --connect <guest-ip>:2222 --host db-1.eu.example \
    --fleet f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1 \
    --authority authority.pub --releases releases.txt \
    --key user.nshakey --grant grant.bin
```

The client checks the host's hybrid signature, its enrolment, and its
attestation **before** sending the user credential, then logs in and runs a
command (`echo nsh-$((6*7))` → `nsh-42`), exactly as the loopback proof does.

## What remains, the exact boundaries

1. **A development twin is refused at PCR 14.** Its capsules are admitted
   on their Merkle path alone, and the client refuses them before any
   credential is sent (nsh-vps-runbook.md, step 3a). A sealed server image's
   capsules are vendor-admitted.
2. **The client `--releases` file must match the live guest.** The client
   checks the live document against `--releases`, whose line
   `release VERSION PCR8 PCR9 LOADER FRONT NETWORK,...` carries the booted
   guest's measurements; `nsh-probe` writes it from the live guest.
3. **The seal writes the trust sets.** It writes the trust artifacts in
   `nonos-data/trust/capsules/` (`nsh_front.*`, `nsh_attest.*`) with the
   release keys (nsh-server-runbook.md, section 2).
4. **First in-guest listener.** On the booted development guests recorded
   in `nsh-vps-runbook.md` (What has been run), net.sockets passed
   nsh.front's OP_LISTEN on to the `net.tcp` service, which in every desktop
   profile is net.core's, and net.core then refused LISTEN and ACCEPT as
   unknown operations, so a connection to 2222 was reset. net.core now has
   a passive open that takes listens from net.sockets alone and only on the
   Direct route (`capsule_net_core/src/listeners/`). On a guest built with nsh.attest's answers carrying their header,
   nsh.front listens on 2222 once Direct is chosen, nsh-probe
   reads the live document, and `nsh` runs its checks up to PCR 14, where
   it refuses the development image's path-only capsules before any
   credential is sent (nsh-vps-runbook.md, step 3a).
