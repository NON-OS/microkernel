# nsh/1 roles and grants

A user logs in with a role grant. The host checks it against a signed role
table and a signed revocation list. Every time is a `u64` count of
milliseconds that the host reads from its own clock and passes in. All
integers are little-endian.

Implemented in: `userland/nsh_roles/src/lib.rs`.

## nsh-grant/1

A grant names a user's role for a while. A base grant opens a session. An
escalation grant names more capabilities, for at most one hour, under a
base grant, and may carry approvals from others.

| Field | Size | Rule |
|---|---|---|
| magic | 4 | `NSHG` |
| version | `u16` | 1 |
| kind | `u8` | 1 base, 2 escalation |
| reserved | `u8` | 0 |
| `grant_id` | 16 | not all zeros |
| `parent_id` | 16 | base: all zeros; escalation: not all zeros and not `grant_id` |
| `fleet_id` | 16 | not all zeros |
| `subject` | 64 | key digest of the user's `HybridPublic` |
| `user` | `bytes8(1..32)` | user name shape |
| `role` | `bytes8(1..16)` | role name shape |
| `caps` | capability list | 1 to 41 names, ascending |
| `not_before_ms` | `u64` | less than `not_after_ms` |
| `not_after_ms` | `u64` | |
| `issuer` | 32 | key id of a role authority |
| `sig` | `HybridSig`, 3373 | |
| approval count | `u8` | 0 to 2; 0 for a base grant |
| approvals | count × 3405 | each `approver: 32` (key id), `sig: HybridSig` |

- **Signed bytes:** every field from `magic` up to and including `issuer`.
- **Issuer signature:** `SHA-512("nsh/1 grant\0" || signed bytes)`, both
  halves, by the role authority `issuer` names.
- **Approval signature:** `SHA-512("nsh/1 approve\0" || SHA-512("nsh/1 grant\0" || signed bytes))`,
  both halves, by the approver.
- **`H(grant)`:** SHA-512 of the signed bytes and `sig`. Approvals are left
  out, since the issuer did not sign them.
- An approval comes from someone other than the issuer. Two approvals name
  two approvers in strictly ascending key id order, so one set of
  approvals has one encoding.
- The largest grant is 11 756 bytes.

Implemented in: `userland/nsh_wire/src/grant/types.rs`, `userland/nsh_wire/src/grant/parse.rs`, `userland/nsh_wire/src/grant/rules.rs`, `userland/nsh_wire/src/grant/approval.rs`, `userland/nsh_roles/src/session.rs`.

## nsh-policy/1

The host's role table. It is trusted only once its signature verifies
under the key id the host pins.

| Field | Size | Rule |
|---|---|---|
| magic | 4 | `NSHP` |
| version | `u16` | 1 |
| reserved | 2 | zero |
| `fleet_id` | 16 | not all zeros |
| `serial` | `u64` | |
| `issued_ms` | `u64` | |
| `revocation_max_age_ms` | `u64` | 1 to 86 400 000 (24 h) |
| role authorities | `u8` count (1 to 8), then that many `HybridPublic` | strictly ascending by key id |
| approvers | `u8` count (0 to 16), then that many `HybridPublic` | strictly ascending by key id |
| roles | `u8` count (1 to 16), then each role | strictly ascending by name |
| `signer` | 32 | key id |
| `sig` | `HybridSig`, 3373 | |

Each role:

| Field | Size | Rule |
|---|---|---|
| `name` | `bytes8(1..16)` | role name shape |
| `flags` | `u8` | bit 0: `record-commands: digest`; other bits zero |
| `approvals` | `u8` | 0 to 2: approvals an escalation needs |
| `max_session_ms` | `u64` | 1 to 604 800 000 (7 days) |
| `ceiling` | capability list | at least one name; known names only; no forbidden name |
| `escalation` | capability list | may be empty; known names only; no forbidden name |

- No key is both a role authority and an approver.
- A role with `record-commands: digest` keeps a SHA-256 of each command in
  the record instead of its text ([nsh-v1-record.md](nsh-v1-record.md)).
