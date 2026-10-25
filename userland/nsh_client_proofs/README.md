# nsh_client_proofs

Host proofs for `nsh_client`. The test server in `src/kit` is built only
from `nsh_wire` and `nsh_crypto`, with real Ed25519 and ML-DSA-65 keys
made from fixed seeds. A stand-in attestation checker reads a test
document. Each proof runs the good handshake and changes one thing.

- **The good run.** The handshake runs end to end, including fed one byte
  at a time. No frame answers `SERVER_HELLO` or a part of `HOST_PROOF`.
  `USER_AUTH` is the only frame after a good `HOST_PROOF`, and the server
  accepts it. The session keys carry frames both ways.
- **`SERVER_HELLO`.** The magic, the version, the suite, the flags, a
  short or a long hello, an oversize or an empty length, an unknown or
  out-of-order type, and plaintext `ABORT` codes are each checked. A
  random or a ciphertext changed in flight means `HOST_PROOF` does not
  open. Nothing is sent before keys exist.
- **Sealed frames.** Every other known type in place of `HOST_PROOF`,
  including `AUTH_OK`, is refused, and no credential follows. So are
  unknown types, oversize and empty frames, and a changed byte in
  `HOST_PROOF`, `AUTH_OK`, `AUTH_REFUSED` and every session frame (none
  of these is answered). The length field is bound to the frame, and a
  replayed frame is refused.
- **The host.** Each half of the host signature is checked, and so are
  other keys and a signature over another transcript. A body changed
  anywhere after signing is answered with `ABORT(POLICY)` or
  `ABORT(PROTOCOL)` alone.
- **Every attester refusal and every enrolment refusal.** The authority
  key, each half of the enrolment signature, the fleet, the AK, the host
  keys, the host name, the expiry at its edge, a withdrawn certificate,
  and an empty policy are each checked. So are the challenge (checked
  again by the client when the attester is lax) and each firmware PCR.
- **The login.** The user signature is bound to the transcript. Each
  `AUTH_REFUSED` code is checked, and so are codes it may not carry, a
  broken `AUTH_OK`, a grant for another key, and an escalation grant at
  login. The signer is asked only for an accepted host, and a failed
  signature sends nothing.
- **The session.** It checks frames the server never sends, the output
  size, escalation answers only for an escalation asked, and sealed
  `ABORT` codes. It checks the ending order `EXIT`, `RECORD`, `CLOSE` and
  the 64 MiB record cap, and that nothing is sent before `AUTH_OK` or
  after `CLOSE`. Bodies the server would refuse are not sent.
- **Records.** A record joined from `RECORD` frames checks against the
  handshake's document. The checks cover:
  - a good record, alone and with checkpoints;
  - `BOX` entries and their body rules;
  - one byte flipped at sampled positions and every field edge, with and
    without checkpoints;
  - dropped and reordered entries, with the sequence fixed and the chain
    recomputed;
  - the document, the challenge, the enrolment and the footer signature;
  - records cut short, torn, or holding checkpoints that are forged,
    beyond the record, for another session or malformed.
- **The tool.** `nsh-verify-record` is run over real files, and its usage
  errors are checked.

The fuzz targets are in `fuzz/`: `nsh_client_feed` and
`nsh_record_verify`. Seed them with `fuzz/seed.py`.

Run with `cargo test` from this directory.
