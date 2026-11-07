# nsh/1 session frames

The frames sent after `AUTH_OK`, under the session keys. Framing, sealing
and the handshake are in [nsh-v1-wire.md](nsh-v1-wire.md).

## Frame bodies

| Type | Frame | Sender | Body |
|---|---|---|---|
| `0x30` | `INPUT` | client | 1 to 16 384 terminal bytes |
| `0x31` | `OUTPUT` | server | 1 to 16 384 terminal bytes |
| `0x32` | `WINDOW` | client | `cols: u16`, `rows: u16`, each 1 to 1000 |
| `0x33` | `SIGNAL` | client | one byte: 1 `Interrupt`, 2 `End` |
| `0x40` | `ESCALATE` | client | `bytes16(1..11 756)`: one `nsh-grant/1` escalation grant |
| `0x41` | `ESCALATE_RESULT` | server | `reason: u16`, `expires_ms: u64` |
| `0x50` | `EXIT` | server | `status: i32` |
| `0x51` | `RECORD` | server | `last: u8` (0 or 1), then 1 to 16 383 bytes of the record |
| `0x52` | `CLOSE` | both | empty |
| `0x7F` | `ABORT` | both | `reason: u16`, 1 to 5 |

- `INPUT` and `OUTPUT` carry at least one byte. The client cuts longer
  input into frames of at most 16 384 bytes.
- A `WINDOW` with either value outside 1 to 1000 is refused.
- A `SIGNAL` with a value other than 1 or 2 is refused, and so is an empty
  one or a longer one.
- `ESCALATE` carries an escalation grant. A base grant is refused
  (`PROTOCOL`). Whether it belongs to this session's base grant is the
  server's check ([nsh-v1-roles.md](nsh-v1-roles.md)).
- `ESCALATE_RESULT` with `reason` = `OK` (0) must carry `expires_ms` other
  than 0: the time the escalation ends. With any other reason it must carry
  `expires_ms` = 0, and the reason must be one that refuses an escalation
  (16 to 24, 32 to 35).
- `EXIT` carries the shell's exit status.
- `RECORD` carries one part of the session record. The part flagged
  `last` = 1 ends it. The parts joined in order are one `nsh-record/1`
  ([nsh-v1-record.md](nsh-v1-record.md)).

- `SIGNAL` reaches the host as it came. 1 (`Interrupt`) is delivered to
  the shell's foreground program. 2 (`End`) asks the shell to end; when it
  exits, the session ends as any exit does (end reason 2).
- A server that has sent `EXIT` takes no more input: anything the client
  sent before it read `EXIT` is dropped.
- After its own `CLOSE` the client sends nothing more. Any frame but
  `ABORT` after it is out of order.
- An `ESCALATE` that arrives while the shell is being stopped is not
  started and not answered; the session's `EXIT` follows.

Implemented in: `userland/nsh_wire/src/session.rs`, `userland/nsh_wire/src/session_codes.rs`, `userland/nsh_wire/src/escalate.rs`, `userland/nsh_wire/src/escalate_result.rs`, `userland/nsh_wire/src/record/part.rs`, `userland/nsh_client/src/client/api.rs`.

## What each side may receive

### By stage

| Stage | Receiver | Admits |
|---|---|---|
| `ClientHello` | server | `CLIENT_HELLO` |
| `ServerHello` | client | `SERVER_HELLO`, plaintext `ABORT` (`VERSION`, `BUSY`, `UNATTESTED`, `UNENROLLED`) |
| `HostProof` | client | `HOST_PROOF`, `ABORT` |
| `UserAuth` | server | `USER_AUTH`, `ABORT` |
| `AuthResult` | client | `AUTH_OK`, `AUTH_REFUSED`, `ABORT` |
| `ServerSession` | server | `INPUT`, `WINDOW`, `SIGNAL`, `ESCALATE`, `CLOSE`, `ABORT` |
| `ClientSession` | client | `OUTPUT`, `ESCALATE_RESULT`, `EXIT`, `RECORD`, `CLOSE`, `ABORT` |

A known type outside its stage's set is refused as out of order and
answered `PROTOCOL`.

Implemented in: `userland/nsh_wire/src/expect.rs`.

### The client in the session

The client keeps three facts while the session runs: how far the ending
has gone (`Running`, `Exited`, `Record`, `RecordDone`), whether an
`ESCALATE` is unanswered, and whether it has sent `CLOSE`.

| Frame received | Admitted when | Effect |
|---|---|---|
| `OUTPUT` | ending is `Running` | Output is shown. |
| `ESCALATE_RESULT` | an `ESCALATE` is unanswered, and ending is `Running` or `Exited` | The escalation is answered. |
| `EXIT` | ending is `Running` | Ending becomes `Exited`. |
| `RECORD` | ending is not `RecordDone` | The part is added. Ending becomes `Record`, or `RecordDone` on the part flagged last. |
| `CLOSE` | ending is `RecordDone` | The session ends. |
| `ABORT` | always | The session ends. |

Any other frame in the session is refused as out of order.

Implemented in: `userland/nsh_client/src/client/stage.rs`, `userland/nsh_client/src/client/session.rs`.

### What the client sends in the session

| Frame | Allowed when |
|---|---|
| `INPUT`, `WINDOW`, `SIGNAL` | ending is `Running` and the client has not sent `CLOSE` |
| `ESCALATE` | ending is `Running`, the client has not sent `CLOSE`, and no `ESCALATE` is unanswered |
| `CLOSE` | once, at any point of the session |
| `ABORT` | on a refusal, while a key exists to seal it |

