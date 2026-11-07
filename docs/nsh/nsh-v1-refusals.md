# nsh/1 refusals

Every refusal is typed on the side that makes it, so that side can log or
show exactly which check failed. The peer is told at most one reason code
([nsh-v1-wire-session.md](nsh-v1-wire-session.md), "Reason codes"), and
only when a key exists to seal it, except the plaintext `VERSION`, `BUSY`,
`UNATTESTED` and `UNENROLLED` before keys.

| Side | Logged or shown locally | Told to the peer |
|---|---|---|
| Client, host refused | The typed refusal: `HostRefusal`, with the `EnrolRefusal`, `AttestRefusal` or failed signature half inside it; `POLICY (<name>)` from the attestation checker. | `ABORT(POLICY)` only. |
| Client, other refusals | The typed `Refusal`. | One code by the table below, or nothing. |
| Server, login | The typed `nsh_roles::Refusal`. | `AUTH_REFUSED(<code>)`. |
| Server, escalation | The typed `nsh_roles::Refusal`. | `ESCALATE_RESULT(<code>, 0)`. |
| Server, table or list | The typed `PolicyError`. | Nothing. |
| Offline checker | The typed `RecordRefusal` or `CkptRefusal`. | No peer. |

Implemented in: `userland/nsh_client/src/error.rs`, `userland/nsh_roles/src/refusal.rs`, `userland/nsh_attest/src/refusal.rs`.

## Wire refusals (both sides)

A frame that breaks the wire format is refused before any check of who
sent it.

| `WireError` | Cause | Told |
|---|---|---|
| `Short` | The bytes end before a field. | `PROTOCOL` |
| `Trailing` | Bytes left after the last field. | `PROTOCOL` |
| `Oversize` | A length over its cap. | `PROTOCOL` |
| `Reserved` | A reserved field or flag is not zero. | `PROTOCOL` |
| `Magic` | Wrong `NSH1` or body magic. | `PROTOCOL` |
| `Version` | A hello's version or suite. | `VERSION` |
| `BodyVersion` | A body version the reader does not read. | `PROTOCOL` |
| `UnknownType` | A frame type no version defines. | `PROTOCOL` |
| `Order` | A known type that may not arrive now, or not from this side. | `PROTOCOL` |
| `Value` | A field outside its range, including a reason outside its frame's set. | `PROTOCOL` |
| `NoRoom` | An encoder's buffer is too small for the frame. | `LIMIT` |

Implemented in: `userland/nsh_wire/src/error.rs`.

## Client refusals

`nsh_client::Refusal` ends the session, except where the table says the
session goes on.

| Refusal | Cause | Told to the server |
|---|---|---|
| `Wire(WireError)` | A frame broke the wire format or arrived out of order. | The error's code, once keys exist; nothing before. |
| `Open` | A sealed frame did not open. | Nothing. |
| `Limit` | A direction is spent. | Nothing: no frame can be sealed. |
| `Host(HostRefusal)` | The host was refused. | `POLICY` |
| `AuthRefused(Reason)` | The server refused the login for this reason. | Nothing. |
| `Aborted(Reason)` | The server ended the session for this reason. | Nothing. |
| `Record(Mismatch)` | The record received is not this session's. | `PROTOCOL` |
| `RecordLimit` | The record would pass 64 MiB. | `LIMIT` |
| `Sign` | The user's key did not sign. | Nothing. |
| `Unsendable(WireError)` | The caller asked to send a body the server would refuse. Nothing is sent and the session goes on. | Nothing. |
| `GrantSubject` | The grant is not for the user key the client holds. Found before connecting. | Nothing. |
| `NotNow` | The call is not one the session allows now. Nothing is sent. | Nothing. |
| `Ended` | The session is over. | Nothing. |

`Mismatch` names the field: `SessionId`, `Transcript`, `Subject`,
`Document`.

Implemented in: `userland/nsh_client/src/error.rs`, `userland/nsh_client/src/client/end.rs`, `userland/nsh_client/src/record/bound.rs`.

### HostRefusal

Every `HostRefusal` is told `ABORT(POLICY)`. No credential has been sent.

