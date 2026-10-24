# nsh v1: attestation before any credential

## What the server sends: `HOST_PROOF` (type 0x10)

| Field | Size |
|---|---|
| host Ed25519 public key | 32 |
| host ML-DSA-65 public key | 1952 |
| enrolment certificate | `bytes16` (below) |
| attestation document, `NONOSATT` from `MkAttestDoc` | u32 length, then the bytes; at most 196608 |
| registry entries from `MkAttestEntries` | u32 length, then 45 × n bytes, with n from 1 to 1024 |
| Ed25519 signature | 64 |
| ML-DSA-65 signature | 3309 |

The two signatures are made by the host keys over
`H("nsh/1 host-sig\0" || TH(HOST_PROOF without its signatures))`. Both
signatures must verify.

## The challenge, which binds this session

`challenge = SHA-256("nsh/1 attest\0" || TH(SERVER_HELLO) || H(host keys))`

The challenge is 32 bytes. Here `H(host keys)` is SHA-512 over the
Ed25519 key followed by the ML-DSA-65 key. The kernel folds the challenge
into the quote's `qualifyingData` through `nonos.attest.bind.v2`
(`src/security/attest_doc/binding.rs`). The quote is therefore fresh,
because it depends on `client_random`, and it names the host keys. A key
lifted from this host and used on another machine has no quote naming it
for that session.

## What the client checks, in order

The first failure ends the session with `ABORT(POLICY)`. Nothing is left
to the user's judgement and nothing is skipped.

1. The document parses as `NONOSATT` version 4 or later. Version 3 is
   refused: it does not quote the NONOS chain (kernel change K1).
2. The document's challenge equals the challenge computed above.
3. `TPMS_ATTEST` is checked:
   - it is a quote (`TPM_GENERATED`, `ST_ATTEST_QUOTE`);
   - its `extraData` equals `qualifying_data(challenge, registry_root, dma)`,
     recomputed by the client;
   - the AK signature (ECDSA P-256, SHA-256) verifies with the document's
     `ak_public`.
4. The enrolment certificate is checked:
   - it is signed with the fleet enrolment authority's hybrid key, from the
     policy;
   - it names this `ak_public`, H(host keys), the host name the user typed
     and the fleet id;
   - it has not expired.
5. The quoted PCRs are checked:
   - PCRs 4, 9 and 14 must equal one accepted release's values from the
     policy;
   - PCRs 0, 1, 2 and 7 must equal the values in the enrolment certificate,
     which were taken when the machine was enrolled.

   A firmware update changes PCRs 0 to 7, so it means the machine is
   enrolled again.
6. The DMA posture: `iommu_enforcing = 1` and `unconfined_grants = 0`.
7. The registry is checked:
   - the entries fold to `registry_root` (`attest_receipt::verify_root`);
   - `registry_complete = 1`;
   - the entry for `nsh.front` holds the release's measurement.
8. Optional, if the policy asks for it: the anonymous device proof from
   `nonos-device-attest`. It is off by default in v1, because it is not yet
   produced on demand.

## The client policy, `nsh-client-policy/1`

The policy is a file signed with the operator's own hybrid key. The client
refuses a policy whose signature is bad. It holds:

- the fleet id;
- the enrolment authority's keys;
- the accepted releases, each given as its version, its rollback index and
  its PCR 4, 9 and 14 values;
- whether the device proof is required;
- the role authority keys, which the client shows to the user but does not
  enforce.

## The enrolment certificate, `nsh-enrol/1`

| Field | Size |
|---|---|
| magic `"NSHE"` | 4 |
| version = 1 | u16 |
| reserved = 0 | u16 |
| fleet_id, never all zeros | 16 |
| host name | `bytes8`: labels of 1 to 63 of `[a-z0-9-]`, no label starting or ending with `-`, at most 253 bytes |
| `ak_public` | 64 |
| H(host keys) | 64 |
| PCR 0, 1, 2 and 7 | 4 × 32 |
| not_after_ms | u64 |
| authority key id | 32 |
| Ed25519 signature | 64 |
| ML-DSA-65 signature | 3309 |

The signatures are over `H("nsh/1 enrol\0" || every field before them)`.
Enrolment uses the TPM's existing EK and AK flow (`src/security/tpm/enroll`,
`SYS_ENROLL`) to show the authority that this AK is resident in this TPM.

## Why the attester is a separate capsule

`can_attest_doc` refuses any caller holding Network. That is the right rule,
and nsh keeps it:

- `nsh.attest` holds AttestRead and no Network. It answers one IPC call,
  "a document for this challenge".
- `nsh.front` holds Network and relays the document.

Both appear in the registry. A modified front would show up as a different
measurement in its own registry entry, and step 7 checks that entry.
