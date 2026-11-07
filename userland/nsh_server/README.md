# nsh_server

The host side of nsh/1, as specified in
[docs/nsh/nsh-v1.md](../../docs/nsh/nsh-v1.md) and its parts.

- **`Server` is a sans-IO state machine.** The host feeds it the client's
  bytes (`feed`), sends what it hands back (`take_output`), and acts on
  its events (`events`). It reads no clock and no entropy: every call
  takes the host clock, and every random byte is the caller's.
- **The handshake runs in the one order the spec allows.** `CLIENT_HELLO`
  comes in and `SERVER_HELLO` goes out. Then the kernel is asked for its
  `NONOSATT` version 5 document over this handshake's challenge
  (`Attestation`), and `HOST_PROOF` goes out. That frame carries the host
  keys, the enrolment, the document, the registry entries and both halves
  of the host keys' signature, each field held to the client's rules
  before it is laid out. Then `USER_AUTH` comes in, and `nsh_roles` checks it
  against the host's trusted table and list (`Roles`), in the spec's order.
  `RoleStore` holds both as bytes: the table verified under the key id the
  host pins and no older than its last serial, the list under that table
  and never older than the one it replaces. Every check opens them again
  from those bytes. A store that cannot vouch for its list refuses every
  login and escalation, and stops every running session.
  A login that passes is handed to the host (`Event::Admit`), which starts
  the session capsule and answers `begin`, naming no more capabilities than
  the grant, or `deny`.
- **Only `VERSION` and `BUSY` are told before keys exist** (`busy` builds
  the second). Any other broken hello is closed without a word. Once keys
  exist, a refusal is told the one code the wire allows, a login refusal as
  `AUTH_REFUSED`, and logged whole as a `Why`. A frame that does not open,
  a spent direction, the client's `ABORT`, or host keys that do not sign
  are not answered.
- **The session.** Input, window and signals reach the host as events, each
  held to its body's rules. `CLOSE`, the idle timeout, the grant's end, the
  role's longest session, a revoked grant and a record at its cap each stop
  the shell once (`Event::Stop`). The host then calls `exit`, and the server
  sends `EXIT`, the signed record in `RECORD` parts, and `CLOSE`. An
  escalation is checked against the session's own login, one at a time. It
  is handed to the host to start (`escalation_begun`), or refused with its
  code, and it ends at its window, on revocation, or with the session.
- **The record.** It is written as the session runs: the header at
  `AUTH_OK`, then commands (or their SHA-256 for a role that keeps digests,
  or a line over 4096 bytes), output runs as a count and SHA-256, refused
  calls, escalations and the exit. Each entry is read back with the
  checker's parser, chained, and handed to the host as it is written
  (`Event::Record`). A checkpoint every `ckpt_every` entries
  (`Event::Checkpoint`) lets a record cut short by a crash still check. The
  record is held to `max_record`, with room kept back for the entries every
  session ends with and the footer, so it always ends signed. If the host
  keys fail at the end, the footer's signature is missing and the
  checkpoints cover the record.
- **Bounded per connection.** One frame of its stage's cap is held at a
  time. The handshake has a deadline, and the record has a cap of 64 KiB to
  64 MiB. Output goes out in frames of 16384 bytes, and one escalation runs
  at a time. Keys are wiped when the connection ends.

The footer's end reasons and the `ESC_ENDED` reasons are in `EndReason` and
`EscEnd`, and in [docs/nsh/nsh-v1-record.md](../../docs/nsh/nsh-v1-record.md).

Every primitive is one the tree already carries, through `nsh_crypto`
(X-Wing, the key schedule, sealed frames), `nonos_ed25519` and
`nonos-mldsa65` (the host keys, `HostKey`). The crate is `no_std` with
`alloc`. The proofs are in [nsh_server_proofs](../nsh_server_proofs).
