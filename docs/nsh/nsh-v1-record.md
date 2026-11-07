# nsh/1 session record

The host keeps a record of every session: who logged in, under which
attestation and grant, with what capabilities, then what happened. The
host keys sign it. The server sends it in `RECORD` parts at the end of the
session ([nsh-v1-wire-session.md](nsh-v1-wire-session.md)), and anyone
holding it, the attestation document and the fleet's keys can check it
offline. All integers are little-endian.

Implemented in: `userland/nsh_wire/src/record/mod.rs`, `userland/nsh_client/src/record/mod.rs`.

## nsh-record/1

A record is a header, zero or more entries, and a footer.

| Limit | Value |
|---|---|
| Record size | at most 64 MiB (67 108 864 bytes) |
| Entries | at most 2^20 (1 048 576) |

### Header

| Field | Size | Rule |
|---|---|---|
| magic | 4 | `NSHR` |
| version | `u16` | 1 |
| reserved | 2 | zero |
| `session_id` | 16 | the id `AUTH_OK` gave |
| `start_ms` | `u64` | |
| `transcript` | 64 | `TH(SERVER_HELLO)` of the session's handshake |
| `host` | `HybridPublic`, 1984 | the host keys |
| `enrol` | `bytes16(1..4205)` | the `nsh-enrol/1` certificate `HOST_PROOF` carried |
| `doc_hash` | 64 | SHA-512 of the attestation document `HOST_PROOF` carried |
| `subject` | 64 | key digest of the user key |
| `user` | `bytes8(1..32)` | user name shape |
| `role` | `bytes8(1..16)` | role name shape |
| `grant_hash` | 64 | `H(grant)` of the base grant ([nsh-v1-roles.md](nsh-v1-roles.md)) |
| `caps` | capability list | 1 to 41 names: what the session was given |

From `transcript` and `host`, a checker recomputes the challenge the
attestation document must carry.

Implemented in: `userland/nsh_wire/src/record/header.rs`, `userland/nsh_wire/src/record/header_read.rs`.

### Entries

| Field | Size | Rule |
|---|---|---|
| `seq` | `u32` | 0 for the first entry, then counting up by one, without a gap |
| `at_ms` | `u64` | not before the previous entry's, nor before `start_ms` |
| `kind` | `u8` | 1 to 9 |
| `body` | `bytes16(1..4096)` | read exactly, by kind |

| Kind | Name | Body |
|---|---|---|
| 1 | `COMMAND` | the command line: 1 to 4096 bytes |
| 2 | `OUTPUT` | `bytes: u64`, `digest: 32` |
| 3 | `REFUSED_CALL` | `tag: u32`, `cap: bytes8(1..32)` (capability name shape) |
| 4 | `ESC_GRANTED` | `grant: 64`, `expires_ms: u64`, `caps`: capability list of 1 to 41 names |
| 5 | `ESC_REFUSED` | `reason: u16` (16 to 24, 32 to 35), `grant: 64` |
| 6 | `ESC_ENDED` | `grant: 64`, `why: u8` (1 to 3) |
| 7 | `EXIT` | `status: i32` |
| 8 | `COMMAND_DIGEST` | 32: SHA-256 of the command line |
| 9 | `BOX` | 110 bytes (below) |

- A kind outside 1 to 9 is refused. So is a reason, a `why` or a flag out
  of its range.
- A role with `record-commands: digest` keeps `COMMAND_DIGEST` entries in
  place of command text.
- In `ESC_GRANTED`, `ESC_REFUSED` and `ESC_ENDED`, `grant` is the
  escalation grant's hash, `H(grant)`.

- `OUTPUT` covers one run of terminal output: every byte the host sent in
  `OUTPUT` frames since the entry before it. `bytes` counts them and
  `digest` is their SHA-256. The host writes the run before any entry of
  another kind and before the session ends, so every byte sent is counted
  once.

- `REFUSED_CALL` names a call the kernel refused the session capsule: `tag`
  is the kernel's identifier for the syscall, as its refused-call report
  gives it, and `cap` the capability the capsule lacked.
- `COMMAND` holds a command line as the shell ran it, 1 to 4096 bytes. A
  role whose flags keep digests, and any line over 4096 bytes, is written
  as `COMMAND_DIGEST`, the line's SHA-256, instead.

- `ESC_ENDED`'s `why`: 1, the escalation grant's window closed; 2, the
  session it belonged to ended first; 3, the escalation grant or its base
  grant was revoked. When a session ends with an escalation running, its
  `ESC_ENDED` (why 2) comes before `EXIT`.

Implemented in: `userland/nsh_wire/src/record/entry.rs`, `userland/nsh_wire/src/record/body.rs`.

### BOX entries

A `BOX` entry records one Linux program's run in a box, as `capsule_linux`
enforced it. It carries BOX REPORT v1 field for field, without the
examples. Its body is exactly 110 bytes:

