# Design: a Secret Service in each Linux box

Status: proposed, before code. Next release, with the
kept homes of `docs/design/kept-homes-and-installs.md`, after K1 and K1b in
0.9.3. Bare paths are under `userland/capsule_linux/src/linux/`.

## The promise

A Linux app that keeps secrets through the Secret Service
(`org.freedesktop.secrets` on the session bus) finds one in its box:
Authenticator adds an account and finds it again. The box sees its own
collection and nothing else, not another app's secrets, names or
existence. The key never enters the box. On an amnesic boot the secrets
last until power off; on a kept one they are on the disk, sealed to this
machine and that app, and survive a NØNOS update as the volume does.

## What is there

- **No bus.** `env.rs:30-48` sets `XDG_RUNTIME_DIR=/run/user/0`, no
  `DBUS_SESSION_BUS_ADDRESS`, and turns GTK's accessibility bridge off "as
  there is no session bus". Nothing in capsule_linux names D-Bus; zbus's
  ENOENT is a connect to a path no file backs (`net/named/name.rs:62-63`).
- **Unix sockets.** A path, which becomes a file, or an abstract name
  (`net/named/addr.rs:29-49`, `net/named/bind.rs:40-65`), never seen
  outside the family (`net/named/name.rs:17-21`). One name is the
  personality's own: a stream connect to a path ending `wayland-0` is the
  display it serves (`net/named/connect.rs:42-48`, `unix/path.rs:45-49`),
  bounded each way (`unix/conn.rs:30-34`), not inherited across fork
  (`guest/fork_state.rs:24`).
- **What family sockets lack.** Control data is refused, so no
  SCM_CREDENTIALS or SCM_RIGHTS (`net/msg/send.rs:36-41`); SO_PEERCRED is
  ENOPROTOOPT (`net/opt/value.rs:66`, `net/opt/get.rs:52-57`). D-Bus
  EXTERNAL auth on a Unix socket rests on the peer's credentials.
- **Processes.** A family is the guest and whatever it forks
  (`serve/family.rs:17-21`): fork, vfork and process clone are served
  (`serve/dispatch.rs:43-45`, `serve/route_life.rs:53-54`), clone3 is
  ENOSYS so glibc falls back (`call/glibc_sched.rs:68-72`), execve needs
  EXEC and runs inside both programs' boxes (`call/spawn/exec.rs:29-31,57`,
  `app/identity_exec.rs:27-37`); a fork storm stops at 511
  (`docs/release-evidence/0.9.3/isolation-61ec29ea2/README.md:40`). A
  dbus-daemon and a keyring daemon could run, but not authenticate a client.
- **Private files.** `/tmp`, `/dev/shm`, `/run`, `/var/tmp` are the run's,
  under `/linux-private/<16 random hex>`, removed at the end
  (`file/private/names.rs:30-55`, `file/private/rule.rs:23`, `start.rs:54-57,72`).
  `/home` and `/root` are `/linux-home/<32 hex>/home` and `/root`, vfs RAM
  for the boot (`file/private/home.rs:17-21,38-65`); no other guest path
  reaches that root (`file/root.rs:36-44`), and vfs serves both trees to
  ForeignExec alone (H1, `userland/capsule_vfs/src/server/fs_gate/guard.rs:19-30`).
- **A box's stable name.** The home key: BLAKE3 of `NONOS.linux.home.v1`
  and the listing key (publisher key, listing name), kept by a new
  release; an unlisted program's lasts the boot (`app/home_key.rs:17-37`).
  The app_id changes each release (`app/identity.rs:62`). Services are
  policy bits, an unnamed bit refused (`userland/nonos_linux_policy/src/mask.rs:17-42`),
  a refusal said in the box report (`app/gate.rs:41-50`).
- **Keys userland may hold.** The keyring: RAM records, 16 per owner, by
  pid, dropped when it ends (`docs/security/device-secrets-and-keys.md:56-64`;
  Keyring only reaches it, `src/services/registry/policy.rs:40-55`).
  Machine keys, `nonos.vault.root.v1` among them, go to any Crypto holder
  (`userland/nonos_vault/src/subkey.rs:32-36`, `device-secrets-and-keys.md:38`)
  or, with a bound label, to its caller (`src/syscall/dispatch/crypto/machine_key.rs:43-50`),
  and are PCR 9 bound, so die at an update (`device-secrets-and-keys.md:31,87`).
