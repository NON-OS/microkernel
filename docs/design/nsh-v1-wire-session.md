# nsh v1: wire protocol, the frames after login

These frames are sealed with the session keys (see nsh-v1-wire.md). One
session has exactly one terminal.

| Type | Name | Direction | Body |
|---|---|---|---|
| 0x21 | `AUTH_OK` | S to C | session_id[16], idle_s: u32, cap count: u8, then that many cap names as `bytes8` |
| 0x22 | `AUTH_REFUSED` | S to C | reason: u16 |
| 0x30 | `INPUT` | C to S | 1 to 16384 bytes |
| 0x31 | `OUTPUT` | S to C | 1 to 16384 bytes |
| 0x32 | `WINDOW` | C to S | cols: u16, rows: u16 |
| 0x33 | `SIGNAL` | C to S | sig: u8 |
| 0x40 | `ESCALATE` | C to S | grant: `bytes16` (see roles.md) |
| 0x41 | `ESCALATE_RESULT` | S to C | reason: u16, expires_ms: u64 |
| 0x50 | `EXIT` | S to C | status: i32 |
| 0x51 | `RECORD` | S to C | one part of the record: last: u8 (0 or 1), then 1 to 16383 bytes (see record.md) |
| 0x52 | `CLOSE` | either way | empty |
| 0x7F | `ABORT` | either way | reason: u16, never `OK` |

## Field rules

- **`AUTH_OK`.** The session id comes from the server's entropy. `idle_s`
  is between 1 and 86400.
- **Cap names.** A cap name matches `[A-Z][A-Za-z0-9]{0,31}`. A list holds
  at most 40 names and never repeats one. The names listed in `AUTH_OK` are
  the capabilities the session capsule was actually given.
- **`RECORD`.** The record arrives in parts. The client joins the parts in
  order, up to and including the one with `last = 1`.
- **`WINDOW`.** Both `cols` and `rows` must be between 1 and 1000.
- **`SIGNAL`.** `sig` is 1 to interrupt the foreground job or 2 to end it.
  Any other value is refused.
- **`ESCALATE_RESULT`.** A `reason` of 0 means granted, and `expires_ms` is
  then the host's clock value at which the elevated capsule ends. Any other
  `reason` is a refusal code from refusals.md, with `expires_ms` set to 0.

## Which reasons each frame may carry

| Frame | Reasons |
|---|---|
| `ABORT` in plaintext, before the keys | `VERSION`, `BUSY` |
| `ABORT`, sealed | 1 to 5 |
| `AUTH_REFUSED` | 16 to 24 |
| `ESCALATE_RESULT` refusing, and the record's `ESC_REFUSED` | 16 to 24, 32 to 35 |

A code outside its frame's set is refused as `PROTOCOL`. So is a code no
version defines, and `OK` anywhere but a granted `ESCALATE_RESULT`.

## How a session ends

The session ends when the shell exits, when either side sends `CLOSE`, at
the idle timeout, or when the grant's validity runs out. The server's
ending sequence is: `EXIT` (if the shell exited), then `RECORD`, then
`CLOSE`. A client that closes early can fetch its record later with
`nsh record <session_id>`, which is a second, separate login (plan item P9).

## Reason codes (u16)

| Code | Name | Meaning |
|---|---|---|
| 0 | `OK` | granted |
| 1 | `PROTOCOL` | the frame was malformed, out of order or unknown |
| 2 | `VERSION` | the version or suite is not 1/1 |
| 3 | `POLICY` | the client refused the host |
| 4 | `LIMIT` | a size, count, sequence or time limit was reached |
| 5 | `BUSY` | the session cap was reached |
| 16 | `GRANT_SIG` | a grant signature is bad |
| 17 | `GRANT_TIME` | the grant is not yet valid, or has expired |
| 18 | `GRANT_SCOPE` | the grant is for another fleet, or for another user key |
| 19 | `GRANT_REVOKED` | the grant has been revoked |
| 20 | `REVOCATION_STALE` | the host's revocation list is out of date |
| 21 | `ROLE_UNKNOWN` | the role is not one the host knows |
| 22 | `CAPS_CEILING` | the grant names capabilities above the role's ceiling |
| 23 | `CAP_UNKNOWN` | the grant names a capability the host does not know |
| 24 | `USER_SIG` | the user's signature over the transcript is bad |
| 32 | `ESC_PARENT` | the escalation's parent is not this session's grant, or it names another user, role or fleet |
| 33 | `ESC_APPROVAL` | the approvals are missing, or not from a distinct person |
| 34 | `ESC_TOO_LONG` | the escalation asks for more than 3600 s |
| 35 | `ESC_ACTIVE` | an escalation is already running |

The client shows the reason by name. The server never says more than the
code.