- The signed bytes are every field before `sig`. The signature is over
  `SHA-512("nsh/1 policy\0" || signed bytes)`, both halves.

The host accepts a table only when:

1. `signer` is the key id the host pins;
2. `signer` is one of the table's role authorities;
3. the signature verifies under that authority;
4. `serial` is at least the highest serial the host has installed. A table
   with a lower serial is refused, so an older table, signed before a key
   was dropped, cannot be replayed. The same serial may be reloaded.

A table is refused whole. A host never runs on part of a table.

Implemented in: `userland/nsh_roles/src/policy/`, `userland/nsh_roles/src/error.rs`.

## nsh-revoke/1

The signed revocation list. It names revoked grants by `grant_id`, never
by hash, so one grant cannot be given a second hash that escapes the list.

| Field | Size | Rule |
|---|---|---|
| magic | 4 | `NSHV` |
| version | `u16` | 1 |
| reserved | 2 | zero |
| `fleet_id` | 16 | not all zeros; the table's |
| `issued_ms` | `u64` | |
| count | `u16` | 0 to 4096 |
| grant ids | count × 16 | strictly ascending, none all zeros |
| `signer` | 32 | key id of a role authority in the table |
| `sig` | `HybridSig`, 3373 | |

- The signed bytes are every field before `sig`. The signature is over
  `SHA-512("nsh/1 revoke\0" || signed bytes)`, both halves.
- The host accepts a list only when it is for the table's fleet, its
  signer is one of the table's role authorities, the signature verifies,
  and `issued_ms` is at least that of the newest list the host has
  accepted. The same list may be reloaded.
- The list is **fresh** when `issued_ms <= now` and
  `now - issued_ms <= revocation_max_age_ms`. A list issued after `now`
  means the host clock is wrong, and is not fresh. A list that is not
  fresh refuses every login and every escalation.

Implemented in: `userland/nsh_roles/src/revoke/`, `userland/nsh_roles/src/check/host.rs`.

## Capabilities

There are 41 kernel capabilities. Each name stands for the bit at its
index:

| Bit | Name | Bit | Name | Bit | Name |
|---|---|---|---|---|---|
| 0 | `CoreExec` | 14 | `GraphicsPresent` | 28 | `Keyring` |
| 1 | `IO` | 15 | `DeviceEnum` | 29 | `Entropy` |
| 2 | `Network` | 16 | `Driver` | 30 | `AppInstall` |
| 3 | `IPC` | 17 | `Mmio` | 31 | `AttestRead` |
| 4 | `Memory` | 18 | `Irq` | 32 | `ForeignExec` |
| 5 | `Crypto` | 19 | `Dma` | 33 | `LocalSign` |
| 6 | `FileSystem` | 20 | `Pio` | 34 | `StreamImport` |
| 7 | `Hardware` | 21 | `InputSource` | 35 | `DeviceSecret` |
| 8 | `Debug` | 22 | `TimeSet` | 36 | `InputConsumer` |
| 9 | `Admin` | 23 | `SpawnBroker` | 37 | `CameraCapture` |
| 10 | `RegisterService` | 24 | `SpawnWindow` | 38 | `MicrophoneCapture` |
| 11 | `GraphicsDisplayQuery` | 25 | `ProcessControl` | 39 | `ScreenCapture` |
| 12 | `GraphicsSurfaceCreate` | 26 | `StoreWrite` | 40 | `UserFiles` |
| 13 | `GraphicsSurfaceMap` | 27 | `EnrolDevRoot` | | |

A name not on this list is unknown.

### Forbidden capabilities

No role ceiling and no escalation ceiling may name any of these fourteen:

`AcpiOwner`, `Admin`, `CameraCapture`, `Debug`, `DeviceSecret`,
`EnrolDevRoot`, `Keyring`, `LocalSign`, `MicrophoneCapture`, `PolicyPush`,
`PowerControl`, `ScreenCapture`, `SpawnWindow`, `UserFiles`.

`SpawnWindow` makes a process a launcher, which the kernel lets load
capsules beyond its own capabilities; a session holding it could pass its
role's ceiling. `AcpiOwner`, `PolicyPush` and `PowerControl` are Admin's
three slices.

