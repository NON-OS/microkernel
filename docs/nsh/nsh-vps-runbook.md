# nsh on a NØNOS guest: the runbook

This runbook boots a NØNOS development image that serves nsh/1, and logs in
to it from another shell on the same host with the real `nsh` client: an
attested, post-quantum login into a measured boot, a command, a clean exit,
and a session record checked offline.

It is for a test machine or a VPS you control, under KVM, with a software TPM.
Everything in it uses **throwaway development keys** that anyone can rebuild
from this repository. An image built this way must never face a network you
do not control. A production nsh host is the sealed `server` profile, enrolled
with real keys from nsh-enrol: [nsh-server-runbook.md](nsh-server-runbook.md).
Every desktop release kernel refuses the two capsules
(`make check-nsh-release-refused`).

Each step says whether it has been run. A step marked *pending* has not yet
been run end to end on a booted guest. Run it, then record the result here.

## What a development guest can prove

A development image admits its capsules on their Merkle path alone
(`nonos-dev-attest`), and the kernel records each one in PCR 14 as
path-only (`src/security/capsule_measure/admit.rs`). The client refuses
any PCR 14 capsule that the vendor's STARK proof did not admit
(`nsh_attest` `steps/capsules.rs`, the `path-only` name in
`nsh-v1-refusals.md`). So against a development guest:

- steps 1 to 4 run in full: the guest boots measured, nsh.attest quotes
  the TPM, nsh.front serves the handshake over the loopback forward, and
  nsh-probe checks the host's signature and enrolment and keeps the live
  document;
- step 5 is refused at attestation, before any credential leaves the
  client, with `refused the host before sending any credential: PCR 14
  holds capsules admitted on their path alone, as a development image's
  are; a release host's are admitted by the vendor's proof
  [Host(Attest(PathOnly))]`. That is the correct outcome: a development
  image proves nothing about which capsules run.

A login that runs commands needs an image whose capsules carry their STARK
proofs under the vendor's root, which only the release signing ceremony
makes. Under the session proofs (`nsh_server_proofs` `session_shell`,
`cli`) the same client logs in to the same server and runs the shell.

## What runs where

- **Host.** Linux, x86_64, with `/dev/kvm`, nix, and this checkout. The dev
  shell (`nix develop`) provides QEMU and swtpm. With no `/dev/kvm`, `nix run .#qemu`
  falls back to TCG: the first boot to setup takes tens of minutes, so give
  each wait in a drive script room, and the client `--handshake 300`.