| `HostRefusal` | Cause |
|---|---|
| `HostSig(Half)` | The host keys' signature over the transcript; names the half that failed (`Ed25519` or `MlDsa65`). |
| `Attest(AttestRefusal)` | The attestation checker refused the document. |
| `Challenge` | The document's challenge, read by the client at the offset the checker gave, is not this handshake's. |
| `Enrol(EnrolRefusal)` | The enrolment certificate. |
| `FirmwarePcrs` | PCRs 0, 1, 2 and 7 are not the enrolled values. |

Implemented in: `userland/nsh_client/src/host_error.rs`, `userland/nsh_client/src/host_check.rs`.

### EnrolRefusal

| `EnrolRefusal` | Cause |
|---|---|
| `Authority` | The certificate names an authority the policy does not hold. |
| `Sig(Half)` | The authority's signature; names the half that failed. |
| `Fleet` | Another fleet. |
| `Ak` | Another AK than the quote's. |
| `HostKeys` | Other host keys than `HOST_PROOF`'s. |
| `HostName` | Another host than the one the user typed. Online only. |
| `Expired` | `now_ms >= not_after_ms`. Online only. |
| `Revoked` | The certificate's SHA-512 is in the policy's revoked list. |

Implemented in: `userland/nsh_client/src/host_error.rs`, `userland/nsh_client/src/enrol_check.rs`.

### AttestRefusal

The client's `Attester` interface refuses by step.

| `AttestRefusal` | Step | Cause |
|---|---|---|
| `Parse` | 1 | The document does not parse as `NONOSATT`. |
| `Version` | 1 | The document is version 4 or older. |
| `Challenge` | 2 | The document's challenge is not the one given. |
| `Quote` | 3 | Not a quote, wrong `extraData`, or the AK signature fails. |
| `ReleasePcrs` | 5 | PCRs 8 and 9 match no accepted release. |
| `Log` | 5 | The event log does not parse or replay to the quoted PCRs, PCR 4 holds an application that is neither the release's loader nor one the enrolment lists, or PCR 14 records a capsule the vendor's root did not admit. |
| `Dma` | 6 | The IOMMU is not enforcing, a grant is unconfined, or a device can reach all of memory. |
| `Registry` | 7 | The registry does not fold to its root, is not complete, or its `nsh.front` entry is not the release's. |
| `DeviceProof` | 8 | The policy asks for the device proof and it fails. |
| `Unavailable` | | No attestation checker is available to this build. |

`NonosAttester` maps the checker's names onto these:

| Checker names | `AttestRefusal` |
|---|---|
| `format` | `Parse` |
| `version` | `Version` |
| `challenge` | `Challenge` |
| `quote`, `binding`, `signer`, `signature`, `selection`, `pcr-digest` | `Quote` |
| `pcr9`, `pcr8` | `ReleasePcrs` |
| `log`, `replay`, `loader`, `boot-application`, `pcr14`, `path-only`, `developer`, `local-build` | `Log` |
| `dma` | `Dma` |
| `registry`, `registry-complete`, `capsule-count`, `not-vendor`, `network-capsule`, `front` | `Registry` |

The step 4 names and `pcr0` to `pcr7` do not arise through
`NonosAttester`: the client makes those checks itself and reports them as
`EnrolRefusal` and `FirmwarePcrs`.

Implemented in: `userland/nsh_client/src/attest.rs`, `userland/nsh_client/src/nonos.rs`, `userland/nsh_client/src/bin/nsh_verify_record.rs`.

### Attestation checker names

The `NONOSATT` checker refuses with one name per check. The user is shown
`POLICY (<name>)`. The host is told `ABORT(POLICY)`.

