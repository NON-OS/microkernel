# nsh in the server profile: the runbook

This runbook covers a NØNOS image that serves nsh/1 for real: the `standard`
profile, which carries the nsh capsules, or the `server` profile, which is the
same system, sealed with the release keys like every other release image, and
the operator's steps to enrol each machine that boots it. The development
twin of the same profile is the test image (section 6); it uses throwaway
keys only.

## 1. What ships, and where the keys live

The `server` profile (`tools/nix/config.nix`) is the `standard` system
with `nonos-server`, which turns on both nsh capsules:

- **nsh.front** holds CoreExec, Network, IPC and Memory, and nothing else.
  It listens on TCP 2222 while the chosen network is Direct, never through
  Nym or Anyone, and drives `nsh_server` for one connection at a time. It
  holds **no host secret** and compiles no identity in: there is no
  `include_bytes!` in it and no identity directory beside it.
- **nsh.attest** holds CoreExec, IPC, Memory, Crypto, FileSystem and
  AttestRead, and **not** Network. It answers nsh.front alone (the pid the
  kernel bound the `nsh.front` name to). It quotes the TPM for each
  handshake, as before; it holds the host's hybrid keys and signs for
  nsh.front; and it reads the machine's own records for each connection.

The keys sit in nsh.attest rather than in a third capsule. The split the
kernel enforces is that no process holding Network is quoted to; the host
keys need the same rule, since a process that reaches the network and
holds the keys could carry them away. nsh.attest already holds no Network
and already answers only nsh.front, so the keys join the one capsule whose
answers are bound to the handshake anyway: the signature over the
transcript and the quote over the same transcript come from the same
place. A separate nsh.keys would add a capsule, a trust set and an IPC hop
without narrowing anything: a process that took over either capsule could
already answer as the host for as long as it ran, and neither can reach the
network. nsh.front can ask nsh.attest to sign a 64-byte digest; it can never
read the keys.

The desktop profiles (`standard`, `hardened`, `airgapped`, `qemu`, `core`)
refuse both capsules: `tools/nix/config.nix` refuses the features beside a
release loader without `nonos-server`, and the kernel refuses them at
compile time (`src/lib.rs`). `make check-nsh-release-refused` proves both
directions: each nsh feature refused in a production and a release kernel
without `nonos-server`, both allowed with it, both allowed in a
development twin, and nsh.front refused without nsh.attest.

## 2. The seal

The build has no input that is an uncommitted secret. The nsh capsules
build from committed files alone, so the seal's build of every capsule runs
with no identity file present.

1. `tools/nix/capsules.json` lists 168 release capsules (`dev_only`
   false), nsh-attest and nsh-front among them; the two left `dev_only`
   are shield-vectors and attack, which the release filter keeps out.
2. One run seals the profile: `make seal PROFILE=server`. It signs every
   release capsule with Ed25519 and ML-DSA-65 and STARK-enrols it, nsh.front
   and nsh.attest with the rest, and writes their trust sets under
   `nonos-data/trust/capsules/` (`nsh_front.*`, `nsh_attest.*`).
3. Once those trust sets are committed, take `nsh-attest` and `nsh-front`
   out of `scripts/baselines/unsealed.txt` and out of the unsealed list in
   `docs/build/feature-inventory.md`: `scripts/check_release_trust.py`
   fails while a sealed slug is still listed.
4. `make build PROFILE=server` writes `receipts/server.json`.

Nothing here touches a key the seal does not already hold. No host key,
enrolment, role table or revocation list is an input of the build or a
byte of the image.

## 3. First boot of a machine

Boot the image on a machine with a TPM 2.0, firmware or discrete (a vTPM
under QEMU: `--tpm`).

1. Walk first-boot setup. Choose **Direct** on the network step, and a mode
   that keeps state: the host keys are kept in the store only once setup
   has chosen to keep.
2. nsh.attest makes the host keys on the machine: an Ed25519 seed and an
   ML-DSA-65 key pair from the kernel's generator. It seals them under the
   TPM's machine key, derived under a label bound to nsh.attest's own
   namespace (`capsule_nsh_attest/src/keys/label.rs`), so no other capsule
   can derive the key that opens them, and keeps the sealed file at
   `/nonos/nsh/host.keys`. Nothing is ever written in the clear. Until the
   store keeps what is written, the keys are held in memory and kept as
   soon as it does, so the keys the operator enrols are the ones kept.
