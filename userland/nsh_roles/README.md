# nsh_roles

The nsh/1 role table and the checks a host runs on a grant, as specified in
[docs/design/nsh-v1-roles.md](../../docs/design/nsh-v1-roles.md).

The crate covers:

- the role table, `nsh-policy/1`, and the revocation list, `nsh-revoke/1`:
  each is read exactly, and trusted only once its hybrid signature verifies;
- the kernel's capability names, and the nine no role may ever hold;
- the three roles shipped: `auditor`, `deploy` and `operator`;
- the checks on a login and on an escalation, in the order roles.md gives,
  each refusal its own typed `Refusal` with the one reason code it is told.

It is `no_std` with `alloc`, and builds for the host and the capsule target.
Every time is u64 milliseconds the caller reads from the host clock; nothing
here reads a clock. Grants, capability names and approvals are read by
`nsh_wire`; the labelled digests are `nsh_crypto`'s, and the signatures are
checked with `nonos_ed25519` and `nonos-mldsa65`. Both halves must verify.

## `nsh-policy/1`

All integers are little-endian. `bytes8` is a u8 length, then the bytes. A
hybrid public key is the Ed25519 key (32) then the ML-DSA-65 key (1952).

| Field | Size |
|---|---|
| magic `"NSHP"` | 4 |
| version = 1 | u16 |
| reserved = 0 | u16 |
| fleet_id, never all zeros | 16 |
| serial | u64 |
| issued_ms | u64 |
| revocation max age, ms, 1 to 86 400 000 (24 h) | u64 |
| role authorities: count u8 (1 to 8), then that many hybrid public keys | |
| approvers: count u8 (0 to 16), then that many hybrid public keys | |
| roles: count u8 (1 to 16), then each role as below | |
| signer key id: a role authority, and the one the host pins | 32 |
| Ed25519 signature | 64 |
| ML-DSA-65 signature | 3309 |

Each role:

| Field | Size |
|---|---|
| name | `bytes8`, matching `[a-z][a-z0-9-]{0,15}` |
| flags: bit 0 `record-commands: digest`, the others zero | u8 |
| approvals an escalation needs, 0 to 2 | u8 |
| longest session, ms, 1 to 604 800 000 (7 days) | u64 |
| ceiling: a capability list as a grant carries it, 1 to 40 names | |
| escalation ceiling: the same, 0 to 40 names | |

- A key id is SHA-256 of the hybrid public key. Each key list is strictly
  ascending by key id, and no key is both an authority and an approver.
- Roles are strictly ascending by name.
- A ceiling names only kernel capabilities, never `Admin`, `Debug`,
  `DeviceSecret`, `EnrolDevRoot`, `Keyring`, `LocalSign`, `CameraCapture`,
  `MicrophoneCapture` or `ScreenCapture`.
- The signatures are over `H("nsh/1 policy\0" || every field before them)`.
- The host passes in the highest serial it has installed. A table with a
  lower serial is refused, so an older table, signed before a key was
  dropped, cannot be replayed. The same serial may be reloaded.

## `nsh-revoke/1`

| Field | Size |
|---|---|
| magic `"NSHV"`, version u16 = 1, reserved u16 = 0 | 8 |
| fleet_id, the table's | 16 |
| issued_ms | u64 |
| count u16 (0 to 4096), then that many grant ids | 16 each |
| signer key id: a role authority in the table | 32 |
| Ed25519 signature, ML-DSA-65 signature | 64 + 3309 |

- Grant ids are strictly ascending, and none is all zeros.
- The signatures are over `H("nsh/1 revoke\0" || every field before them)`.
- The host passes in the issue time of the newest list it has accepted; an
  older list is refused, the same one may be reloaded.
- A list issued after `now`, or more than the table's maximum age before
  it, refuses every login and every escalation.

## Escalation

An escalation grant's window, `not_after - not_before`, is at most one
hour, and a longer one is refused outright. Since `not_before <= now`, this
also keeps `not_after - now` within the hour roles.md asks for.

## The shipped roles

- `auditor`: AttestRead, CoreExec, FileSystem, IPC, Memory. It never
  escalates. 8 h sessions.
- `deploy`: CoreExec, FileSystem, IPC, Memory, StoreWrite, StreamImport.
  An escalation, with 1 approval, adds AppInstall, Network and
  ProcessControl. 1 h sessions.
- `operator`: AttestRead, CoreExec, FileSystem, IPC, Memory, ProcessControl.
  An escalation, with 1 approval, adds Network and StoreWrite. 8 h sessions.

No role reaches the network at login: `AppInstall` makes init fetch a
marketplace listing the caller names, so it is an outbound request and comes
only by escalation. `Crypto` (TPM-bound machine keys) and `RegisterService`
(service names) are left out of every ceiling. They can come back once
session capsules have their own key namespace and service names are scoped,
which is a kernel change (K2 and later).

**Until K2 lands, a role's ceiling does not bound what capsules a login can
load.** A session can load an installed capsule whose own manifest grants
more, Network included. K2, which limits a loaded capsule's capabilities to
its caller's, closes this; nothing in this crate can.

The proofs are in [nsh_roles_proofs](../nsh_roles_proofs).