A login must never be able to mint authority, reach the machine's own
secrets, see or hear what is in front of the machine (its camera,
microphone or screen), or reach the person's files on the volume past the
broker. A table whose ceiling names one is refused. With ten forbidden, a
ceiling names at most 31 capabilities.

Implemented in: `userland/nsh_roles/src/caps.rs`, `userland/nsh_roles/src/cap_set.rs`.

## Shipped roles

nsh ships three roles. A fleet's role table writes them as they are here.

| Role | Ceiling | Escalation ceiling | Approvals | Longest session | Command record |
|---|---|---|---|---|---|
| `auditor` | AttestRead, CoreExec, FileSystem, IPC, Memory | (none) | 2 | 8 h | text |
| `deploy` | CoreExec, FileSystem, IPC, Memory, StoreWrite, StreamImport | AppInstall, CoreExec, FileSystem, IPC, Memory, Network, ProcessControl, StoreWrite, StreamImport | 1 | 1 h | text |
| `operator` | AttestRead, CoreExec, FileSystem, IPC, Memory, ProcessControl | AttestRead, CoreExec, FileSystem, IPC, Memory, Network, ProcessControl, StoreWrite | 1 | 8 h | text |

- `auditor` never escalates: its escalation ceiling is empty, and a grant
  names at least one capability.
- No role reaches the network at login. `AppInstall` makes init fetch a
  marketplace listing the caller names, so it is an outbound request and
  comes only by escalation.
- No shipped ceiling names a forbidden capability, nor `TimeSet` (the
  grant checks trust the host clock), `InputConsumer` (every keystroke),
  `SpawnBroker`, `ForeignExec` or any driver capability.
- `Crypto` (TPM-bound machine keys) and `RegisterService` (service names)
  are left out of every shipped ceiling. Admitting them needs session
  capsules with their own key namespace and scoped service names, which
  is a kernel change (K2 and later).
- A role's ceiling bounds the session capsule's capabilities, and through
  it every capsule the session loads: the kernel refuses a load whose
  capabilities pass its parent's and the ambient set, unless the parent is
  a launcher (`src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/bound.rs`),
  and no ceiling may make it one.

Implemented in: `userland/nsh_roles/src/shipped.rs`, `userland/nsh_roles/README.md`.

## Login

The server checks `USER_AUTH` in this order. The first failure refuses the
login with `AUTH_REFUSED(<reason>)`.

| # | Check | Refusal | Told |
|---|---|---|---|
| 1 | The grant is a base grant. | `Kind` | `PROTOCOL` |
| 2 | `issuer` is a role authority in the table. | `Issuer` | `USER_SIG` |
| 3 | Both issuer signatures verify. | `IssuerSig` | `USER_SIG` |
| 4 | `subject` is the key digest of the user key in `USER_AUTH`. | `Subject` | `USER_SIG` |
| 5 | Both user signatures verify over `SHA-512("nsh/1 user-sig\0" \|\| TH(USER_AUTH without its signatures))`. | `UserSig` | `USER_SIG` |
| 6 | `fleet_id` is the table's. | `Fleet` | `GRANT_SCOPE` |
| 7 | `now >= not_before_ms`. | `NotYet` | `GRANT_TIME` |
| 8 | `now < not_after_ms`. | `Expired` | `GRANT_TIME` |
| 9 | The revocation list is fresh. | `RevocationStale` | `REVOCATION_STALE` |
| 10 | The list does not name `grant_id`. | `Revoked` | `GRANT_REVOKED` |
| 11 | The table has the role. | `RoleUnknown` | `ROLE_UNKNOWN` |
| 12 | Every capability name is known. | `CapUnknown` | `CAP_UNKNOWN` |
| 13 | The capabilities are within the role's ceiling. | `CapsCeiling` | `CAPS_CEILING` |

- Checks 2 to 5 prove the key. Every failure among them is told the one
  code `USER_SIG`. Nothing about the grant, the role or the list is judged
  before they pass, so nobody learns of them without holding the user key.