- **Guest.** The `qemu` profile's development twin, with `nsh.front` (holds
  Network, listens on TCP 2222) and `nsh.attest` (holds AttestRead, answers
  each handshake's challenge with the TPM-quoted attestation document). See
  `nsh-front-devkit.md` for the split and why it exists.
- **Client.** `nsh`, from `userland/nsh_client` (features `cli,nonos`),
  run on the host. It reaches the guest through a port forward bound to the
  host's loopback.

## 1. Build the image (pending)

```
nix develop --command python3 tools/nonos-dev-image --profile qemu --nsh
```

`--nsh` works in the throwaway copy under `target/dev/tree` only:

- adds `nonos-capsule-nsh-front` and `nonos-capsule-nsh-attest` to the
  copy's `nonos.toml` features;
- runs the devkit once, which makes fresh keys from the OS generator and
  writes a development machine's records to `target/dev/nsh-records`
  (`roles`, `roles.anchor`, `revocations`) and the matching client files to
  `target/dev/nsh-client`: `user.secret`, `authority.public`, `grant`. Every
  run makes new keys, so always use the client files of the same run as the
  records.

Nothing is baked into the image. The guest makes its own host keys at its
first boot, seals them to its TPM, and writes its enrolment request under
`/nonos/nsh` (nsh-server-runbook.md, section 3); until it holds its records
nsh.front turns each connection away with `UNENROLLED`. The revocation list
the devkit writes is fresh for one day (the role table's limit), so put it
on the machine no more than a day before the login.

On a machine that reaches neither the NONOS package mirror nor the source
recipes' content (DarkFi, SimpleX), add `--offline`: the copy's nonos.toml
then names no package mirror and no source recipes, as a test image does,
and its Marketplace lists no Linux tool. nsh needs neither.

The image is `target/dev/tree/target/release/qemu-dev/nonos.img`.

## 2. Boot it (run, under TCG)

```
nix run .#qemu -- --image target/dev/tree/target/release/qemu-dev/nonos.img \
    --tpm --headless --serial target/qemu/serial.log --forward 2222:2222
```

- `--tpm` attaches a software TPM whose state is kept from boot to boot
  (`target/qemu`), so the attestation key and the measured PCRs stay the same
  across boots. `--fresh` makes a new TPM, and the enrolment below has to be
  made again.
- `--forward 2222:2222` adds `hostfwd=tcp:127.0.0.1:2222-:2222`: the guest's
  listener is reachable from this host only. Do not use make's
  `QEMU_NET_MODE=hostfwd`: it binds every interface and already takes host
  port 2222 for the guest's port 22.
- Set the guest's network to Direct. net.sockets opens a listener only on
  the Direct route; under Nym or Anyone nsh.front never listens.
- Watch the serial log for nsh.attest and then nsh.front coming up in the
  network spawn plan.

On a shared machine, stop only your own QEMU: note its pid when it starts,
and `kill` that pid. `pkill -f qemu-system-x86_64` stops every QEMU on the
machine.

## 3. Probe the live guest (done, under TCG)

A guest that makes its own host keys holds no enrolment until step 4, and
turns the probe away with `UNENROLLED` before HOST_PROOF. Its attestation
key, PCRs and host keys are in its enrolment request instead
(`nsh-enrol inspect --doc enrol.doc`). The probe below is for a guest that
holds its records; the runs recorded here were made on images that baked a
development identity in.

The enrolment certificate binds the TPM's attestation key and PCRs 0, 1, 2
and 7, and the image from step 1 carries the proof spec's. So the client
refuses the live guest at the attestation check, before any credential is
sent. That is the correct outcome. Read the live values without logging in:

```
cd userland/nsh_client
cargo run --release --features cli,nonos --bin nsh-probe -- \
    --connect 127.0.0.1:2222 --host db-1.eu.example \
    --fleet f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1 \
    --authority ../../target/dev/nsh-client/authority.public \
    --key ../../target/dev/nsh-client/user.secret \
    --grant ../../target/dev/nsh-client/grant \
    --out ../../target/dev/nsh-probe [--loader HEX]
```

The probe checks the host's signature and enrolment as `nsh` does, keeps
the HOST_PROOF document and registry entries, and aborts before USER_AUTH.
It writes `doc`, `entries` and `found` to `--out`, and prints the
attestation key, PCRs 0, 1, 2 and 7, each PCR 4 application, PCRs 8 and 9,
and every Network holder with its capability mask. When the loader is known
(`--loader`, or the only PCR 4 application), and exactly one entry holds
nsh.front's mask (`0x1d`, or `0x11d` on a development twin, whose kernel
adds Debug), it also writes `releases`.

The loader's digest is the Authenticode SHA-256 of the image's
`EFI/BOOT/BOOTX64.EFI`, the one PCR 4 application that is not the firmware's.

Probe only over a path you control: the loopback forward on the machine that
runs QEMU, never across a network. The probe takes the attestation key from
whoever answers, checked only against host keys this machine made; anyone
who sits on the path could answer with an attestation key of their own, and
step 4 would enrol it. Compare the `ak` line with the guest's serial console
or the TPM state before enrolling when you can. None of these values is
trusted yet; the next login checks them all against what is made from them.

## 3a. The client up to its first refusal (done, under TCG)

With the `releases` line the probe wrote, run the client of step 5 against
the same guest. It checks, in this order (`nsh_client/src/nonos.rs`,
`host_check.rs`): the document parses and carries this handshake's
challenge; the TPM quote verifies under the document's attestation key;
PCRs 8 and 9 are the release's; the firmware log replays to the quoted
PCRs 0, 1, 2, 4, 7 and 14 and the loader is the PCR 4 application; then
PCR 14's capsules must each be admitted by the vendor's STARK proof. Every
capsule of a development image is admitted on its Merkle path alone, so
the client stops there, before the enrolment's key and PCRs are compared
and before any credential is sent:

```
nsh: refused the host before sending any credential: PCR 14 holds capsules
admitted on their path alone, as a development image's are; a release
host's are admitted by the vendor's proof [Host(Attest(PathOnly))]
```

The guest says `[NSH-FRONT] refused or ended: Aborted(Policy)`, the client
having aborted. Each check of this step has its own name in the brackets: `Log` (the log does not parse), `Replay` (it does not replay
to a quoted PCR), `BootApplication`, `Loader`, `PathOnly`, and `NotVendor`
(no capsule, or one a person's root or a local build admitted). A client
from before that change says `[Host(Attest(Log))]` for all of them; to tell
which there, run `nsh_attest::log::replay` and `nsh_attest::steps::capsules`
on the probe's `doc`. On the guests below every replayed PCR matched, PCR 4
held one application, and `capsules` answered `Pcr14(PathOnly)` for every
capsule (69 on one image, 70 on another). That refusal is by design; how a
development release passes it is a decision not yet taken, and until
then steps 4 to 6 change nothing on a development image: the path-only
refusal comes before the enrolment is compared.

## 4. Enrol it and boot again (pending)

Copy the machine's request (`host.public`, `enrol.nonce`, `enrol.doc` from
`/nonos/nsh`) off the guest into a directory, then:

```
cd userland/nsh_server_proofs
cargo run --bin nsh-devkit -- enrol ../../target/dev/nsh-records \
    ../../target/dev/nsh-client REQ/host.public REQ/enrol.doc LOADER
```

This writes the records again with new keys, and an `enrol` naming the
machine, its key, its PCRs, its other PCR 4 applications as boot
applications, and the host keys it made. The client files in
`target/dev/nsh-client` are written again with it; use the new ones. Put
`enrol`, `roles`, `roles.anchor` and `revocations` under `/nonos/nsh` on the
guest. No rebuild and no reboot: nsh.front reads the records for the next
connection, and its measurement does not depend on them.

## 5. Log in (pending)

```
cd userland/nsh_client
cargo run --release --features cli,nonos --bin nsh -- \
    --connect 127.0.0.1:2222 --host db-1.eu.example \
    --fleet f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1 \
    --authority ../../target/dev/nsh-client/authority.public \
    --releases ../../target/dev/nsh-probe/releases \
    --key ../../target/dev/nsh-client/user.secret \
    --grant ../../target/dev/nsh-client/grant \
    --records records
```

The client checks the host's hybrid signature, its enrolment and its
attestation **before** it sends the user credential. A host that has not
admitted the login 60 seconds after the connection is given up on; a slow
guest under TCG may need more, with `--handshake SECS` (1 to 3600). When
the client refuses the host, or the host refuses the login, `nsh` prints
what failed and what to change, then the refusal's name from
`nsh-v1-refusals.md` in brackets, and exits 255. A connection closed before
the login is said apart from one lost during a session: through the QEMU
forward it is what a guest that does not listen looks like, so check that
the route is Direct. The session then runs
the shared NONOS shell (`nonos_shell::session`):

```
for i in 1 2 3; do echo nsh-$i; done
if test 6 -lt 7; then seq 40 | tail -n 1; fi
date; uptime; mem; ps | grep nsh
exit
```

`help` lists every command. nsh.front holds no filesystem capability, so
`diff` and redirects are refused.

`exit`, Ctrl-] or the end of standard input closes the session; the host
then ends it and sends its record. A host that has not ended a closed
session 60 seconds later is given up on, with `--close-wait SECS` (1 to
3600) to allow more, and no record is kept.

