# nsh_client

The client side of nsh/1, as specified in
[docs/nsh/nsh-v1.md](../../docs/nsh/nsh-v1.md) and its parts, and
the offline checker of session records.

- **`Client`** is a sans-IO state machine. The caller feeds it the
  server's bytes (`feed`) and sends what it hands back (`take_output`).
  It reads no clock and no entropy: the time, the X-Wing seed, the client
  random and the signing randomness are the caller's.
- **The handshake runs in the one order the spec allows.** `CLIENT_HELLO`
  goes out, then `SERVER_HELLO` comes in, then `HOST_PROOF`. Then the host
  is checked: its hybrid signature over the transcript (both halves), the
  attestation document through the caller's `Attester`, the challenge
  again, the enrolment certificate under the fleet authority, and the
  firmware PCRs against the enrolment. Only after all of these does
  `USER_AUTH` go out. Then `AUTH_OK` comes in, and the session starts.
- **Every frame out of order, unknown, oversize, short or trailing is
  refused**, and so is a reason code its frame may not carry. A refusal
  ends the session. Once keys exist, the server is told the one code the
  refusal allows (`POLICY` for a refused host); a frame that does not
  open is not answered.
- **The `Attester` trait** is the `NONOSATT` checker's narrow interface:
  the document, the challenge and the registry entries go in, and either
  the facts the client checks the enrolment against or a typed refusal
  come out. The attester names where the challenge sits in the document
  (`challenge_at`), and the client reads it there itself, so an attester
  cannot echo back the challenge it was given.
- **`verify_record`** carries out steps 1 to 6 of "Checking a record
  offline" and every checkpoint given. A record without a footer is
  reported cut short at its last checkpoint, never as complete.
  It also holds the header to a `Bound` (session id, `TH(SERVER_HELLO)`,
  subject) when the caller names one. The client does the same for the
  record it receives in `RECORD` frames, and also checks the document
  hash. The result says what offline checking could not do: the registry
  entries, which a record does not carry. `nsh-verify-record` (feature
  `cli`) is the host tool built over it. It prints whose record it is,
  takes `--session`, `--subject` and `--revoked`, and exits 4 for a
  complete record holding a provisional `BOX` entry.

## `nsh`, the command line

`nsh` (features `cli` and `nonos`) connects to a host and logs in:

```
nsh --connect HOST:PORT --host NAME --fleet HEX --authority FILE ... \
    --releases FILE --key FILE --grant FILE [--revoked-enrol FILE ...] \
    [--escalate FILE ...] [--records DIR]
```

- The host is checked as every client checks it, with `NonosAttester`
  over the fleet's accepted releases. The releases file has one
  `release VERSION PCR8 PCR9 LOADER FRONT NETWORK,...|-` line each. The
  key and grant leave only after the host passes.
- The user's key is in nsh-enrol's secret file format (`nsh-roles keygen`
  writes one). The grant is one `nsh-roles grant` wrote.
- On a terminal the session runs in raw mode, and the terminal is put back
  however the session ends. The window size goes to the host at login.
  Ctrl-], or the end of standard input, closes the session.
- Each escalation grant is sent in turn once logged in.
- The record is checked as soon as it arrives. With `--records` it is kept
  with the host's document, for `nsh-verify-record`.
- The exit status is the shell's, 255 when the session was refused or
  broken, and 2 for a usage error.

## Checking a record offline

A record does not carry the registry entries, so step 2 of
[docs/nsh/nsh-v1-record.md](../../docs/nsh/nsh-v1-record.md), "Checking a
record offline", holds the document to steps 1 to 6 of
[docs/nsh/nsh-v1-attest.md](../../docs/nsh/nsh-v1-attest.md) and the
registry root through the quote. The root is bound, because the quote's
`extraData` covers it; the entries themselves, including `nsh.front`'s,
are checked only online.

Revocation is checked both online and offline, since it needs no clock.
The enrolment's expiry and host name are checked online only.

The keys are wiped when the session ends or the client is dropped: the
X-Wing key pair, the handshake secret and both directions' keys. So are
the user's key halves in `UserKey`.

The crate is `no_std` with `alloc`, and builds for `x86_64-nonos-user`.
The proofs are in [nsh_client_proofs](../nsh_client_proofs).
