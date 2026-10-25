# nsh v1: wire protocol, framing and handshake

## Transport and limits

- nsh/1 runs over TCP, on port 4422 by default. The server profile can
  set another port, and the client takes one as `nsh -p PORT`. Integers are little-endian unless a field says otherwise.
  A `bytes8` or `bytes16` field is a length (u8 or u16) followed by that
  many bytes.
- Every frame is laid out as `len: u32 | type: u8 | body`. `len` counts the
  type and the body. A reader refuses a frame whose `len` is over the
  phase's cap. It does this before it reads the body.
- Each stage caps the type byte and the body together:
  - Hellos: 2048.
  - From `SERVER_HELLO` until login is settled (`HOST_PROOF`, `USER_AUTH`,
    `AUTH_OK` or `AUTH_REFUSED`, or an `ABORT`): 1179648 (1152 KiB), so
    `HOST_PROOF` carries a document with a full 1 MiB event log.
  - After login: 16385, which is one type byte and 16384 bytes of body.

  A sealed frame's `len` is that plus the 16-byte tag.
- A body must be consumed exactly. Trailing bytes are refused, a short body
  is refused, and a nonzero reserved field or flag is refused. Any of these
  ends the session with `ABORT(PROTOCOL)`.
- The whole handshake, from the TCP accept to `AUTH_OK`, must finish within
  30 s.

## The two hellos (plaintext)

`CLIENT_HELLO`, type 0x01, exact body length 1258:

| Offset | Field | Size |
|---|---|---|
| 0 | magic `"NSH1"` | 4 |
| 4 | version = 2 | u16 |
| 6 | suite = 1 | u16 |
| 8 | flags = 0 | u16 |
| 10 | client_random | 32 |
| 42 | X-Wing encapsulation key | 1216 |

`SERVER_HELLO`, type 0x02, exact body length 1162:

| Offset | Field | Size |
|---|---|---|
| 0 | magic `"NSH1"` | 4 |
| 4 | version = 2 | u16 |
| 6 | suite = 1 | u16 |
| 8 | flags = 0 | u16 |
| 10 | server_random | 32 |
| 42 | X-Wing ciphertext | 1120 |

Version 2 raised the login cap from 262144 so that `HOST_PROOF` can carry
an attestation document with the firmware's whole event log, which the
loader takes up to 1 MiB. A peer that speaks version 1 is refused at its
hello with `ABORT(VERSION)`, before any proof is sent, so a document is
never cut short to fit an older cap.

The preamble (magic, version, suite and flags) is judged before the
length. After it, the hello must be exactly the random and the key or
ciphertext: nothing short and nothing more. A server that sees a version
other than 2 or a suite other than 1 sends
`ABORT(VERSION)` in plaintext and closes. That is type 0x7F with a body of
`reason: u16`. A server at its session cap sends `ABORT(BUSY)` in its
place.

A plaintext `ABORT` may carry only `VERSION` or `BUSY`, because nothing is
authenticated before the keys exist. The promise to answer `VERSION`
holds for any hello of up to 2048 bytes. A later version must keep its
hello within that size, or be refused as `PROTOCOL`.

## Keys

Here `H` is SHA-512, `Extract` and `Expand` are HKDF-SHA-512, and labels
are ASCII strings.

**The transcript.** The transcript is a chain of SHA-512 values, one link
per frame, in the order the frames were sent:

- Each frame's entry is `n: u32 | type | body`, where `n = 1 + body
  length`. `n` is always the plaintext length, never the length on the
  wire, which for a sealed frame also counts the tag.
- The chain starts at `TH(start) = H("nsh/1 transcript\0")`.
- Each frame F then gives `TH(F) = H(TH(previous) || F's entry)`.

**A frame that carries its own signatures.** These are `HOST_PROOF` and
`USER_AUTH`. They sign `TH(F without its signatures)`, which is
`H(TH(previous) || n' | type | body[..m])`. Here `m` is the body's length
less the 3373 bytes of the hybrid signature, and `n' = 1 + m`. `TH(F)`
itself is computed over the whole entry, signatures included, as for any
other frame.

Each link is a single value, so a prefix needs no saved hash state.

- `ss` is the X-Wing shared secret (32 bytes). The X-Wing key pair in
  `CLIENT_HELLO` is made for this one connection and discarded after
  `SERVER_HELLO`. A reused key would let a server or an observer link one
  client's connections before any credential is sent.
- `hs = Extract(salt = TH(SERVER_HELLO), ikm = ss)`.
- The handshake keys are `Expand(hs, "nsh/1 c2s hs", 32)` and
  `Expand(hs, "nsh/1 s2c hs", 32)`.
- Once `AUTH_OK` has been sent and received,
  `ts = Extract(salt = TH(AUTH_OK), ikm = hs)`. The session keys are
  `Expand(ts, "nsh/1 c2s data", 32)` and `Expand(ts, "nsh/1 s2c data", 32)`.
- `hs` and the handshake keys are wiped when the session keys are made.

## Sealed frames

After `SERVER_HELLO`, every frame is laid out as
`len: u32 | ChaCha20-Poly1305(type | body) | tag`. The AAD is the four
`len` bytes. The nonce is four zero bytes followed by `seq: u64`, kept per
direction. `seq` starts at 0 for each key and is never sent on the wire.

- A frame that fails to open ends the session. Nothing is answered.
- At `seq = 2^32`, or once 2^32 bytes have been sealed in one direction,
  that direction is spent. Not even an `ABORT` can be sealed, since no
  nonce is left. The side whose sending direction is spent closes the
  connection without a frame. The side whose receiving direction is spent
  refuses the next frame and closes. Version 1 has no rekeying.

## The handshake, in order

1. C to S: `CLIENT_HELLO`.
2. S to C: `SERVER_HELLO`.
3. S to C: `HOST_PROOF`, sealed. See attest.md.
4. The client checks its policy. On a refusal it sends `ABORT(POLICY)` and
   closes. The user name, the grant and the user key are never sent.
5. C to S: `USER_AUTH`, sealed. See roles.md.
6. S to C: `AUTH_OK` or `AUTH_REFUSED`, sealed. After `AUTH_REFUSED` both
   sides close.

A frame out of this order is `ABORT(PROTOCOL)`. The session frames are in
[nsh-v1-wire-session.md](nsh-v1-wire-session.md).