| Step | Name | Cause |
|---|---|---|
| 1 | `format` | The document does not parse: short, trailing, oversize, wrong magic, or a value out of range. |
| 1 | `version` | A version other than 5. |
| 2 | `challenge` | Another challenge. |
| 3 | `quote` | `TPMS_ATTEST` does not parse, does not start with `TPM_GENERATED_VALUE`, or is not `TPM_ST_ATTEST_QUOTE`. |
| 3 | `binding` | `extraData` is not the client's own `qualifying_data`. |
| 3 | `signer` | `qualifiedSigner` is not a SHA-256 name. |
| 3 | `signature` | The AK signature does not verify, or is not ECDSA over SHA-256. |
| 3 | `selection` | The quote does not select exactly PCRs 0, 1, 2, 4, 7, 8, 9, 14 in one SHA-256 bank. |
| 3 | `pcr-digest` | The document's PCR values are not the ones the quote signed. |
| 4 | `enrol` | The certificate does not parse. |
| 4 | `enrol-authority` | It names an authority the policy does not hold. |
| 4 | `enrol-signature` | Either half of the authority's signature fails. |
| 4 | `enrol-fleet` | Another fleet. |
| 4 | `enrol-host` | Another host name. |
| 4 | `enrol-ak` | Another AK. |
| 4 | `enrol-host-keys` | Other host keys. |
| 4 | `enrol-expired` | `now_ms >= not_after_ms`. |
| 4 | `enrol-revoked` | The policy revoked it. |
| 5 | `pcr9` | PCR 9 is zero or names no accepted release. |
| 5 | `pcr8` | PCR 8 is zero or not that release's. |
| 5 | `log` | The event log does not parse, or holds an event PCR 4 or PCR 14 may not hold. |
| 5 | `replay` | Replaying the log does not give a quoted PCR among 0, 1, 2, 4, 7, 14. |
| 5 | `boot-application` | PCR 4 holds an application that is neither the loader nor an enrolled boot application. |
| 5 | `loader` | The release's loader is not among PCR 4's applications. |
| 5 | `pcr14` | PCR 14 records no capsule. |
| 5 | `path-only` | A capsule of kind 2. |
| 5 | `developer` | A capsule of kind 3. |
| 5 | `local-build` | A capsule of kind 4. |
| 5 | `pcr0`, `pcr1`, `pcr2`, `pcr7` | That firmware PCR is not the enrolled value. |
| 6 | `dma` | DMA is not confined. |
| 7 | `registry` | The entries do not fold to the quoted root. |
| 7 | `registry-complete` | `registry_complete` is not 1. |
| 7 | `capsule-count` | `capsule_count` is not the number of entries. |
| 7 | `not-vendor` | An entry the vendor's root did not admit. |
| 7 | `network-capsule` | An entry holds Network that is neither the release's `nsh.front` nor in its network set. |
| 7 | `front` | Not exactly one Network entry with the release's `nsh.front` measurement. |

A client passing the check logs `attest: ok release=<version> pcr9=<hex>`.

Implemented in: `userland/nsh_attest/src/refusal_kind.rs`, `userland/nsh_attest/src/refusal_name.rs`, `userland/nsh_attest/src/refusal_log.rs`, `userland/nsh_attest/src/check.rs`.

## Server refusals

### Login

A login refusal is answered `AUTH_REFUSED(<code>)`. The host's refusal log
names the check.

| `Refusal` | Cause | Code |
|---|---|---|
| `Kind` | The grant is an escalation grant. | `PROTOCOL` |
| `Issuer` | The issuer is not a role authority. | `USER_SIG` (24) |
| `IssuerSig` | The issuer's signatures do not both verify. | `USER_SIG` (24) |
| `Subject` | The subject is not the digest of the key in `USER_AUTH`. | `USER_SIG` (24) |
| `UserSig` | The user's signatures over the transcript do not both verify. | `USER_SIG` (24) |
| `Fleet` | The grant is for another fleet. | `GRANT_SCOPE` (18) |
| `NotYet` | `now < not_before_ms`. | `GRANT_TIME` (17) |
| `Expired` | `now >= not_after_ms`. | `GRANT_TIME` (17) |
| `RevocationStale` | The revocation list is not fresh. | `REVOCATION_STALE` (20) |
| `Revoked` | The list names the grant. | `GRANT_REVOKED` (19) |
| `RoleUnknown` | The table has no such role. | `ROLE_UNKNOWN` (21) |
| `CapUnknown` | A capability name the kernel does not have. | `CAP_UNKNOWN` (23) |
| `CapsCeiling` | A capability outside the role's ceiling. | `CAPS_CEILING` (22) |

- Every failure in the proof of the key is told `USER_SIG`.
- `PROTOCOL` is not a login code. A grant of the wrong kind is refused by
  the wire parser first, and answered `ABORT(PROTOCOL)`.

Implemented in: `userland/nsh_roles/src/refusal.rs`, `userland/nsh_roles/src/reason.rs`, `userland/nsh_roles/src/check/login.rs`.

### Escalation

An escalation refusal is answered `ESCALATE_RESULT(<code>, 0)`.

