# nsh/1

nsh/1 gives an operator a shell on a NØNOS machine. The client proves the
machine before it proves the user. No credential leaves the client until
the host has passed every check.

The parts:

- [nsh-v1-wire.md](nsh-v1-wire.md): framing, the handshake frames, keys.
- [nsh-v1-wire-session.md](nsh-v1-wire-session.md): the session frames and reason codes.
- [nsh-v1-attest.md](nsh-v1-attest.md): how the client checks a host.
- [nonosatt-v5.md](nonosatt-v5.md): the attestation document.
- [nsh-v1-roles.md](nsh-v1-roles.md): grants, the role table, the server's checks.
- [nsh-v1-record.md](nsh-v1-record.md): the session record.
- [nsh-v1-refusals.md](nsh-v1-refusals.md): every refusal and what each side is told.

## Roles

| Party | What it is | What it holds |
|---|---|---|
| Host | An attested NØNOS machine that serves nsh. | A hybrid host key pair, an enrolment certificate (`nsh-enrol/1`) signed by the fleet's enrolment authority, a TPM attestation key, a signed role table (`nsh-policy/1`) and a signed revocation list (`nsh-revoke/1`). |
| Client | The operator's machine. | A client policy: the fleet id, the enrolment authorities' keys, the accepted releases and the withdrawn enrolments. |
| User | A person who logs in through the client. | A hybrid user key pair and a base role grant (`nsh-grant/1`) for that key; optionally escalation grants. |

- The client checks the host before it sends any credential: the host's
  signature over the transcript, the attestation document for this
  handshake's challenge, the enrolment certificate, and the firmware PCRs.
  A host that fails is told `ABORT(POLICY)`.
- The user logs in with a base grant issued by a role authority the host's
  role table lists, and signs the transcript with the key the grant names.
- An escalation grant adds capabilities for at most one hour, under the
  base grant, with the approvals the role demands.
- The host keeps a signed record of the session and sends it to the client
  before it closes.

Implemented in: `userland/nsh_client/src/lib.rs`, `userland/nsh_roles/src/lib.rs`.

## Handshake order

There is one order. Each side admits only the frames its stage allows
([nsh-v1-wire-session.md](nsh-v1-wire-session.md), "What each side may receive").

| # | Direction | Frame | Protection | Effect |
|---|---|---|---|---|
| 1 | client to server | `CLIENT_HELLO` | plaintext | Carries a fresh X-Wing encapsulation key. |
| 2 | server to client | `SERVER_HELLO`, or `ABORT` (`VERSION`, `BUSY`, `UNATTESTED` or `UNENROLLED`) | plaintext | Carries the X-Wing ciphertext. Both sides derive the handshake keys under `TH(SERVER_HELLO)`. |
| 3 | server to client | `HOST_PROOF` | handshake keys | Host keys, enrolment certificate, attestation document, registry entries, host signature. |
| 4 | client | (check) | | The client checks the host ([nsh-v1-attest.md](nsh-v1-attest.md)). A refusal is answered `ABORT(POLICY)`. |
| 5 | client to server | `USER_AUTH` | handshake keys | User public key, base grant, user signature over the transcript. |
| 6 | server to client | `AUTH_OK` or `AUTH_REFUSED` | handshake keys | `AUTH_OK` ends the handshake. Both sides derive the session keys under `TH(AUTH_OK)`. |
| 7 | both | session frames | session keys | `INPUT`, `OUTPUT`, `WINDOW`, `SIGNAL`, `ESCALATE`, `ESCALATE_RESULT`. |
| 8 | server to client | `EXIT`, `RECORD` parts, `CLOSE` | session keys | The ending order ([nsh-v1-wire-session.md](nsh-v1-wire-session.md)). |

Either side may send `ABORT` where the receiving stage admits it. After a
frame fails to open, the receiver opens no later frame; the client ends
the session without answering it.

Implemented in: `userland/nsh_client/src/client/mod.rs`, `userland/nsh_wire/src/expect.rs`.

## The hybrid suite

Suite 1 is the only suite.

| Purpose | Algorithm | Sizes |
|---|---|---|
| Key exchange | X-Wing KEM (X25519 + ML-KEM-768), `x-wing` crate 0.1.0 | encapsulation key 1216, ciphertext 1120, shared secret 32, client seed 32, encapsulation randomness 64 |
| Signatures | Ed25519 and ML-DSA-65 over the same 64-byte digest | public key 32 + 1952 = 1984; signature 64 + 3309 = 3373 |
| Transcript, digests, key schedule | SHA-512, HMAC-SHA-512, HKDF-SHA-512 | 64 |
| Key id, attestation challenge | SHA-256 | 32 |
| Sealed frames | ChaCha20-Poly1305 | key 32, nonce 12, tag 16 |

- A hybrid public key is the Ed25519 key, then the ML-DSA-65 key.
- A hybrid signature is the Ed25519 signature, then the ML-DSA-65 signature.
- Both halves are always present and both must verify. A signature with
  one good half is refused.
- A key id is SHA-256 of the hybrid public key (Ed25519 key, then ML-DSA-65
  key). Grants, certificates and tables name keys by key id.
- A key digest is SHA-512 of the hybrid public key in the same order. A
  grant's `subject` and a certificate's `host_keys` are key digests.

Implemented in: `userland/nsh_wire/src/keys.rs`, `userland/nsh_crypto/src/kem.rs`, `userland/nsh_crypto/src/challenge.rs`, `userland/nsh_client/src/hybrid.rs`, `userland/nsh_roles/src/hybrid.rs`.

## Versions

| Field | Value |
|---|---|
| Magic | `NSH1` |
| `VERSION` | 2 |
| `SUITE` | 1 |

- Version 2 sets the handshake cap (`AUTH_CAP`) to 1152 KiB, because
  `HOST_PROOF` carries an attestation document with up to 1 MiB of event
  log. A version 1 peer, whose cap is 256 KiB, is told `VERSION` at the
  hello, before any proof is sent.
- A hello's version and suite are judged before its size. A peer that
  speaks another version is told `VERSION`, whatever size its hello is.
- Bodies inside frames carry their own versions: `nsh-enrol/1`,
  `nsh-grant/1`, `nsh-record/1`, `nsh-ckpt/1`, `nsh-policy/1`,
  `nsh-revoke/1`, and the boot applications section of `nsh-enrol/1`. A
  body version the reader does not know is refused as `PROTOCOL`. Only a
  hello's version is answered with `VERSION`.
- Version 1 of the sealed-frame layer does not rekey. A direction that
  reaches its limit ends the session.

Implemented in: `userland/nsh_wire/src/hello.rs`, `userland/nsh_wire/src/error.rs`, `userland/nsh_crypto/src/sealer.rs`.
