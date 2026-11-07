# nsh/1 host attestation

The client checks the host before it sends any credential. The checks run
over the `HOST_PROOF` frame ([nsh-v1-wire.md](nsh-v1-wire.md)): the host
keys, the enrolment certificate, the `NONOSATT` version 5 document
([nonosatt-v5.md](nonosatt-v5.md)) and the registry entries.

- The first failed check ends the session with `ABORT(POLICY)`. The host is
  told nothing more.
- The user is shown `POLICY (<name>)`, where `<name>` names the check that
  failed ([nsh-v1-refusals.md](nsh-v1-refusals.md)).
- No credential has been sent when a check fails.

Implemented in: `userland/nsh_attest/src/lib.rs`, `userland/nsh_client/src/client/proof.rs`.

## The host signature

Before step 1, the client verifies the host keys' signature over the
transcript:

```
SHA-512("nsh/1 host-sig\0" || TH(HOST_PROOF without its signatures))
```

Both halves must verify under the host key carried in `HOST_PROOF`. No
other check runs over parts the host keys have not been shown to sign.

Implemented in: `userland/nsh_client/src/client/proof.rs`, `userland/nsh_attest/src/evidence_verify.rs`.

## The challenge

The document must carry, as its challenge, the 32 bytes:

```
challenge = SHA-256("nsh/1 attest\0" || TH(SERVER_HELLO) || H(host keys))
H(host keys) = SHA-512(host Ed25519 key || host ML-DSA-65 key)
```

- `TH(SERVER_HELLO)` covers both randoms and the X-Wing exchange, so a
  document made for another session is refused.
- `H(host keys)` names the host keys, so a document made for another key
  is refused.
- The client reads the challenge from the document itself, at its fixed
  place, and compares it in constant time.

Implemented in: `userland/nsh_crypto/src/challenge.rs`, `userland/nsh_attest/src/steps/challenge.rs`, `userland/nsh_client/src/host_check.rs`. Vector: `userland/nsh_crypto_proofs/vectors/schedule.txt` (`challenge`).

## What the client checks, in order

| Step | Check | Refusal names |
|---|---|---|
| 1 | The document parses as `NONOSATT` version 5. Any other version is refused, versions 3 and 4 included. | `format`, `version` |
| 2 | The document's challenge is this handshake's. | `challenge` |
| 3 | The quote is a TPM quote, binds the document's challenge, registry root and DMA posture, is signed by the document's AK, selects exactly the eight PCRs, and its digest covers the document's PCR values. | `quote`, `binding`, `signer`, `signature`, `selection`, `pcr-digest` |
| 4 | The enrolment certificate: the fleet authority signed it, for this fleet, host, AK and host keys; it has not run out; it is not revoked. | `enrol`, `enrol-authority`, `enrol-signature`, `enrol-fleet`, `enrol-host`, `enrol-ak`, `enrol-host-keys`, `enrol-expired`, `enrol-revoked` |
| 5 | The boot chain: PCR 9 names an accepted release, PCR 8 is that release's, the event log replays to the quoted PCRs, PCR 4 holds the release's loader and only enrolled boot applications, PCR 14 records only vendor-admitted capsules, and PCRs 0, 1, 2 and 7 are the enrolled values. | `pcr9`, `pcr8`, `log`, `replay`, `boot-application`, `loader`, `pcr14`, `path-only`, `developer`, `local-build`, `pcr0`, `pcr1`, `pcr2`, `pcr7` |
| 6 | Device DMA is confined. | `dma` |
| 7 | The registry entries fold to the quoted root, the registry is complete, the count matches, every entry is vendor-admitted, and every entry holding Network is the release's: its `nsh.front`, exactly once, or one of its network stack. | `registry`, `registry-complete`, `capsule-count`, `not-vendor`, `network-capsule`, `front` |
| 8 | The device proof, when the client policy asks for it. | |

Within step 5 the checks run in the order: PCR 9, PCR 8, the log and its
replay, PCR 4's applications, the loader, PCR 14's capsules, then the
firmware PCRs.

The step numbers are fixed. The order in which the checks run differs
between the two implementations. Either way, the first failure ends the
session with `ABORT(POLICY)`.