| Field | Size | Rule |
|---|---|---|
| `preset` | `u8` | 0 strict, 1 offline, 2 allow |
| `policy` | 32 | BLAKE3 hash of the box's policy |
| `started` | `u8` | 0 or 1 |
| `exit` | `i32` | |
| `counts` | 9 × `u64` | refusals by kind, in this order: `write`, `network`, `exec`, `jit`, `display`, `models`, `call`, `barred`, `unserved` |

The run's total refusals are the sum of the counts, held at `u64::MAX`.

Kind 9 is provisional. While it is, a host does not sign a record that
holds a `BOX` entry. A checker reads it and reports that the record holds one: a
complete record holding a `BOX` entry is not reported as plainly complete.

Implemented in: `userland/nsh_wire/src/record/boxed.rs`, `userland/nsh_client/src/record/verdict.rs`, `userland/nsh_client/src/cli/out.rs`.

### Footer

The footer is the last 3450 bytes. Entries are read while more than that
remains.

| Field | Size | Rule |
|---|---|---|
| `count` | `u32` | the number of entries |
| `end_ms` | `u64` | not before the last entry's `at_ms`, nor before `start_ms` |
| `end_reason` | `u8` | 1 to 6 |
| `chain` | 64 | the chain after the last entry |
| `sig` | `HybridSig`, 3373 | |

The host keys sign:

```
SHA-512("nsh/1 record-sig\0" || SHA-512(every record byte before sig))
```

Both halves must verify under the header's `host`.

`end_reason`:

| Code | The session ended because |
|---|---|
| 1 | the client sent `CLOSE` |
| 2 | the shell exited on its own |
| 3 | no input arrived within `AUTH_OK`'s `idle_s` |
| 4 | the grant ran out, or the role's longest session was reached |
| 5 | it was aborted: a broken frame, a spent direction, or the client's `ABORT` |
| 6 | the host ended it: the grant was revoked, the record reached the host's cap, or the host could not keep the session |

A host holds each record to a cap of its choosing, from 64 KiB to 64 MiB.
It keeps room back for the entries a session ends with (`OUTPUT`,
`ESC_ENDED`, `EXIT`) and the footer, so a record at its cap still ends and
is signed (end reason 6).

Implemented in: `userland/nsh_server/src/end_reason.rs`, `userland/nsh_server/src/record/`.

Implemented in: `userland/nsh_wire/src/record/footer.rs`, `userland/nsh_wire/src/record/parse.rs`, `userland/nsh_client/src/record/verify.rs`.

## The chain

```
c(0)   = SHA-512("nsh/1 record\0" || header bytes)
c(n+1) = SHA-512(c(n) || entry n bytes)
```

- The header bytes are the whole header, from `magic` to the end of `caps`.
- An entry's bytes are the whole entry: `seq`, `at_ms`, `kind`, the
  body's length field and the body.
- The footer's `chain` is `c(count)`.

Implemented in: `userland/nsh_client/src/record/walk.rs`, `userland/nsh_wire/src/record/entry.rs`.

## nsh-ckpt/1

A checkpoint is the host keys' signature over the chain after one entry.
It lets a record that has no footer, as a crash leaves it, be checked as
far as the checkpoint.

| Field | Size | Rule |
|---|---|---|
| magic | 4 | `NSHC` |
| version | `u16` | 1 |
| reserved | 2 | zero |
| `session_id` | 16 | the record's |
| `seq` | `u32` | the last entry it covers |
| `chain` | 64 | `c(seq + 1)`: the chain after entry `seq` |
| `sig` | `HybridSig`, 3373 | |

A checkpoint is 3465 bytes. The host keys sign:

```
SHA-512("nsh/1 record-ckpt\0" || SHA-512(every checkpoint byte before sig))
```

Checkpoints are kept beside the record, not inside it.

A host writes a checkpoint after every `ckpt_every` entries, a number it
chooses (0 writes none), and keeps each beside the record as the record
grows. A record cut short by a crash is then checked as far as its last
checkpoint.

Implemented in: `userland/nsh_server/src/record/ckpt.rs`.

Implemented in: `userland/nsh_client/src/record/ckpt.rs`.

## Checking a record offline

A checker is given the record, the attestation document, the fleet's
client policy (fleet id, enrolment authorities, withdrawn enrolments, and
the accepted releases its attestation checker holds), any checkpoints,
and optionally the session id, `TH(SERVER_HELLO)` and subject the record
must name.

1. **Parse.** The record parses as `nsh-record/1` with every rule above.
   If it does not, and no checkpoint is given or it is over 64 MiB, it is
   refused here. If it does not and checkpoints are given, it is checked
   as a record without a footer (below).

   Then the header must name the `session_id`, `transcript`
   (`TH(SERVER_HELLO)`) and `subject` the caller expects, for each one the
   caller names.
2. **The document.** SHA-512 of the document given equals `doc_hash`. The
   document passes steps 1 to 6 of [nsh-v1-attest.md](nsh-v1-attest.md).
   The registry root is bound through the quote, since the quote's
   `extraData` covers it. A record does not carry the registry entries,
   so step 7's entries, `nsh.front`'s included, are checked only online.