| `Refusal` | Cause | Code |
|---|---|---|
| `Active` | An escalation already runs in this session. | `ESC_ACTIVE` (35) |
| `Kind` | The grant is a base grant. | `PROTOCOL` |
| `GrantSig` | The issuer is unknown, or its signatures do not both verify. | `GRANT_SIG` (16) |
| `Parent` | The parent is not the base grant, or the user, subject, role or fleet differ from it. | `ESC_PARENT` (32) |
| `TooLong` | The window is over one hour. | `ESC_TOO_LONG` (34) |
| `NotYet` | `now < not_before_ms`. | `GRANT_TIME` (17) |
| `Expired` | `now >= not_after_ms`. | `GRANT_TIME` (17) |
| `BaseEnded` | The base session has ended. | `GRANT_TIME` (17) |
| `RevocationStale` | The revocation list is not fresh. | `REVOCATION_STALE` (20) |
| `Revoked` | The list names the grant or its parent. | `GRANT_REVOKED` (19) |
| `RoleUnknown` | The table has no such role. | `ROLE_UNKNOWN` (21) |
| `CapUnknown` | A capability name the kernel does not have. | `CAP_UNKNOWN` (23) |
| `CapsCeiling` | A capability outside the role's escalation ceiling. | `CAPS_CEILING` (22) |
| `ApprovalCount` | Not the number of approvals the role demands. | `ESC_APPROVAL` (33) |
| `ApprovalDuplicate` | An approver twice. | `ESC_APPROVAL` (33) |
| `ApprovalOrder` | Approvers not in ascending order. | `ESC_APPROVAL` (33) |
| `ApprovalIssuer` | The issuer approved its own grant. | `ESC_APPROVAL` (33) |
| `ApprovalUnknown` | The approver is not an approver in the table. | `ESC_APPROVAL` (33) |
| `ApprovalSubject` | The user approved their own grant. | `ESC_APPROVAL` (33) |
| `ApprovalSig` | An approval's signatures do not both verify. | `ESC_APPROVAL` (33) |

- An escalation's issuer failure is told `GRANT_SIG`, since the user is
  already known.
- `PROTOCOL` is not an escalation code. A grant of the wrong kind is
  refused by the wire parser first, and answered `ABORT(PROTOCOL)`.
- The record's `ESC_REFUSED` entry carries codes from the same set as
  `ESCALATE_RESULT`'s refusals.

Implemented in: `userland/nsh_roles/src/refusal.rs`, `userland/nsh_roles/src/reason.rs`, `userland/nsh_roles/src/check/escalate.rs`, `userland/nsh_roles/src/check/approvals.rs`.

### Role table and revocation list

These are refused whole, on the host, and never told to a peer. A host
whose revocation list is not fresh refuses every login and escalation
with `REVOCATION_STALE`.

| `PolicyError` | Cause |
|---|---|
| `Wire(WireError)` | A length, magic, version, reserved byte or name shape is wrong. |
| `Range` | A count, an age, a session length, an approval count, or a revoked id of all zeros. |
| `Order` | Keys, roles or revoked ids not strictly ascending. |
| `Overlap` | A key is both a role authority and an approver. |
| `CapUnknown` | A ceiling names a capability the kernel does not have. |
| `CapForbidden` | A ceiling names a forbidden capability. |
| `EmptyCeiling` | A role's ceiling grants nothing. |
| `Signer` | The signer is not the pinned key, or not a role authority of the table. |
| `Signature` | A signature does not verify. |
| `Fleet` | The revocation list is for another fleet. |
| `Replayed` | A lower table serial, or a list issued before the newest accepted. |

Implemented in: `userland/nsh_roles/src/error.rs`, `userland/nsh_roles/src/check/host.rs`.

### The connection

What `nsh_server` logs (`Why`) and what it tells the client. A refusal
before keys exist is told only as `VERSION`; everything else then is closed
without a frame.