| Implementation | Order |
|---|---|
| `nsh_attest::check` | 1, 2, 3, 4, 5 (release, log, PCR 14, firmware PCRs), 6, 7 |
| `nsh_client` with `NonosAttester` | host signature, 1, 2, 3, 5 (release, log, PCR 14), 6, 7, the challenge read again, 4, 5 (firmware PCRs) |

> **Not settled by the code:** step 8, the device proof. The code names
> the step and its refusal (`AttestRefusal::DeviceProof`), and no checker
> implements it. Its content and the policy field that asks for it are not
> defined.

Implemented in: `userland/nsh_attest/src/check.rs`, `userland/nsh_attest/src/steps/`, `userland/nsh_attest/src/refusal_kind.rs`, `userland/nsh_attest/src/refusal_name.rs`.

### Step 1: the document

The document is read exactly as [nonosatt-v5.md](nonosatt-v5.md) lays it
out. These layout rules are part of step 1:

- Big-endian. Every length is capped. Nothing may trail.
- The whole document is at most 1 114 112 bytes.
- Flags (`registry_complete`, `iommu_enforcing`) are 0 or 1.
- `iommu_vendor` is at most 3.
- `pcr_count` is exactly 8, with indices 0, 1, 2, 4, 7, 8, 9, 14 in that order.
- `log` is 0 to 1 048 576 bytes. An empty log parses here and is refused
  in step 5 as `log`.
- `attest` is 1 to 1024 bytes. `signature` is 1 to 72 bytes.
- `ak_public` is exactly 64 bytes.
- The version is read before any field after it. A version other than 5
  is refused as `version`.

Implemented in: `userland/nsh_attest/src/doc/parse.rs`, `userland/nsh_attest/src/consts.rs`.

### Step 3: the quote

The quote is `TPMS_ATTEST` as the TPM signed it, and the AK's
`TPMT_SIGNATURE` over it. TPM structures are big-endian.

`TPMS_ATTEST` is read to its last byte:

| Field | Size | Rule |
|---|---|---|
| `magic` | `u32` | `0xFF544347` (`TPM_GENERATED_VALUE`), else `quote` |
| `type` | `u16` | `0x8018` (`TPM_ST_ATTEST_QUOTE`), else `quote` |
| `qualifiedSigner` | `u16` size (0 to 66), bytes | |
| `extraData` | `u16` size (0 to 66), bytes | |
| `clockInfo` | `u64` clock, `u32` resetCount, `u32` restartCount, `u8` safe (0 or 1) | read past |
| `firmwareVersion` | `u64` | read past |
| selections | `u32` count (1 to 16), then each: `u16` hash, `u8` sizeofSelect (0 to 4), bitmap | |
| `pcrDigest` | `u16` size (0 to 66), bytes | |

Then, in order:

1. `extraData` must equal `qualifying_data(challenge, registry_root, dma)`
   recomputed by the client from the document ([nonosatt-v5.md](nonosatt-v5.md),
   "The binding"). Else `binding`.
2. `qualifiedSigner` must be a SHA-256 name: 34 bytes, the first two
   `0x000B`. Else `signer`.
3. The signature must be `TPMT_SIGNATURE` with `sigAlg` = `0x0018`
   (ECDSA) and `hash` = `0x000B` (SHA-256), then `r` and `s` each as a
   `u16` size (1 to 32) and bytes, left-padded to 32, and nothing after.
   It must verify as ECDSA P-256 over `SHA-256(TPMS_ATTEST)` under
   `ak_public` (x then y): `r` and `s` in 1 to n−1, the key on the curve.
   Else `signature`.
4. There must be exactly one selection, in the SHA-256 bank (`0x000B`),
   with bitmap `97 43 00` (PCRs 0, 1, 2, 4, 7, 8, 9, 14). Else `selection`.
5. `pcrDigest` must equal SHA-256 of the eight PCR values in the document,
   in document order. Else `pcr-digest`.

> **Not settled by the code:** `qualifiedSigner`. The code holds it to the
> shape of a SHA-256 name (34 bytes, algorithm `0x000B`). Its exact value
> is not fixed.

Implemented in: `userland/nsh_attest/src/steps/quote.rs`, `userland/nsh_attest/src/quote/`, `userland/nsh_attest/src/p256/verify.rs`.

### Step 4: the enrolment certificate

The certificate is `nsh-enrol/1` (below). The client checks:

| Check | Refusal |
|---|---|
| It parses. | `enrol` |
| `authority` names a key id the client policy holds. | `enrol-authority` |
| Both halves of `sig` verify under that key over `SHA-512("nsh/1 enrol\0" \|\| signed bytes)`. | `enrol-signature` |
| `fleet_id` is the policy's fleet. | `enrol-fleet` |
| `host` is the host name the user typed. | `enrol-host` |
| `ak_public` is the document's AK. | `enrol-ak` |
| `host_keys` is `H(host keys)` of `HOST_PROOF`. | `enrol-host-keys` |
| `now_ms < not_after_ms`. A certificate is refused from `now_ms >= not_after_ms`. | `enrol-expired` |
| SHA-512 of the whole certificate (signed bytes and both signatures) is not in the policy's revoked list. | `enrol-revoked` |

- The authority and signature checks come first. Then the rest, in the
  order above.
- Host names are compared byte for byte. The client lowercases the typed
  name first.
- A record checked offline has no typed host name and no clock: `host`
  and expiry are checked online only. Revocation is checked online and
  offline.

Implemented in: `userland/nsh_attest/src/steps/enrol.rs`, `userland/nsh_client/src/enrol_check.rs`.

### Step 5: the boot chain

**Release.** PCR 9 names the admitted kernel and so picks the one accepted
release. A PCR 9 that is all zeros, or matches no release, is `pcr9`. PCR 8
(the loader's own measurement) must then be that release's, and not all
zeros, else `pcr8`. A release carries the loader's Authenticode digest,
PCR 8, PCR 9 and the `nsh.front` measurement, not a PCR 4 or PCR 14 value.
It carries no rollback index: the document has none to compare, and the
loader enforces it against the TPM before PCR 9 can name the kernel.

**Replay.** The event log must parse ([nonosatt-v5.md](nonosatt-v5.md),
"The event log"), else `log`. PCRs 0, 1, 2, 4, 7 and 14 must each equal the
log's replay, else `replay`. PCRs 8 and 9 are extended by NØNOS code
outside the log and are held to the release instead.

- Every PCR starts at zero. Each extending event, in log order, sets
  `pcr = SHA-256(pcr || event's SHA-256 digest)`. `EV_NO_ACTION` is logged
  and never extended.
- The header must be PCR 0, `EV_NO_ACTION`, a 20-byte zero digest, and a
  body that is exactly a `TCG_EfiSpecIDEvent`: signature
  `Spec ID Event03\0`, version 2.0, `uintnSize` 1 or 2, at most 8 banks,
  each bank once with a digest size of 1 to 64, a 32-byte SHA-256 bank
  among them, then the vendor bytes and nothing after.
- Every event carries exactly the declared banks, each once.
- A PCR index must be below 24.
- A startup-locality event (`EV_NO_ACTION` whose body starts
  `StartupLocality\0`) is honoured once, in PCR 0, before any extend of
  PCR 0, with a 17-byte body naming locality 0, 3 or 4. PCR 0 then starts
  with its last byte set to that locality. Any other placement is refused.
- Caps are the loader's: 1 048 576 bytes of log, 4096 events after the
  header, 65 536 bytes per event body. A log over a cap is refused, never
  cut.

**PCR 4.** PCR 4 differs between machines, so it is pinned through the log
rather than to a release value:

- the log's replay must equal the quoted PCR 4;
- PCR 4 may hold only `EV_NO_ACTION` (`0x00000003`), `EV_SEPARATOR`
  (`0x00000004`), `EV_EFI_ACTION` (`0x80000007`) and
  `EV_EFI_BOOT_SERVICES_APPLICATION` (`0x80000003`) events. The TPM extends
  digests and never types, so any other type could hide a measurement;
- a separator or action must carry the SHA-256 of its own body;
- an event that breaks either of these two rules is refused as `log`;
- every application digest in PCR 4 must be the release's loader or a boot
  application the machine's enrolment lists (its boot manager, pinned and
  signed at enrolment), else `boot-application`;
- the loader must be present among them, else `loader`. Which application
  ran last is not enough: an earlier application could load the loader
  and never start it.

**PCR 14.** PCR 14 holds the kernel's own events, after the firmware's:
one per distinct admitted capsule ([nonosatt-v5.md](nonosatt-v5.md), "PCR 14").