## 6. Check the record (pending)

With `--records`, the client keeps the session's record and the attestation
document beside it. Check them offline with `nsh-verify-record`
(`nsh-v1-record.md`).

## What has been run

- **Host side, over real TCP (done).** `nshd` and `nsh-fixture-client` from
  `userland/nsh_server_proofs`, with attestation enforced by the real
  NONOSATT checker: admitted, a command run (`nsh-live-42`), a clean exit.
  With `NSHD_RECORDS=<dir>`, the host kept the record owner-only in that
  directory.
- **The session, in memory (done).** `nsh_server_proofs` `session_shell`:
  the real client logs in to the real server, and lines run through the
  shared shell exactly as nsh.front wires it.
- **Step 1, up to the seal (done).** `nonos-dev-image --nsh` made the copy
  and its throwaway keys, wrote both features into the copy's `nonos.toml`,
  and baked the devkit identity into the copy's nsh.front. The seal then
  needs every derivation of the image, including the Linux userland, whose
  sources a build machine fetches over the network.
- **The probe and the live enrolment, in memory and over loopback TCP
  (done).** `nsh_server_proofs` `probe` and `devkit_live`.
- **Steps 1 and 2 on a guest (done, under TCG).** `nonos-dev-image
  --profile standard --nsh --offline` sealed in about two hours on 4 cores
  with no KVM; `tools/nonos-drive` booted it with `--forward 2222:2222`
  (QEMU ran `hostfwd=tcp:127.0.0.1:2222-:2222`). The serial log shows
  `[SPAWN] name=nsh.attest ... caps=0x0000000080000019` and
  `[SPAWN] name=nsh.front ... caps=0x000000000000001D`, both
  `[ZK-ATTEST] ok ... vendor`, then the setup wizard. Setup has a lock-screen step (a passphrase of at least 8
  characters, typed twice) after the computer name; a drive script must
  type it. With Direct chosen and applied, the Terminal's `ping 10.0.2.2`
  was answered and `ps` showed nsh.attest and nsh.front idle.