- **K1, K1b** (`s1/k1-update`). `approved_key` and `handover_key` are
  kernel-only, PolicyAuthorize over PCR 9 then PCR 0, 4, 7, or 0, 7
  (`src/security/tpm/device_secret/approved_key.rs:38-58`, `consts.rs:38-46`).
  The volume key is random, wrapped under `approved_key("blockfs.data.v2")`
  (`src/fs/blockfs_volume/key_pick.rs:33-61`); the handover moves it
  alone. The open key is in `VOLUME` (`src/fs/blockfs_volume/state.rs:20-31`),
  drawn for the boot when nothing is kept (`src/fs/blockfs_volume/session.rs:40-58`).
  No userland key follows the approved chain. Kept names (`/h-`, `/li.`,
  `/lp-`) reach ForeignExec only (`src/syscall/microkernel/data/kept_name.rs:25-38`);
  `MkKeptPut` is on `s1/kept-index`, not here.

## The design: (a), the personality serves the bus

- **(b) Stock dbus-daemon and gnome-keyring or oo7-daemon.** Least TCB,
  and the keyring file is kept with the home for free. But the master key
  sits in guest memory and the collection in the guest's tree, open to
  anything the box runs or execs; family sockets must carry credentials;
  and both daemons want a Prompt window to make or unlock a collection.
- **(c) A NØNOS capsule behind a bus proxy.** capsule_linux must still say
  which box asks, so two parties hold one boundary; in the keyring, an
  app's protocol would sit beside the wallet's keys.
- **(a) capsule_linux answers a minimal bus and the Secret Service**, as
  it answers the display. The box reaches items one at a time, never a key,
  a file or another box's collection. A bounded parser joins a capsule that
  already parses one guest protocol (`wayland/`, about 8,700 lines) and is
  already the guard between apps (kept-homes, "What it does not do").
  Chosen: what the box can take out is what it put in.

**The bus.** A box granted it (question 2) with a listed home key gets
`DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/0/bus`; `/run` is its own. No
file is made: a stream connect to exactly that path becomes a bus
connection, as the display's does, and a bind on it is EADDRINUSE. Exact,
not a suffix, so a guest socket named `bus` stays the guest's. Up to 8
connections a family (GDBus and zbus may be two in one app), each bounded
as the display's, none inherited across fork. Auth: EXTERNAL or ANONYMOUS,
then BEGIN; NEGOTIATE_UNIX_FD is ERROR; the peer is this family by
construction. `org.freedesktop.DBus`: Hello, RequestName, ReleaseName,
GetNameOwner, NameHasOwner, ListNames, AddMatch, RemoveMatch, GetId,
Peer.Ping. No message passes between connections; none exceeds 64 KiB.

**The Secret Service.** `login`, at `/org/freedesktop/secrets/collection/login`,
alias `default`, kept as the box keeps; `session`, the personality's memory
for the run, never written. ReadAlias of any other name answers `/`.

| Interface | Served |
|---|---|
| Service | OpenSession (`plain`: the secret crosses no wire), SearchItems, GetSecrets, ReadAlias, Unlock and Lock (no change, prompt `/`), CreateCollection for `default` only (answers `login`); Collections |
| Collection | CreateItem (with replace), SearchItems; Items, Label, Locked (false), Created, Modified; Item signals to the box's connections |
| Item | GetSecret, SetSecret, Delete; Attributes, Label, Locked, Created, Modified |
| Session | Close |
| Bounds | 1024 items, 64 KiB a secret, 1 MiB sealed |

**At rest.** One file, `NLXSEC1` (magic, version, generation, items),
sealed by `nonos_vault::seal` (`userland/nonos_vault/src/seal.rs:34-56`)
under the box key, record name `linux.secrets.v1:` and 32 hex of the home
key, so box A's blob does not open as B's. It lies at
`/linux-home/<32 hex>/secrets`, beside `home` and `root`, where no guest
path reaches. On a kept boot it also goes to `/s-<32 hex>.a` and `.b`, a
fourth kept prefix, by `MkKeptPut`: the newer valid slot wins, a torn one
never does, as homes'.

**The key.** A new kernel call, `MkKeptKey(home key)`, ForeignExec only:
HKDF-SHA256 over the open volume key, salt `NONOS.linux.secrets.v1`, info
the home key; never the volume key; ENODEV with no volume, and the bus then
refuses secrets. Only a boot that unwrapped way 3 under the approved chain,
or K1b's record once, holds the volume key, so the box key is bound to K1
and lives as long as the volume, with no handover of its own. A label of
its own under `approved_key` would die at the first new bootloader (K1b
moves the volume key only), cost a TPM session a call
(`src/security/tpm/device_secret/authorized.rs:36-55`), and repeat on every
amnesic boot.