- An event in PCR 14, or of type `0x4E4F4E01`, must be both, with a
  65-byte body whose SHA-256 is the event's digest, a kind of 1 to 4, and
  the path-only tag's digest as its verifier exactly when its kind is 2.
  Else `log`.
- A release host must have admitted at least one capsule, else `pcr14`.
- Every capsule must be kind 1 (vendor proof). The first that is not is
  named: `path-only` (kind 2), `developer` (kind 3), `local-build` (kind 4).

**Firmware PCRs.** PCRs 0, 1, 2 and 7 must be the values the enrolment
certificate fixed, in that order. The first that differs is named `pcr0`,
`pcr1`, `pcr2` or `pcr7`. A firmware update moves them, and means the
machine is enrolled again.

Implemented in: `userland/nsh_attest/src/steps/release.rs`, `userland/nsh_attest/src/steps/log.rs`, `userland/nsh_attest/src/steps/capsules.rs`, `userland/nsh_attest/src/steps/firmware.rs`, `userland/nsh_attest/src/log/`.

### Step 6: DMA

All of these must hold, else `dma`:

- `iommu_vendor` is not 0 (an IOMMU is present);
- `iommu_enforcing` is 1;
- `unconfined_grants` is 0;
- `identity_devices` is 0.

All four values are bound into the quote's `extraData`.

Implemented in: `userland/nsh_attest/src/steps/dma.rs`.

### Step 7: the registry

The entries come from `HOST_PROOF` ([nonosatt-v5.md](nonosatt-v5.md),
"Registry entries").

1. The entries fold to the document's `registry_root`, else `registry`.
   The root is bound by the quote.
2. `registry_complete` is 1, else `registry-complete`. The flag is not
   bound into the quote and is not the evidence: the kernel quotes only
   when its registry is complete, checked before and after the quote, and
   so always writes 1.
3. `capsule_count` equals the number of entries sent, else
   `capsule-count`. `capsule_count` is not bound into the quote; the
   entries are.
4. Every entry's authority byte is 0 (vendor), Network or not, else
   `not-vendor`. This matches step 5's rule that PCR 14's capsules are all
   kind 1.
5. Every entry holding Network (bit 2 of the capability mask) has the
   release's `nsh.front` measurement or one in the release's network set,
   else `network-capsule`.
6. Exactly one entry holding Network has the `nsh.front` measurement, else
   `front`.

A record checked offline carries no entries. Step 7 then binds only the
root, through the quote ([nsh-v1-record.md](nsh-v1-record.md), "Checking a record offline").

Implemented in: `userland/nsh_attest/src/steps/registry.rs`.

## nsh-enrol/1

The enrolment certificate: the fleet's enrolment authority states that
this TPM attestation key, these host keys, this host name, these firmware
PCRs and these boot applications belong together, until `not_after_ms`.
All integers are little-endian.

| Field | Size | Rule |
|---|---|---|
| magic | 4 | `NSHE` |
| version | `u16` | 1 |
| reserved | 2 | zero |
| `fleet_id` | 16 | not all zeros |
| `host` | `bytes8(1..253)` | host name shape |
| `ak_public` | 64 | the quote's P-256 AK, x then y |
| `host_keys` | 64 | `H(host keys)`: SHA-512 of the host `HybridPublic` |
| `pcr0` | 32 | SHA-256 bank |
| `pcr1` | 32 | |
| `pcr2` | 32 | |
| `pcr7` | 32 | |
| boot apps version | `u8` | 1 |
| boot apps count | `u8` | 0 to 8 |
| boot apps | count × 32 | SHA-256 Authenticode digests, none all zeros, none twice, in the order they ran |
| `not_after_ms` | `u64` | |
| `authority` | 32 | key id of the enrolment authority |
| `sig` | `HybridSig`, 3373 | |

- The signed bytes are every field before `sig`. The authority signs
  `SHA-512("nsh/1 enrol\0" || signed bytes)` with both halves.
- The boot applications section carries its own version. A client that
  does not know a later version refuses the certificate rather than
  misreading it.
- The largest certificate is 4205 bytes.
- A revocation names a certificate by SHA-512 of all its bytes.

Implemented in: `userland/nsh_wire/src/enrol.rs`, `userland/nsh_wire/src/enrol_parse.rs`, `userland/nsh_wire/src/enrol/boot.rs`, `tools/nsh-enrol/src/encode.rs`.

## The client policy

The client holds `nsh-client-policy/1`. Once its signature has been
checked it gives:

| Item | Rule |
|---|---|
| `fleet_id` | 16 bytes, not all zeros |
| enrolment authorities | 1 to 16 hybrid public keys, none twice (by key id) |
| accepted releases | 1 to 64; each has a version, PCR 8, PCR 9, loader digest, `nsh.front` measurement and network set (0 to 32 measurements of the release's network stack, each once, never the front), none all zeros; no two share a PCR 9 |
| revoked enrolments | 0 to 65 536 SHA-512 certificate digests |

A policy that could never admit a host, or could admit an unmeasured one,
is refused before use.

> **Not settled by the code:** the `nsh-client-policy/1` file. The code
> defines the policy's contents once read and checked. It does not define
> the file's byte layout, its signer or its signature label.

Implemented in: `userland/nsh_attest/src/policy/`, `userland/nsh_client/src/policy.rs`.

## The client's split of the steps

`nsh_client` takes the attestation check through its `Attester`
interface. In go the document, this handshake's challenge, the registry
entries and the enrolment certificate as sent, not yet checked. Out come
the AK, the quoted PCRs 0, 1, 2 and 7, and the offset of the challenge in
the document, or a refusal.

- The attester performs steps 1, 2, 3, the release, log and PCR 14 parts
  of step 5, 6, 7, and 8 when the policy asks. It reads the certificate
  only for its boot applications: the ones PCR 4 may hold besides the
  loader.
- The client then reads the challenge from the document at the offset
  given and compares it itself, so an attester cannot echo back the
  challenge it was handed.
- The client performs step 4 and the firmware half of step 5 itself, in
  that order, including the certificate's signature.
- The same check serves the offline record checker, with no registry
  entries, no typed host name and no clock.
- `NonosAttester` (feature `nonos`) is `nsh_attest` as the client's
  attester. It runs steps 1, 2, 3, 5 (release, log, PCR 14), 6 and 7, in
  that order, against the fleet's releases. It runs step 7 only when
  registry entries are given. It does not run step 8.

Implemented in: `userland/nsh_client/src/attest.rs`, `userland/nsh_client/src/host_check.rs`, `userland/nsh_client/src/nonos.rs`.

## Enrolment

A machine is enrolled by the fleet operator with `nsh-enrol`, on the
operator's own machine.

1. **keygen.** A new enrolment authority key: `authority.secret` (mode
   0600) and `authority.public` (1984 bytes, Ed25519 then ML-DSA-65). The
   tool prints the key id.
2. **challenge.** A fresh 32-byte nonce and the enrolment challenge for
   the machine's host keys:
   `SHA-256("nsh-enrol challenge\0" || nonce || H(host keys))`.
   The machine is asked for a `NONOSATT` version 5 document over it.
3. **body.** From the document, the nonce, the AK, the host keys, the host
   name, the fleet id, the release loader digests, the expected boot
   applications, a validity of 1 to 3650 days and the authority's public
   key, the tool makes every field of the certificate before `sig`. It
   refuses unless:
   - the document parses as version 5, answers the enrolment challenge,
     its quote passes step 3, and its log replays to PCRs 0, 1, 2, 4, 7
     and 14;
   - the document's AK is the one given, the AK the registrar activated;
   - exactly one of the given release loaders is in PCR 4;
   - every other PCR 4 application is one the operator expects and is not
     all zeros; these become the boot applications, each once, in the
     order they ran, at most 8;
   - the host name has the host shape, the fleet id is not all zeros, and
     `not_after_ms` is in the future.

   PCRs 0, 1, 2 and 7 are taken from the document. `authority` is the key
   id of the authority's public key. The body is shown with its SHA-256.
4. **sign.** Only the body whose SHA-256 is given as `--expect`, only
   while it has not run out, and only if its `authority` is this key's id.
   Both halves sign `SHA-512("nsh/1 enrol\0" || body)`. The certificate is
   then parsed and verified as the client will, shown, and written.
5. **show** and **inspect** check and describe a certificate or a document.

- The tool does not show that the AK lives in the machine's TPM. That is
  the registrar's job: MakeCredential to the machine's EK, answered by
  `src/security/tpm/enroll` (`activate_credential`).
- A file is written whole or not at all, and never over an existing one.

Implemented in: `tools/nsh-enrol/src/`, `tools/nsh-enrol/README.md`.