- **Step 3 on a guest (refused).** nsh-probe, before and after Direct:
  `the host closed the connection before its HOST_PROOF`, the guest
  answering the forwarded SYN with a reset. nsh.front never listens on
  these images: every desktop profile, its development twin included,
  builds `nonos-capsule-net-core`, whose `net.tcp` service answers CONNECT,
  SEND, RECV, CLOSE, STATE and POLL and refuses LISTEN and ACCEPT as an
  unknown operation (`capsule_net_core/src/server/handlers/tcp/dispatch.rs`).
  net.sockets reports that to nsh.front as no transport, and nsh.front asks
  again every 500 ms. Only the input test kernels build the separate
  `capsule_net_tcp`, which has both. Steps 3 to 6 wait on a listening
  `net.tcp` in net.core; any capsule holding Network can reach that service
  directly, so it would have to keep the Direct-only rule net.sockets keeps.
  net.core now answers LISTEN and ACCEPT for net.sockets alone and only on
  the Direct route (`capsule_net_core/src/listeners/`); this step is to be
  run again on an image built with it.
- **Step 3 again, with the listener (refused at HOST_PROOF).** An image
  with the passive open and nsh.front's
  reopened listener, booted the same way on 3 CPUs and 6 GiB.
  With Direct chosen nsh.front now listens: nsh-probe's connection through
  127.0.0.1:2222 is taken, its HELLO read and answered. Every probe ended
  at once with `nsh-probe: the host was refused before its attestation:
  Aborted(Busy)`. Before HOST_PROOF the server answers `BUSY` only for
  `Why::Attest` (`nsh_server` `server/proof.rs`): the document or the
  entries nsh.front asked nsh.attest for did not come back. The kernel
  printed no `[ATTEST] refused` line, which `MkAttestDoc` prints on every
  refusal, so the kernel was not asked for a document, or the entries call
  failed; both capsules stayed up (`ps`: nsh.attest idle, nsh.front
  running). nsh.front drops `Event::Refused` and holds no Debug, so the
  host's `Why` is shown nowhere on the guest. Which of the two relayed
  calls fails is the next thing to find; steps 4 to 6 wait on it.
  A second image, a later build, on this host's own QEMU 8.2, OVMF and
  swtpm (TCG, 4 CPUs, 8 GiB) gave the same: before Direct was chosen a
  connection to 2222 closed at once; with Direct the probe was refused
  `Aborted(Busy)` about 50 ms after it connected.
- **Saying which relay fails.** nsh.front writes one line when a relay to
  nsh.attest fails (`[NSH-FRONT] attest relay doc: ...` or `entries`, with
  the reason and how long it took; `relay_said.rs`). `MkDebug` needs the
  Debug capability, so nsh.front takes Debug as an optional capability,
  folded in by `serial_debug_cap()`: a development twin, built with
  `capsule-serial-debug`, grants it, and every image a release loader boots
  is built without that feature (tools/nix/config.nix), so its nsh.front
  holds 0x1D as before. Boot an image with both and read the serial log
  for that line after a probe.
- **Steps 3 and 3a on a guest (done, under TCG).** An image with
  nsh.attest's answers carrying their header, on this host's
  QEMU 8.2, OVMF and swtpm through tools/nonos-qemu, 4 CPUs, 8 GiB, a
  fresh data disk, `--forward 2222:2222`, setup walked by keys with Direct
  (two Down presses on step 6). nsh.attest spawned with caps 0x80000019 and
  nsh.front with 0x11D, both `[ZK-ATTEST] ok ... vendor`. nsh-probe took
  the host's proof in about a second and wrote `doc`, `entries`, `found`
  and `releases`: one PCR 4 application (the loader), PCRs 8 and 9, and 15
  Network holders with nsh.front the one at 0x11d. `nsh` with that
  `releases` file was refused at PCR 14 as above, exit 255. On the image
  before that change the probe was refused BUSY at HOST_PROOF and the guest
  said `[NSH-FRONT] attest relay doc: the answer was not a whole frame`.
- **Quiet clients and reconnects on a guest (done, under TCG).** Images
  from three successive builds booted by tools/nonos-drive on
  the flake's QEMU 11.1.1 with `--forward 2222:2222` and Direct chosen. A
  client that connected and sent nothing was closed after 30.2 s, the
  handshake bound, not at nsh.front's first empty read; probes
  run back to back were each taken by the listener nsh.front keeps open.
  On the second build the serial line read `[NSH-FRONT] attest relay doc: the
  answer was not a whole frame, after 12 ms` on the boot's first probe;
  on the third the probe passed HOST_PROOF with a 27,288-byte document.
- **Driving setup.** Under TCG the lock screen's slow hash can take the
  second entry before the field is ready; check the review screen (step 14
  of 14) reads `Network: direct` before the last Enter, and type the
  passphrase again where the screen still asks for it.