**Amnesic, kept, update.** Amnesic: a new volume key each boot, so a new
box key; the vfs file lasts across runs until power off, `MkKeptPut` is
ENOTSUP, and the personality says once "this boot keeps nothing: <app> and
its secrets last until power off". Kept (installed, keeping on, kept homes
landed): loaded from the newer slot at the first run of the boot, written
on change, at most once a second, and when the family ends. A NØNOS update
keeps it by K1 (signed kernel) and K1b (new bootloader); firmware, Secure
Boot or a TPM clear lose it, as the volume. A way-1 volume
(`key_pick.rs:46-50`) loses it at the next update, and the store says so.
An app update keeps the listing, so the home key and the collection.
Uninstall removes the slots and the file with the home (I.5).

**Never seen across boxes.** Not the bus (family sockets, a personality a
run), object paths (one collection loaded), file (H1, no guest path), slots
(`kept_name.rs`) or key (one per home key, bound into the record). Not
done: capsule_linux sees every box it serves and the kernel cannot tell
which box a key is for; as for homes, the guard is capsule_linux. No
rollback guard beyond a boot.

## Refusals

- Any destination but `org.freedesktop.DBus` and `org.freedesktop.secrets`
  is ServiceUnknown: `org.freedesktop.portal.*` (Secret, Settings),
  Notifications, `ca.desrt.dconf`, `org.gtk.vfs.*`, `org.a11y.Bus`.
- RequestName of either bus name: no app becomes the Secret Service, not
  KeePassXC's provider. No Prompt is ever made: SetAlias, CreateCollection
  for another alias and Delete of `login` are NotSupported.
- Another box's collection by path or alias: NoSuchObject or `/`, never
  saying whether it exists. BecomeMonitor, eavesdropping matches,
  StartServiceByName, fd passing, a message over 64 KiB: refused.
- No grant, an unlisted program, a built-in: no bus, said by name in the
  box report, as the display is.

## What 0.9.3 does meanwhile

No in-RAM bus: it is not a cheap half (the wire, auth and objects are most
of the work; keeping is two slots). The store's text for
gnome-authenticator and Seahorse says "needs the Secret Service, which
NØNOS does not have yet: it will not start", and no
`DBUS_SESSION_BUS_ADDRESS` is set. Rejected: gnome-keyring in the closure
(no credentials, wants a Prompt); posing as a Flatpak so oo7 takes its file
backend (it asks the Secret portal for that key).

## Proofs and evidence

| # | Rule | Mutant that must fail a test |
|---|---|---|
| S.1 | the decoder refuses a bad signature, alignment, a length past the message, nesting past 32, trailing bytes (fuzz target) | skip the array length check |
| S.2 | only the two bus names are answered | match by `org.freedesktop.` prefix |
| S.3 | nothing passes between connections; RequestName of a bus name refused | deliver to an owned name |
| S.4 | no answer carries a prompt but `/` | a prompt for Lock |
| S.5 | NLXSEC1 round-trips; box A's blob fails as box B's | home key left out of the record |
| S.6 | NLXSEC1 refuses bad magic, version, a duplicate item, a size past bounds | skip the bound |
| S.7 | the newer valid slot wins, a torn one never | prefer slot a |
| S.8 | MkKeptKey: EPERM without ForeignExec, ENODEV with no volume, two home keys two keys, never the volume key | HKDF without info |
| S.9 | the same box key after a K1 re-open and a K1b move | derive from `approved_key` |
| S.10 | a connect without the grant is refused by name | check WAYLAND |

KVM lane, logs under `docs/release-evidence/<next>/`. (1) Amnesic: add an
Authenticator account, close, reopen: there; after Power Off
`tools/nonos-zero-state-check` finds no seed. (2) Installed NVMe, software
TPM, keeping on: there after a reboot, a K1 update and a K1b update; no
seed in the disk image. (3) Seahorse lists only its own empty `login`.
(4) A hostile guest (`isolation-61ec29ea2/hostile.steps`): other boxes'
paths, invented aliases, BecomeMonitor, deep nesting, big messages, a bind
on the bus path, all refused. (5) Uninstall, reinstall: empty.

## Open questions for the owner

1. If oo7 will not fall back to `plain` from `dh-ietf1024-sha256-aes128-cbc-pkcs7`, is that DH and AES in capsule_linux accepted?
2. A new SECRETS policy bit (listings republished), or every listed FILES_HOME box?
3. Seahorse sees only its own empty collection here. List it at all?
4. Two runs of one app at once: the second read-only, or refused? H2 asks the same.
5. A rotated publisher key or renamed listing is a new, empty box. A migration signed by the old key?
6. "Erase secrets" apart from "Erase home" (kept-homes unit 6)?
7. 0.9.3: also a box-report line on a connect to `/run/user/0/bus` ("the session bus: not served yet", still ENOENT)?
8. No volume (a live boot short of 256 MiB): refuse secrets, as proposed, or keep them for the run?