3. Once the keys are kept and the machine holds no enrolment, nsh.attest
   writes the **enrolment request** under `/nonos/nsh`:
   - `host.public`: the host keys, the Ed25519 key then the ML-DSA-65 key
     (1984 bytes), as `nsh-enrol --host-keys` reads them;
   - `enrol.nonce`: a nonce from the kernel's generator, in hex;
   - `enrol.doc`: the kernel's NONOSATT v5 document, quoted by the TPM over
     `SHA-256("nsh-enrol challenge\0" || nonce || H(host keys))`, the
     challenge `nsh-enrol challenge` computes.

   So one image enrols on any number of machines: each makes its own keys
   and its own request.
4. Until it is enrolled, nsh.front turns every connection away before keys
   with `UNENROLLED`, and the client says so (section 7).

On a development twin every step is said on the serial line, with the host
key id (SHA-256 of `host.public`), for example:

```
[NSH-ATTEST] host keys made on this machine at first boot, sealed to the TPM and kept, key id 3f0c...
[NSH-ATTEST] enrolment request written to /nonos/nsh: host.public, enrol.nonce, enrol.doc, key id 3f0c...
[NSH-FRONT] turned away: not enrolled, /nonos/nsh holds no enrol roles roles.anchor revocations
```

A release image writes no capsule line to the serial console; the client
is told the refusal instead.

### No TPM

With no TPM (QEMU with no vTPM, most development machines), nsh.attest
makes no keys, claims no attestation and fakes nothing: the machine key
call answers that there is no TPM, and nsh.attest says so once:

```
[NSH-ATTEST] no TPM: no host keys and no attestation; nsh serves no one
```

nsh.front turns every connection away with `UNATTESTED`.

### A new kernel or firmware

The machine key belongs to the boot state. After an update that changes
it, the sealed keys no longer open: nsh.attest sets them aside as
`/nonos/nsh/host.keys.prev`, makes new keys, and writes a new request.
Enrol the machine again (section 4); the old certificate names the old
keys and PCRs, and the client refuses it.

## 4. Enrol the machine (operator, off the machine)

Copy the three request files off the machine (the Files app, or the
Terminal, onto removable media). On the operator's own machine, with
`tools/nsh-enrol`:

```
nsh-enrol inspect --doc enrol.doc
nsh-enrol body --doc enrol.doc --nonce "$(cat enrol.nonce)" --ak AK_HEX \
    --host-keys host.public --host NAME --fleet FLEET_HEX \
    --loader LOADER_HEX [--boot-app HEX ...] --days N \
    --authority authority.public --out body
nsh-enrol sign --body body --expect BODY_SHA256 --key authority.secret --out enrol
```

`inspect` prints the attestation key the TPM quoted with. Pass `body` only
an AK the registrar activated against the machine's EK
(`src/security/tpm/enroll`, `activate_credential`); `nsh-enrol` checks the
quote, the binding to these host keys and the event log, not that the AK is
the TPM's own. The nonce is the machine's: the document is as fresh as the
machine's first boot, and it names exactly the keys in `host.public`.

## 5. The machine's records

nsh.attest reads four files from `/nonos/nsh` for **every** connection and
hands them to nsh.front, which checks them before it serves:

| file | what | made by |
|---|---|---|
| `enrol` | the machine's `nsh-enrol/1` certificate | `nsh-enrol sign` (section 4) |
| `roles` | the signed role table (`nsh-policy/1`) | `nsh-roles table` |
| `roles.anchor` | the key id the table is pinned under: the hex `nsh-roles table` prints, or the 32 bytes raw | `nsh-roles table` |
| `revocations` | the signed revocation list (`nsh-revoke/1`) | `nsh-roles revoke` |

Put them on the machine as the request came off it. Each is signed or
pinned data: a file that does not open is refused, never trusted.

**Freshness.** The role table says how long a revocation list is trusted
(`revocation_max_age_ms`, at most one day). The rule is applied to the list
the machine holds, at each login, against the machine's clock: a stale
list refuses every login `REVOCATION_STALE` until a fresh one is put in
place. The image itself never ages, since it carries no list: refresh
`/nonos/nsh/revocations` with `nsh-roles revoke` within the table's age,
and the next login reads it, with no reboot.

**Rollback.** nsh.front keeps the newest table serial and list issue time
it has served under this boot, and turns away a table or list older than
either (`capsule_nsh_front/src/marks.rs`). Across a reboot the freshness
rule keeps an old list out.

## 6. The development twin (test image)

