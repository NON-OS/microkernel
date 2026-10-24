# nsh v1: the session record, `nsh-record/1`

The tree has no signed session or box report to copy; this format is new.
The record is one byte string made of a header, a list of entries and a
footer. A host tool and the client library check it offline.

## Header

| Field | Size |
|---|---|
| magic `"NSHR"` | 4 |
| version = 1 | u16 |
| reserved = 0 | u16 |
| session_id | 16 |
| start_ms, from the host's clock | u64 |
| H(attestation document sent in `HOST_PROOF`) | 64 |
| H(host keys) | 64 |
| subject: H(user keys) | 64 |
| user name, role | two `bytes8` |
| H(base grant) | 64 |
| granted capabilities | count: u8, then that many names as `bytes8` |

## Entries

Each entry is laid out as `seq: u32 | at_ms: u64 | kind: u8 | body: bytes16`.
`seq` starts at 0 and counts up by one. `at_ms` is the host clock's value;
it is never before `start_ms` and never before the entry ahead of it. A
body is at most 4096 bytes. A record is at most 64 MiB and holds at most
2^20 entries. The footer's count must equal the number of entries, and its
`end_ms` must not be before the last entry.

| Kind | Name | Body |
|---|---|---|
| 1 | `COMMAND` | the command line as typed, 1 to 4096 bytes |
| 2 | `OUTPUT` | bytes: u64, then SHA-256 of the command's output (output itself is never kept) |
| 3 | `REFUSED_CALL` | syscall tag: u32, capability name: `bytes8` |
| 4 | `ESC_GRANTED` | H(grant): 64, expires_ms: u64, capability names |
| 5 | `ESC_REFUSED` | reason: u16, H(grant): 64 |
| 6 | `ESC_ENDED` | H(grant): 64, why: u8 (1 expired, 2 user, 3 session end) |
| 7 | `EXIT` | status: i32 |
| 8 | `COMMAND_DIGEST` | SHA-256 of the command line, for a role set to `record-commands: digest` |

## The chain

`c(0) = H("nsh/1 record\0" || header)`. For each entry,
`c(n+1) = H(c(n) || entry bytes)`.

## Footer

| Field | Size |
|---|---|
| entry count | u32 |
| end_ms | u64 |
| end reason: 1 exit, 2 close, 3 idle, 4 grant end, 5 abort, 6 host shutdown | u8 |
| final chain value | 64 |
| Ed25519 signature | 64 |
| ML-DSA-65 signature | 3309 |

The host keys sign `H("nsh/1 record-sig\0" || header || entries || the
first four footer fields)`.

## Checking a record offline

`nsh verify-record <file> --doc <attestation document>` refuses the record
unless all of these hold:

- the attestation document's hash equals the one in the header;
- the host keys named in that document's session verify the signatures;
- the chain recomputes;
- `seq` is dense;
- the count matches.

## What the record does and does not show

- A refusal by the kernel reaches the record only if the kernel tells the
  session's parent about it (kernel change K4).
- A record cut off by a crash has no footer. The host writes a signed
  checkpoint of the chain value every 64 entries and at every escalation.
  A record that ends at a checkpoint is shown as `cut short at seq N`, never
  as complete.
- Command lines may contain secrets typed by the user. The record keeps
  what was typed. The role table can mark a role `record-commands: digest`,
  which keeps SHA-256 of each command line (`COMMAND_DIGEST`) instead of its
  text.