- Nothing is sent before `AUTH_OK`.
- One escalation at a time: the next `ESCALATE` waits for the answer to
  this one.
- After `CLOSE` the client sends nothing more. The server answers with its
  record and its own `CLOSE`.
- A body the server would refuse (a window out of range, a grant of the
  wrong kind) is not sent, and the session goes on.

Implemented in: `userland/nsh_client/src/client/api.rs`, `userland/nsh_client/src/client/api_close.rs`.

## The ending order

The server ends a session in this order:

1. `EXIT`, when the shell has exited. `OUTPUT` stops at `EXIT`.
2. The record's `RECORD` parts, in order, the last flagged `last` = 1.
3. `CLOSE`.

- `EXIT` comes before the first `RECORD` part, or not at all. A client
  admits `RECORD` while the ending is `Running`; after the first part it
  admits neither `OUTPUT` nor `EXIT`.
- An `ESCALATE_RESULT` may arrive before `EXIT` or between `EXIT` and the
  first `RECORD` part, never inside the record.
- `CLOSE` is admitted only after the last `RECORD` part. A `CLOSE` before
  it is refused as out of order and answered `ABORT(PROTOCOL)`.
- The joined record is held to 64 MiB. A part that would pass it is
  answered `ABORT(LIMIT)`.
- When the last part arrives, the client checks that the record is this
  session's: the header's `session_id` is the one `AUTH_OK` gave, its
  `transcript` is this handshake's `TH(SERVER_HELLO)`, its `subject` is
  the user's key digest, and its `doc_hash` is SHA-512 of the document
  `HOST_PROOF` carried. A mismatch is answered `ABORT(PROTOCOL)`.
- The client hands the record to its caller. It is checked with the
  offline checker ([nsh-v1-record.md](nsh-v1-record.md), "Checking a record offline").

Implemented in: `userland/nsh_client/src/client/session.rs`, `userland/nsh_client/src/client/record.rs`, `userland/nsh_client/src/record/bound.rs`.

## Reason codes

A reason is a `u16`. An unknown code is refused, not shown as a number.

| Code | Name | Group |
|---|---|---|
| 0 | `OK` | success |
| 1 | `PROTOCOL` | session |
| 2 | `VERSION` | session |
| 3 | `POLICY` | session |
| 4 | `LIMIT` | session |
| 5 | `BUSY` | session |
| 6 | `UNATTESTED` | host |
| 7 | `UNENROLLED` | host |
| 16 | `GRANT_SIG` | grant |
| 17 | `GRANT_TIME` | grant |
| 18 | `GRANT_SCOPE` | grant |
| 19 | `GRANT_REVOKED` | grant |
| 20 | `REVOCATION_STALE` | grant |
| 21 | `ROLE_UNKNOWN` | grant |
| 22 | `CAPS_CEILING` | grant |
| 23 | `CAP_UNKNOWN` | grant |
| 24 | `USER_SIG` | grant |
| 32 | `ESC_PARENT` | escalation |
| 33 | `ESC_APPROVAL` | escalation |
| 34 | `ESC_TOO_LONG` | escalation |
| 35 | `ESC_ACTIVE` | escalation |

### Which frame may carry which

| Carrier | Codes allowed |
|---|---|
| plaintext `ABORT` (before keys) | 2, 5, 6, 7 |
| sealed `ABORT` | 1 to 5 |
| `AUTH_REFUSED` | 16 to 24 |
| `ESCALATE_RESULT` | 0 (with `expires_ms` ≠ 0); 16 to 24 and 32 to 35 (with `expires_ms` = 0) |
| record entry `ESC_REFUSED` | 16 to 24, 32 to 35 |

A code outside its carrier's set is refused (`PROTOCOL`). A peer cannot end
a session "because the grant expired", or refuse a login "because of the
version".

| Code | Plain `ABORT` | Sealed `ABORT` | `AUTH_REFUSED` | `ESCALATE_RESULT` | `ESC_REFUSED` |
|---|---|---|---|---|---|
| 0 `OK` | | | | yes | |
| 1 `PROTOCOL` | | yes | | | |
| 2 `VERSION` | yes | yes | | | |
| 3 `POLICY` | | yes | | | |
| 4 `LIMIT` | | yes | | | |
| 5 `BUSY` | yes | yes | | | |
| 6 `UNATTESTED` | yes | | | | |
| 7 `UNENROLLED` | yes | | | | |
| 16 to 24 | | | yes | yes | yes |
| 32 to 35 | | | | yes | yes |

### What a wire refusal is told

| Wire refusal | Told |
|---|---|
| A hello's version or suite | `VERSION` |
| An encoder's buffer too small for the frame | `LIMIT` |
| Anything else: short, trailing, oversize, reserved, magic, body version, unknown type, out of order, value out of range | `PROTOCOL` |

What each server-side and client-side refusal is told is in
[nsh-v1-refusals.md](nsh-v1-refusals.md).

Implemented in: `userland/nsh_wire/src/reason.rs`, `userland/nsh_wire/src/reason_sets.rs`, `userland/nsh_wire/src/error.rs`, `userland/nsh_wire/src/session_codes.rs`, `userland/nsh_wire/src/escalate_result.rs`, `userland/nsh_wire/src/record/body.rs`.
