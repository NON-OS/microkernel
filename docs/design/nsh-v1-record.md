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
| transcript: `TH(SERVER_HELLO)` | 64 |
| host Ed25519 and ML-DSA-65 public keys | 32 + 1952 |
| the enrolment certificate from `HOST_PROOF` | `bytes16` |
| H(attestation document sent in `HOST_PROOF`) | 64 |
| subject: H(user keys) | 64 |
| user name, role | two `bytes8` |
| H(base grant), as nsh-v1-roles.md defines it | 64 |
| granted capabilities | count: u8 (1 to 40), then the names as `bytes8`, ascending |

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
| 9 | `BOX` | one Linux program's run in a box: the layout below |

`REFUSED_CALL` is a refusal by the kernel (K4). `BOX` is the refusals of a
Linux program that `capsule_linux` ran in a box, as `capsule_linux` itself
enforced them. These are not the kernel's sealed errno histogram, and the
record never sums the two.

## The `BOX` entry

It carries BOX REPORT v1, from `userland/capsule_linux/src/linux/boxed/`,
field for field, without the examples. The kind order, the preset names,
the report version and the saturating counts are checked against those
sources by `nsh_wire_proofs` (`record_box_source`), so a change on either
side fails the build's tests.

No host writes a `BOX` entry yet. The session host may not start
`capsule_linux` or read its private frames until a kernel change grants
it that right explicitly, audited and shown in `audit` (a K item for
0.9.4).

| Field | Size |
|---|---|
| preset: 0 `strict`, 1 `offline`, 2 `allow` | u8 |
| policy: the BLAKE3 hash of the box's policy | 32 |
| started: 0 no, 1 yes | u8 |
| exit status | i32 |
| refusal counts, in this order | 9 x u64 |

The nine counts are in BOX REPORT v1's fixed order, and the position is
the kind:

0. `write`
1. `network`
2. `exec`
3. `jit`
4. `display`
5. `models`
6. `call`
7. `barred`
8. `unserved`

Each count saturates, as `capsule_linux` counts it. The total is not
stored, because it is their sum. The body is 110 bytes exactly.

The verifier refuses an entry with:

- a preset above 2;
- a `started` above 1;
- a body of any other length.

The examples are left out on purpose: they are paths and addresses the
program asked for, and the screen report keeps them. The entry is
written from the run's `[BOX]` line, read out of the frames `capsule_linux`
keys to that run, never from text the program printed:

- the session host starts `capsule_linux` with `--box-key=<32 hex>`, a
  fresh key, as its first argument;
- it reads the run's private output with `mk_proc_output(pid)`, and keeps
  only frames that open with tag 3 and that key;
- it parses the `[BOX]` line from those frames, refuses any `v` other than
  1, and checks that `refused=` is the saturating sum of the nine counts;
- a run started without a key has no `BOX` entry.

Until that grant exists, the session host has no such right; the lead
ruled that reusing the Terminal's path would be a new, unaudited authority.

A run that never started (`started` 0) carries counts of 0 in practice,
but nothing in `capsule_linux` enforces it, so the verifier accepts any
counts and any exit status with it. A box report version other than 1
needs a new entry kind.

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

The host keys sign `H("nsh/1 record-sig\0" || SHA-512(header || entries ||
the first four footer fields))`.

## Checking a record offline

`nsh verify-record <file> --doc <attestation document>` refuses the record
unless all of these hold:

1. The record parses: the sequence is dense, times never go back, and the
   footer's count and end time agree with the entries.
2. SHA-512 of the given document equals the header's document hash. The
   document passes the client's checks (nsh-v1-attest.md, steps 1 to 7)
   against the verifier's policy.
3. `SHA-256("nsh/1 attest\0" || transcript || SHA-512(host keys))` is the
   document's challenge. The header holds both the transcript and the host
   keys, so this ties those host keys to that attested boot and that
   handshake.
4. The enrolment certificate in the header verifies under the fleet
   authority, and names the document's AK and SHA-512 of the host keys.
5. The chain recomputes to the footer's chain value.
6. The host keys' hybrid signature verifies over the footer.

A key pair that never took part in the attested handshake cannot sign a
record that passes step 3.

## Checkpoints, `nsh-ckpt/1`

Every 64 entries, and at each escalation's start and end, the host writes a
checkpoint beside the record it is building:

| Field | Size |
|---|---|
| magic `"NSHC"`, version u16 = 1, reserved u16 = 0 | 8 |
| session_id | 16 |
| seq, the last entry it covers | u32 |
| chain value after that entry | 64 |
| Ed25519 signature, ML-DSA-65 signature | 3373 |

The host keys sign `H("nsh/1 record-ckpt\0" || SHA-512(the fields before
the signatures))`. A record without a footer is checked as far as its last
checkpoint, which needs steps 1 to 5 above and the checkpoint's own
signature. It is reported as `cut short at seq N`, never as complete.

## What the record does and does not show

- A refusal by the kernel reaches the record only if the kernel tells the
  session's parent about it (kernel change K4).
- A record cut off by a crash has no footer, and is checked as far as its
  last checkpoint (above).
- Command lines may contain secrets typed by the user. The record keeps
  what was typed. The role table can mark a role `record-commands: digest`,
  which keeps SHA-256 of each command line (`COMMAND_DIGEST`) instead of its
  text.
