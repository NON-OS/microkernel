# nsh v1: the attested remote shell

Status: draft 1, for review. Interface version: `nsh/1`. Every byte format
named here is a versioned interface; a change to one is a new version, never
an edit in place.

nsh is a remote shell in which the server proves what it is running before
the person types anything. The client checks the server's measured boot,
its TPM quote and its enrolment, against a policy the client holds. If any
check fails, the connection ends before a user name, a role or a signature
leaves the client. A login is never a root shell. It is a fresh capsule
holding the capabilities a signed role names, and the kernel enforces them.
When the session ends, the server seals a signed record of who logged in,
what they could do, what they ran and what was refused. That record can be
checked offline.

## The parts

| Part | File |
|---|---|
| Threat model | [nsh-v1-threat.md](nsh-v1-threat.md) |
| Wire protocol: framing, the handshake, the keys | [nsh-v1-wire.md](nsh-v1-wire.md) |
| Wire protocol: the frames after login | [nsh-v1-wire-session.md](nsh-v1-wire-session.md) |
| Attestation before any credential | [nsh-v1-attest.md](nsh-v1-attest.md) |
| Roles, grants and escalation | [nsh-v1-roles.md](nsh-v1-roles.md) |
| The session record | [nsh-v1-record.md](nsh-v1-record.md) |
| What is refused, and why | [nsh-v1-refusals.md](nsh-v1-refusals.md) |
| Server layout and what the kernel must add | [nsh-v1-server.md](nsh-v1-server.md) |

## Decisions in one place

- One suite only. Key exchange is X-Wing (X25519 with ML-KEM-768).
  Signatures are Ed25519 together with ML-DSA-65, and both must verify.
  Records are sealed with ChaCha20-Poly1305. Keys come from HKDF-SHA-512.
  The transcript, the digests that are signed, the subject and the
  record's chain all use SHA-512. The exceptions use SHA-256: the
  attestation challenge (the kernel's challenge field is 32 bytes), a key
  id (32 bytes), and the record's output and command digests. There is no
  negotiation, so there is nothing to downgrade.
- The crypto code is reused, never written again:
  - X-Wing comes from the `x-wing =0.1.0` crate already pinned by
    `userland/shield_core`.
  - Ed25519 comes from `userland/nonos_ed25519`.
  - SHA-512 and HMAC come from `userland/nonos_hash`. HKDF is added to
    `nonos_hash` once, for every user.
  - ChaCha20-Poly1305 comes from `userland/nonos_seal`.
  - ML-DSA-65 comes from the PQClean source pinned in `third_party/pqclean`,
    through one shared wrapper crate (see nsh-v1-server.md, open question Q1).
- The server's proof is the kernel's own attestation document,
  `MkAttestDoc` (`NONOSATT`). Its 32-byte challenge is bound to this
  handshake and to the host keys. A host key is accepted only through an
  enrolment certificate, never on first use.
- Credentials are keys: a user's hybrid key pair and a signed role grant.
  There are no passwords.
- No root. Escalation needs a second signed grant. It can require a second
  person's approval, and it expires. It runs in a second capsule; the
  capabilities of a running capsule are never widened.
- There is one terminal channel. There is no forwarding of any kind, no file
  transfer, no compression, no rekeying and no session resumption.
- The record is hash-chained, signed with the hybrid host key, and names the
  attestation document it ran under.

## Who builds on what

| nsh piece | Existing code |
|---|---|
| TCP | `userland/capsule_net_tcp` and `capsule_net_sockets` |
| The attestation document | `src/security/attest_doc` |
| Registry entries | `userland/attest_receipt` |
| Path and STARK checks | `nonos-attest-path`, `nonos-device-attest` |
| Commands | the command layer of `userland/capsule_terminal`, once it is lifted out (Q3) |

Not used: `nonos_tls`. It speaks TLS 1.3 with classical groups only, and its
key schedule and record code are private modules. Its framing is a different
protocol.