- Check 1 is told `PROTOCOL`, which `AUTH_REFUSED` cannot carry. The wire
  parser refuses an escalation grant in `USER_AUTH` before these checks,
  and the server answers `ABORT(PROTOCOL)`.

An admitted login gives the session capsule the grant's capabilities. The
session ends at `min(not_after_ms, now + max_session_ms)`. The role's
`record-commands: digest` flag and `H(grant)` go to the record.

Implemented in: `userland/nsh_roles/src/check/login.rs`, `userland/nsh_roles/src/check/mod.rs`, `userland/nsh_roles/src/check/host.rs`, `userland/nsh_roles/src/reason.rs`.

## Escalation

The server checks `ESCALATE` in this order. The first failure is answered
`ESCALATE_RESULT(<reason>, 0)`. The user's signature is the one this
session already gave.

| # | Check | Refusal | Told |
|---|---|---|---|
| 1 | No escalation already runs in this session. | `Active` | `ESC_ACTIVE` |
| 2 | The grant is an escalation grant. | `Kind` | `PROTOCOL` |
| 3 | `issuer` is a role authority in the table. | `GrantSig` | `GRANT_SIG` |
| 4 | Both issuer signatures verify. | `GrantSig` | `GRANT_SIG` |
| 5 | `parent_id` is the base grant's `grant_id`, and `user`, `subject`, `role` and `fleet_id` are the base grant's. | `Parent` | `ESC_PARENT` |
| 6 | `not_after_ms - not_before_ms <= 3 600 000`. | `TooLong` | `ESC_TOO_LONG` |
| 7 | `now >= not_before_ms`. | `NotYet` | `GRANT_TIME` |
| 8 | `now < not_after_ms`. | `Expired` | `GRANT_TIME` |
| 9 | `now` is before the base session's end. | `BaseEnded` | `GRANT_TIME` |
| 10 | The revocation list is fresh. | `RevocationStale` | `REVOCATION_STALE` |
| 11 | The list names neither `grant_id` nor `parent_id`. | `Revoked` | `GRANT_REVOKED` |
| 12 | The table has the base grant's role. | `RoleUnknown` | `ROLE_UNKNOWN` |
| 13 | Every capability name is known. | `CapUnknown` | `CAP_UNKNOWN` |
| 14 | The capabilities are within the role's escalation ceiling. | `CapsCeiling` | `CAPS_CEILING` |
| 15 | The approvals hold (below). | `Approval*` | `ESC_APPROVAL` |

- Because the parent must be the base grant, an escalation never
  escalates an escalation.
- The whole window is at most one hour. With `not_before_ms <= now` from
  check 7, `not_after_ms - now` is also at most one hour, and the grant
  cannot be presented again days later.
- Check 2 is told `PROTOCOL`, which `ESCALATE_RESULT` cannot carry. The
  wire parser refuses a base grant in `ESCALATE` before these checks, and
  the server answers `ABORT(PROTOCOL)`.

Approvals are checked in order, each approval in turn:

| Check | Refusal |
|---|---|
| The number of approvals equals the role's `approvals`. | `ApprovalCount` |
| The approver is not the one before it. | `ApprovalDuplicate` |
| The approver's key id is above the one before it. | `ApprovalOrder` |
| The approver is not the issuer. | `ApprovalIssuer` |
| The approver is a key the table lists as an approver. | `ApprovalUnknown` |
| The approver's key digest is not the grant's `subject`. | `ApprovalSubject` |
| Both halves of the approval signature verify. | `ApprovalSig` |

Every approval refusal is told `ESC_APPROVAL`.

An admitted escalation gives a second session capsule the grant's
capabilities. It ends at `min(not_after_ms, base session end)`, the
kernel's hard end. `ESCALATE_RESULT(OK, expires_ms)` names that end.
`H(grant)` goes to the record.

Implemented in: `userland/nsh_roles/src/check/escalate.rs`, `userland/nsh_roles/src/check/approvals.rs`, `userland/nsh_roles/src/reason.rs`, `userland/nsh_roles/src/session.rs`.
