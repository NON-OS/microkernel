# nsh v1: roles, grants and escalation

## The host's role table, `nsh-policy/1`

The role table is installed on the host. It is signed with the fleet's role
authority key, and the host refuses a table whose signature is bad. It
names:

- the fleet id;
- the role authority keys;
- the approver keys;
- for each role:
  - its name;
  - its capability ceiling: kernel `Capability` names from
    `src/capabilities/types`;
  - its escalation ceiling;
  - how many approvals an escalation needs (0, 1 or 2);
  - the longest session it allows;
- the signed revocation list, with its issue time.

The three roles shipped are `operator`, `auditor` and `deploy`. Their
capability sets are a separate, reviewed table (plan P6). `Admin`,
`Debug`, `DeviceSecret`, `EnrolDevRoot`, `LocalSign`, `Keyring`,
`CameraCapture`, `MicrophoneCapture` and `ScreenCapture` are never in any
ceiling: the role table parser refuses them.

## A grant, `nsh-grant/1`

| Field | Size |
|---|---|
| magic `"NSHG"` | 4 |
| version = 1 | u16 |
| kind: 1 base, 2 escalation | u8 |
| reserved = 0 | u8 |
| grant_id | 16 |
| parent_id: zero for a base grant | 16 |
| fleet_id | 16 |
| subject: H(user Ed25519 key, user ML-DSA-65 key) | 64 |
| user name | `bytes8`, matching `[a-z][a-z0-9._-]{0,31}` |
| role | `bytes8`, matching `[a-z][a-z0-9-]{0,15}` |
| capabilities | count: u8 (1 to 40), then that many names as `bytes8`, none repeated |
| not_before_ms | u64 |
| not_after_ms | u64 |
| issuer key id | 32 |
| Ed25519 signature | 64 |
| ML-DSA-65 signature | 3309 |
| approvals | count: u8 (0 for a base grant, at most 2 otherwise); each is a key id (32), an Ed25519 signature (64) and an ML-DSA-65 signature (3309) |

- The issuer signs `H("nsh/1 grant\0" || every field before the issuer
  signatures)`.
- An approver signs `H("nsh/1 approve\0" || the same digest)`.
- A key id is SHA-256 of the hybrid public key (Ed25519, then ML-DSA-65).
  The subject is SHA-512 of the same bytes.
- What the bytes alone settle, a reader refuses before any signature is
  checked:
  - `grant_id` and `fleet_id` are not all zeros;
  - `parent_id` is zeros for a base grant, and for an escalation is
    neither zeros nor its own `grant_id`;
  - `not_before < not_after`;
  - an approver is not the issuer, and two approvers differ.
- Capability names are listed in ascending byte order. A list in another
  order is refused, so a list never names one twice and one set has one
  encoding.
- A capability name must be one the host knows. Names, not bit numbers,
  cross the wire, so a renumbered kernel cannot silently change a grant.

## `USER_AUTH` (type 0x20)

The body is:

- the user Ed25519 public key (32 bytes);
- the user ML-DSA-65 public key (1952 bytes);
- the base grant, as `bytes16`. It must be a base grant; an escalation
  grant here is refused by the reader;
- an Ed25519 signature (64 bytes);
- an ML-DSA-65 signature (3309 bytes).

The signatures are over `H("nsh/1 user-sig\0" || TH(USER_AUTH without its
signatures))`, and that transcript includes `HOST_PROOF`. A user's
signature is therefore tied to one attested host and one session.

## The server's checks, in order

A `USER_AUTH` whose bytes do not parse, including the grant inside it, is
a broken frame. It is answered with `ABORT(PROTOCOL)`.

The checks run in two parts. Nothing about the grant, the role or the
revocation list is told to anyone who has not yet shown they hold the
user's key.

**First, proof of the key.** Any failure in this part is answered with the
one code `AUTH_REFUSED(USER_SIG)`:

1. The issuer is a role authority in the role table, and its signatures
   verify.
2. The subject equals H(the keys in `USER_AUTH`).
3. The user's signatures over the transcript verify.

**Then, what the grant allows.** Each failure here has its own code:

4. The fleet id matches the host's fleet id (`GRANT_SCOPE`).
5. `now` is at or after `not_before`, and before `not_after`
   (`GRANT_TIME`).
6. The revocation list was issued at most 24 h ago, unless the table states
   a different maximum age (`REVOCATION_STALE`).
7. The grant is not on that list (`GRANT_REVOKED`).
8. The role exists (`ROLE_UNKNOWN`).
9. Every capability in the grant is a known name (`CAP_UNKNOWN`), within
   the role's ceiling (`CAPS_CEILING`).

Every refusal goes into the host's refusal log (record.md).

**A grant's hash and its revocation.** A grant's hash, `H(grant)`, is
SHA-512 over the bytes the issuer signed, followed by the issuer's
signature. The approvals are left out, because the issuer does not sign
them. The revocation list names grants by `grant_id`, never by hash, so
one grant cannot be given a second hash that escapes the list.

## Escalation

The user sends `ESCALATE` with a grant of kind 2. The server checks it in
the same order as a login, except that the user signature is the one
already given in this session. On top of that:

- `parent_id` must be the session's base grant. `user`, `subject`, `role`
  and `fleet_id` must equal the base grant's (`ESC_PARENT`). The role, and
  so the escalation ceiling and the number of approvals, is the base
  grant's role. It is never one the escalation names for itself.
- its capabilities must be within that role's escalation ceiling;
- `not_after - now` must be at most 3600 s (`ESC_TOO_LONG`);
- it must carry the number of approvals the role demands, each from an
  approver key in the table (`ESC_APPROVAL`);
- each approver must not be the subject or the issuer. Two approvers come
  in ascending key-id order, which the reader enforces, so one set of
  approvals has one encoding;
- only one escalation may run at a time (`ESC_ACTIVE`).

The elevated capsule ends at `min(escalation not_after, base not_after)`.
The kernel holds that time (K3), so an escalation never outlives the
grant it came from, whether or not `nsh.front` is still correct.

A running capsule's capabilities are never widened. The server starts a
second session capsule holding the escalated set, and switches the terminal
to it. When the grant expires, the kernel ends that capsule (kernel change
K3), and the terminal returns to the base capsule. The start, the end and
every command run in between go into the record.