3. **The challenge.** The challenge is recomputed from the header's
   `transcript` and `host`, and the document carries it.
4. **The enrolment.** The header's certificate passes step 4 of
   [nsh-v1-attest.md](nsh-v1-attest.md) against the document's AK and the
   header's `host`, and is not revoked. Its host name and expiry are
   checked online only: offline there is no typed host and no clock. PCRs
   0, 1, 2 and 7 in the document equal the certificate's.
5. **The chain.** Every checkpoint given parses, names the header's
   `session_id`, and verifies under the header's `host`. The chain is
   recomputed over the entries. After each entry a checkpoint names, the
   chain equals the checkpoint's `chain`. Every checkpoint names an entry
   the record holds. The chain after the last entry equals the footer's
   `chain`.
6. **The signature.** Both halves of the footer's `sig` verify under the
   header's `host`.

A record with a footer is complete only if all six steps pass.

### A record without a footer

When the record does not parse whole and checkpoints are given:

- the header must parse, name what the caller expects, and pass steps 2
  to 4;
- every checkpoint must verify;
- entries are read in order while each one parses, has the next `seq`,
  and does not go back in time; the first that does not ends the reading;
- the chain over the entries read must meet every checkpoint, and every
  checkpoint must name an entry that was read.

The record is then reported cut short at the highest `seq` a checkpoint
names. Entries 0 to that `seq` are checked and signed. Nothing after it
is. Such a record is never reported complete.

### The result

A checker reports the header's `session_id`, `subject` and `start_ms`, and
one verdict:

| Verdict | Meaning |
|---|---|
| complete | All six steps passed. Reports `count`, `end_ms`, `end_reason`, and whether a `BOX` entry is present. |
| cut short at `seq` | No footer. Checked as far as the last checkpoint. Reports whether a `BOX` entry is at or before `seq`. |
| refused | The step that failed. |

It also reports that the registry entries were not checked offline and
that the registry root is bound by the quote.

`nsh-verify-record <file> --doc <doc> --fleet <32 hex> --authority <key>
[--ckpt <file>]... [--revoked <file>]... [--session <32 hex>]
[--subject <128 hex>]` runs this check. An authority key file holds the
raw 1984-byte hybrid key. A revoked file holds 64-byte SHA-512 digests of
withdrawn enrolments.

| Exit code | Meaning |
|---|---|
| 0 | complete |
| 4 | complete, holding a provisional `BOX` entry |
| 3 | cut short |
| 1 | refused |
| 2 | usage |

The live client applies part of this to the record it receives: the
session id, `TH(SERVER_HELLO)`, subject and document hash
([nsh-v1-wire-session.md](nsh-v1-wire-session.md), "The ending order").

Implemented in: `userland/nsh_client/src/record/verify.rs`, `userland/nsh_client/src/record/header.rs`, `userland/nsh_client/src/record/cut.rs`, `userland/nsh_client/src/record/partial.rs`, `userland/nsh_client/src/record/walk.rs`, `userland/nsh_client/src/record/bound.rs`, `userland/nsh_client/src/record/report.rs`, `userland/nsh_client/src/cli/`, `userland/nsh_client/README.md`.

## Keeping records on a host

`userland/nsh_record_store` keeps each session's record on a host that has a
filesystem. A `Keeper` follows one connection's server events and writes,
into a directory the host owns (made `0700` if new):

| File | Holds |
|---|---|
| `<session id in hex>.record` | every `Record` event's bytes, in order |
| `<session id in hex>.ckpts` | every `nsh-ckpt/1`, end to end, 3465 bytes each |

- Both files are created new and owner-only (`0600`). A second session with
  the same id is refused; nothing is written over.
- The record's header comes before `Started` names the session, so it is held
  until then, never more than the largest header (`RECORD_HEADER_MAX`,
  16 KiB): a keeper handed more before `Started` fails.
- Before a checkpoint is written, the record bytes it covers are synced. A
  checkpoint therefore never names an entry a crash lost, and a record cut
  short checks as far as its last checkpoint. The checker takes one
  checkpoint per `--ckpt` file, so split the `.ckpts` file into 3465-byte
  pieces first (`split -b 3465 -d ID.ckpts ID.ckpt.`).
- Both files are synced when the session closes.
- A host whose keeper fails ends the connection, so no command runs
  unrecorded. `nshd` keeps records when `$NSHD_RECORDS` names a directory, with
  a checkpoint every 16 entries.

**nsh.front keeps no record in this version.** It holds no filesystem
capability, so the record and checkpoints the server hands it are dropped,
and it writes no checkpoints (`ckpt_every: 0`). The client still receives the
whole record, signed by the host keys, at the session's end (`RECORD`), and
keeps it with `--records`. A session cut short before its end leaves no
record anywhere. Keeping one in the guest needs a store nsh.front can write,
which changes its capability mask and so its measurement.

The record does not carry the attestation document; the client keeps the
document beside its own copy (`--records`).

Implemented in: `userland/nsh_record_store`.
