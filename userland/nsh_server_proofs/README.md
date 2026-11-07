# nsh_server_proofs

Host proofs for `nsh_server`. There are 73 tests. Every run is against the
real client, `nsh_client`, with the real `NONOSATT` checker, `nsh_attest`
(`NonosAttester`), and the real role table, `nsh_roles`.

The host is nsh_attest_proofs' passing spec. Its enrolment, its P-256 AK
and its version 5 documents are built by that crate's builders, compiled
here from their sources, and made afresh for each challenge the server
asks the kernel for. The role table, revocation lists, grants and the
user's keys are nsh_roles_proofs' builders, also compiled from source.
Every signature is real: Ed25519 and ML-DSA-65 keys from fixed seeds.

- **The good run** (`good`). The client accepts the host, the table admits
  the user, a command runs, and the record the client receives is the one
  the host kept, and checks offline with `verify_record`.
- **The hello** (`hello`). Another version or suite is told `VERSION` in
  plaintext. A broken, short, oversize or out-of-order first frame is closed
  without a word. The hello may arrive a byte at a time, and `busy` is read
  by the client. A kernel that cannot attest, or that hands back entries
  that are not whole records, is told `BUSY`. Host keys that do not sign
  send no proof, and no randomness means no hello. Neither does a document
  that is empty or over the cap. The handshake deadline holds before and
  after keys, and an invalid config is refused.
- **The login** (`login`, `admit`). Each role-table refusal reaches the
  client as its `AUTH_REFUSED` code and is logged whole. That covers the
  issuer and each half of its signature, the window, the fleet, the role,
  the capabilities, a revoked grant, a stale list and a list the table
  does not trust. A raw client sends what the real one never does: other
  frames, broken bodies, an escalation grant, another key, an `ABORT`
  (with a code it may and may not carry), a changed byte, and frames while
  the host starts the session. `begin` names no more than the grant, and
  `deny` says `BUSY`.
- **The session** (`session`). Input, window and signals reach the host.
  Each body the rules refuse ends the session, and the record says
  aborted. `CLOSE`, an exit, the idle timeout, the grant's end and a
  revocation each end with their own footer reason. Output is cut into
  frames and counted, commands, digests and refused calls are recorded,
  and host calls wait for a running session.
- **Escalation** (`escalate`). One runs until its window closes, is
  revoked, or the session ends, and each is recorded. Every refusal is
  answered and recorded while the session goes on, one runs at a time,
  and a host that cannot start one ends the session.
- **The record** (`record`). Checkpoints check the whole record and one cut
  short. A record at its cap stops the shell and still ends signed, with
  every output byte counted and its `EXIT` last. A clock that goes back
  never takes the record back. A record the host keys cannot sign is not
  sent. `HostKey` signs with both halves.
- **The role store** (`store`). The table is held to its pin and serial.
  A list is never replaced by an older one, or by one the table does not
  trust, and a refused install keeps the list held. A store that cannot
  vouch refuses logins and stops every session once.
- **`nsh`** (`cli`). The client's command line runs against the loopback
  host from the files an operator hands a user. It logs in, runs a
  command, prints its output, keeps the record with the host's document,
  and exits with the shell's status. It refuses a host attesting another
  handshake with 255, and bad arguments with 2.
- **Noise** (`noise`). The fuzz target's oracle, `kit::noise`, runs on
  fixed inputs: noise at each stage, every kind with bodies at their edges,
  and a good `USER_AUTH` flipped or cut short. The server never panics, and
  every frame it sends opens and is one the client may receive then.
  Nobody is admitted, nothing follows the end, and a record that ends
  checks.
- **Loopback** (`loopback`). The server runs over TCP on 127.0.0.1 with
  `/bin/sh` behind it and the system's randomness. An attested host runs
  `echo nsh-loopback-$((6*7))` and the client sees `nsh-loopback-42`. A
  host whose kernel cannot attest, or attests another handshake, is
  refused before any credential leaves, and an expired grant is refused.
  The wire shows the hybrid suite. The plaintext hellos carry suite 1 with
  an X-Wing key of 1216 bytes (ML-KEM-768 then X25519) and a ciphertext of
  1120. The record's Ed25519 and ML-DSA-65 halves each verify alone, and
  spoiling either one fails the record.

Each rule of `nsh_server` was also broken on purpose, one at a time: 35
mutants, 34 caught. The one left removes the check that an `AUTH_REFUSED`
code is one that frame may carry. It is equivalent: the only refusal
outside that set is a login with an escalation grant, which `nsh_wire`'s
`USER_AUTH` parser refuses first.

The fuzz target is in `fuzz/`: `nsh_server_feed`, over `kit::noise`. Seed
with `python3 -I fuzz/seed.py --out <dir>`, then run
`cargo +nightly-2026-01-16 fuzz run nsh_server_feed <dir>/nsh_server_feed`.
It has run for ten minutes from the seeds, 10973 inputs, with nothing
found.

Run with `cargo test --release` from this directory.