| `Why` | Cause | Told |
|---|---|---|
| `Wire(e)` | A frame broke the wire format or came out of order. | `ABORT(VERSION)` in plaintext for a hello of another version or suite; otherwise `ABORT(e's code)` once keys exist, and nothing before. |
| `Open` | A sealed frame did not open. | nothing |
| `Limit` | A direction is spent. | nothing |
| `HandshakeTime` | The handshake ran past the host's deadline. | `ABORT(LIMIT)` once keys exist, and nothing before. |
| `Attest(fault)` | The kernel could not attest, or answered with a document or entries no client could accept. | `ABORT(BUSY)` |
| `Sign` | The host keys did not sign. | nothing |
| `Rand` | The entropy source failed. | nothing |
| `Policy(e)` | The revocation list is not one the table trusts. | `AUTH_REFUSED(REVOCATION_STALE)`, or `ESCALATE_RESULT(REVOCATION_STALE)` |
| `Login(r)` | The role table refused the login. | `AUTH_REFUSED(r's code)` |
| `Escalation(r)` | The role table refused an escalation. The session goes on. | `ESCALATE_RESULT(r's code)` |
| `Denied` | The host could not start an admitted session or escalation. | `ABORT(BUSY)` |
| `Aborted(code)` | The client sent `ABORT`. | nothing |

Before the server exists for a connection, the host itself may turn it away
(nsh.front, `capsule_nsh_front/src/unready.rs`), in plaintext, and says the
whole reason on its serial console:

| Host refusal | Cause | Told |
|---|---|---|
| `NoTpm` | The machine has no TPM: no host keys, no attestation. | `ABORT(UNATTESTED)` |
| `NoAnswer` | nsh.attest gave no host keys or records now. | `ABORT(BUSY)` |
| `Missing` | The machine holds no enrolment, role table, anchor or revocation list under /nonos/nsh. | `ABORT(UNENROLLED)` |
| `Enrol` | /nonos/nsh/enrol is not an `nsh-enrol/1` certificate. | `ABORT(UNENROLLED)` |
| `Table(e)` | The role table or revocation list does not open. | `ABORT(UNENROLLED)` |
| `Older` | The role table or revocation list is older than one this boot already served under. | `ABORT(UNENROLLED)` |

A session that ran is ended with its record signed and kept, whatever
ended it; the footer's end reason is in [nsh-v1-record.md](nsh-v1-record.md).

Implemented in: `userland/nsh_server/src/why.rs`, `userland/nsh_server/src/server/end.rs`.

## Offline record refusals

The record checker has no peer. It reports the refusal and the step.

| `RecordRefusal` | Step | Cause |
|---|---|---|
| `Parse(WireError)` | 1 | The record does not parse, and there is no checkpoint to check a footerless record against. |
| `Unbound(Mismatch)` | 1 | The header names another session, handshake or user than expected. |
| `DocHash` | 2 | The document given is not the one the header names. |
| `Host(HostRefusal)` | 2 to 4 | The document, the challenge, the enrolment or the firmware PCRs. |
| `Chain` | 5 | The chain does not recompute to the footer's value. |
| `Sig(Half)` | 6 | The host keys' signature over the footer. |
| `Ckpt(CkptRefusal)` | 5 | A checkpoint. |

| `CkptRefusal` | Cause |
|---|---|
| `Parse(WireError)` | The checkpoint does not parse. |
| `Session` | It names another session than the record's. |
| `Sig(Half)` | The host keys' signature over it. |
| `Beyond` | It covers an entry the record does not hold. |
| `Chain` | The chain after its entry is not the value it signs. |

Implemented in: `userland/nsh_client/src/record/verdict.rs`, `userland/nsh_client/src/record/ckpt.rs`, `userland/nsh_client/src/record/walk.rs`.

## Enrolment tool refusals

`nsh-enrol` refuses to make or sign a certificate the client would then
refuse. Each refusal is shown to the operator and nothing is written.

| `Error` | Cause |
|---|---|
| `Doc(<name>)` | The document failed one of the client's own checks; shown by the checker's name. |
| `Challenge` | The document answers another challenge. |
| `Ak` | The document's AK is not the one the registrar activated. |
| `Replay(i)` | The log does not replay to PCR *i*. |
| `NoLoader` | No release loader is in PCR 4. |
| `BootApplications(n)` | More than 8 boot applications. |
| `Unexpected(digest)` | A PCR 4 application the operator did not expect. |
| `Value(what)` | An invalid host name, fleet id, validity, body or other value. |
| `KeyFile` | A key file this tool did not write. |
| `Randomness` | No randomness, or signing failed. |
| `SelfCheck` | The certificate made does not verify. |

Implemented in: `tools/nsh-enrol/src/error.rs`.