Build the profile's development twin with the scratch keys CI uses, in the
copy under `target/dev/tree`; it is a test artifact, never committed,
never signed with release keys, never sealed:

```
nix develop --command python3 tools/nonos-dev-image --profile server --offline
```

or any profile's twin with `--nsh`, which adds the two capsules. With
`--nsh`, `nsh-devkit` writes a development machine's records to
`target/dev/nsh-records` (`roles`, `roles.anchor`, `revocations`) and the
client files to `target/dev/nsh-client`; given the machine's request
(`--nsh-request DIR --nsh-loader HEX`), it also writes the `enrol` for
that machine's own keys. Nothing goes into the image.

Boot it with a vTPM and the 127.0.0.1-only forward; `tools/nonos-drive`
attaches the software TPM itself and walks setup by its steps:

```
tools/nonos-drive --image target/dev/tree/target/release/server-dev/nonos.img \
    --out target/dev/drive --qemu-arg=--forward --qemu-arg=2222:2222 --steps FILE
```

A development twin admits its capsules on their Merkle path alone, so the
client refuses it at PCR 14 (`[Host(Attest(PathOnly))]`) before any
credential is sent, as `nsh-vps-runbook.md` describes. A sealed server
image's capsules are vendor-admitted, and the same client passes that
check with no exception for path-only capsules.

## 7. What the client says

| the host | `nsh` says |
|---|---|
| has no TPM | `the host turned the connection away: it has no TPM to attest with, so it serves no one; boot it with a TPM (a vTPM under QEMU)` |
| holds no enrolment or role table, or ones that do not open | `the host turned the connection away: it holds no enrolment certificate or role table yet; enrol the machine with nsh-enrol and put its records under /nonos/nsh` |
| nsh.attest did not answer now | `it is serving all the sessions it takes; try again later` |
| holds a stale revocation list | `the host refused the login: its own revocation list is out of date; ...` |
| is a development twin | `refused the host before sending any credential: PCR 14 holds capsules admitted on their path alone ...` |

Each ends with the refusal's name in brackets and exit status 255.

Implemented in: `userland/capsule_nsh_attest/src/keys/`,
`userland/capsule_nsh_attest/src/request.rs`,
`userland/capsule_nsh_attest/src/records.rs`,
`userland/capsule_nsh_front/src/host.rs`,
`userland/capsule_nsh_front/src/session.rs`,
`userland/capsule_nsh_front/src/unready.rs`,
`userland/nsh_front_wire/src/records.rs`.

## 8. What has been run

- **The development twin on a guest (done, under TCG), 2026-11-06.** The
  image `tools/nonos-dev-image --profile server --nsh --offline` from
  the release tree, every capsule built from committed files with no identity
  file present; booted by `tools/nonos-drive` with a software TPM, 4 CPUs,
  8 GiB and `--forward 2222:2222` (QEMU ran
  `hostfwd=tcp:127.0.0.1:2222-:2222`); setup walked by keys, Amnesic, the
  route Direct (the review read `Network: direct`). The serial line read
  `[SPAWN] name=nsh.attest ... caps=0x0000000080000179` and
  `[SPAWN] name=nsh.front ... caps=0x000000000000011D`, both
  `[ZK-ATTEST] ok ... vendor`, then
  `[NSH-ATTEST] host keys made on this machine, sealed to the TPM; this boot
  keeps nothing yet, so they are kept once it does, key id 108d3b44...`.
  The store had not settled at the first ask, and nsh.attest said so and
  asked again. `nsh` against 127.0.0.1:2222 exited 255 with
  `the host turned the connection away: it holds no enrolment certificate
  or role table yet; enrol the machine with nsh-enrol and put its records
  under /nonos/nsh [Aborted(Unenrolled)]`, and the guest said
  `[NSH-FRONT] turned away: not enrolled, /nonos/nsh holds no enrol roles
  roles.anchor revocations`. Three connections back to back were each
  answered in about 20 ms, and `nsh-probe` was turned away the same way.
  An amnesic boot keeps nothing, so its keys stay in memory and no request
  is written; a machine that keeps state writes it once setup has chosen to
  keep.
- **Not run on a guest:** a machine that keeps state, its request taken off
  and its records put back, and a login that runs commands. The second
  needs the sealed server image, whose capsules are vendor-admitted; a
  development twin is refused at PCR 14 by design. On the host the same
  path is proved: `nsh_server_proofs` `devkit_cli` logs the real client in
  over TCP with records and keys made as the machine makes them.
