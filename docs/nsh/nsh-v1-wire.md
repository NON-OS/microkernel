# nsh/1 wire format

This part covers framing, the handshake frames and the cryptography under
them. Session frames are in [nsh-v1-wire-session.md](nsh-v1-wire-session.md).

## Encoding rules

- Every integer in an nsh/1 body is little-endian. (The attestation
  document inside `HOST_PROOF` is big-endian; see [nonosatt-v5.md](nonosatt-v5.md).)
- Every body is read exactly. Bytes left after the last field are refused.
- A reserved field or flag that is not zero is refused.
- A length is judged against its cap before any byte it covers is read. A
  length over the cap is refused whether or not the bytes follow.
- A body is refused whole. No field of a refused body is used.

| Notation | Meaning |
|---|---|
| `u8`, `u16`, `u32`, `u64` | Unsigned integer, little-endian. |
| `i32` | Signed integer, little-endian, two's complement. |
| `bytes8(min..max)` | A `u8` length, then that many bytes. |
| `bytes16(min..max)` | A `u16` length, then that many bytes. |
| `bytes32(min..max)` | A `u32` length, then that many bytes. |
| `HybridPublic` | Ed25519 public key (32), then ML-DSA-65 public key (1952): 1984 bytes. |
| `HybridSig` | Ed25519 signature (64), then ML-DSA-65 signature (3309): 3373 bytes. |
| key id | SHA-256 of a `HybridPublic`: 32 bytes. |
| key digest | SHA-512 of a `HybridPublic`: 64 bytes. |

A versioned body starts with a preamble: a 4-byte magic, a `u16` version
and two reserved zero bytes. A wrong magic and a wrong version are told
apart from a malformed body, but each is answered `PROTOCOL`.

Implemented in: `userland/nsh_wire/src/reader.rs`, `userland/nsh_wire/src/reader_fields.rs`, `userland/nsh_wire/src/keys.rs`.

## Names

| Name | Shape | Length |
|---|---|---|
| user | `[a-z][a-z0-9._-]*` | 1 to 32 |
| role | `[a-z][a-z0-9-]*` | 1 to 16 |
| host | Lowercase DNS name: labels of 1 to 63 of `[a-z0-9-]`, not starting or ending with `-`, joined by single dots, no final dot. | 1 to 253 |
| capability | `[A-Z][A-Za-z0-9]*` | 1 to 32 |

A name of another shape is refused.

Implemented in: `userland/nsh_wire/src/names.rs`.

## Capability lists

A capability list names kernel capabilities by name, never by bit number.

| Field | Size |
|---|---|
| count | `u8`, 0 to 41 |
| each name | `bytes8(1..32)`, capability shape |

- Names are in strictly ascending byte order. A list therefore names no
  capability twice, and one set has one encoding.
- 41 is the number of kernel capabilities. Whether a list may be empty is
  set by the body that carries it.

Implemented in: `userland/nsh_wire/src/cap_names.rs`, `userland/nsh_wire/src/cap_names_write.rs`.

## Frames

| Field | Size | Notes |
|---|---|---|
| `len` | `u32` LE | Number of bytes after this field. |
| `type` | 1 | Frame type. Sealed after the hellos. |
| `body` | `len - 1` (plaintext) or `len - 17` (sealed) | Sealed after the hellos. |
| `tag` | 16 | Poly1305 tag. Sealed frames only. |

- The two hellos are plaintext: `len = 1 + body length`.
- Every frame after the hellos is sealed: `len = 1 + body length + 16`.
- The receiver judges `len` against its stage's cap as soon as the four
  bytes arrive, before it keeps any body byte. It refuses `len > cap + t`
  (oversize) and `len < 1 + t` (short), where `t` is 16 for a sealed stage
  and 0 otherwise.
- After opening, the type byte and body together may not exceed the cap.
- A type byte that names no frame type is refused. A known type that the
  receiving stage does not admit is refused as out of order. Both are
  answered `PROTOCOL`.
- A sender computes `len` from the cap of the stage the receiver is in,
  and does not send a frame the receiver would refuse for size.

Implemented in: `userland/nsh_wire/src/frame.rs`, `userland/nsh_client/src/framer.rs`.

### Frame types

| Type | Name | Sent by |
|---|---|---|
| `0x01` | `CLIENT_HELLO` | client |
| `0x02` | `SERVER_HELLO` | server |
| `0x10` | `HOST_PROOF` | server |
| `0x20` | `USER_AUTH` | client |
| `0x21` | `AUTH_OK` | server |
| `0x22` | `AUTH_REFUSED` | server |
| `0x30` | `INPUT` | client |
| `0x31` | `OUTPUT` | server |
| `0x32` | `WINDOW` | client |
| `0x33` | `SIGNAL` | client |
| `0x40` | `ESCALATE` | client |
| `0x41` | `ESCALATE_RESULT` | server |
| `0x50` | `EXIT` | server |
| `0x51` | `RECORD` | server |
| `0x52` | `CLOSE` | both |
| `0x7F` | `ABORT` | both |

Implemented in: `userland/nsh_wire/src/kind.rs`.

### Caps per stage

The cap counts the type byte and the body, not the length field or the tag.

| Receiving stage | Who is in it | Admits | Cap | Sealed |
|---|---|---|---|---|
| `ClientHello` | server, before anything | `CLIENT_HELLO` | `HELLO_CAP` = 2048 | no |
| `ServerHello` | client, after its hello | `SERVER_HELLO`, `ABORT` | `HELLO_CAP` = 2048 | no |
| `HostProof` | client, after `SERVER_HELLO` | `HOST_PROOF`, `ABORT` | `AUTH_CAP` = 1 179 648 (1152 KiB) | yes |
| `UserAuth` | server, after `HOST_PROOF` | `USER_AUTH`, `ABORT` | `USER_AUTH_CAP` = 10 405 | yes |
| `AuthResult` | client, after `USER_AUTH` | `AUTH_OK`, `AUTH_REFUSED`, `ABORT` | `AUTH_RESULT_CAP` = 1 474 | yes |
| `ServerSession` | server, after `AUTH_OK` | `INPUT`, `WINDOW`, `SIGNAL`, `ESCALATE`, `CLOSE`, `ABORT` | `SESSION_CAP` = 16 385 | yes |
| `ClientSession` | client, after `AUTH_OK` | `OUTPUT`, `ESCALATE_RESULT`, `EXIT`, `RECORD`, `CLOSE`, `ABORT` | `SESSION_CAP` = 16 385 | yes |

`SESSION_CAP` is 1 + `MAX_DATA`, where `MAX_DATA` = 16 384.

`USER_AUTH_CAP` is 1 + the largest legal `USER_AUTH` body: the user's
hybrid key (1984), the grant's `u16` length (2), the largest base grant
(`MAX_BASE_GRANT` = 5045, which is `MAX_GRANT` less the two approvals only an
escalation carries) and the signature (3373). An escalation grant always
carries an approval, so a `USER_AUTH` holding one is refused at its length.
`AUTH_RESULT_CAP` is 1 + the largest `AUTH_OK` body: the session id (16), the
idle timeout (4) and a full capability list (1 + 44 × 33). `AUTH_REFUSED` and
`ABORT` are three bytes framed. Only `HOST_PROOF`, which carries the
attestation document, needs `AUTH_CAP`.

Implemented in: `userland/nsh_wire/src/expect.rs`, `userland/nsh_wire/src/limits.rs`.

## The hellos

Both hellos share a 10-byte preamble.

| Offset | Field | Size | Value |
|---|---|---|---|
| 0 | magic | 4 | `NSH1` |
| 4 | version | `u16` | 2 |
| 6 | suite | `u16` | 1 |
| 8 | flags | `u16` | 0 |

`CLIENT_HELLO` body, 1258 bytes:

| Offset | Field | Size |
|---|---|---|
| 0 | preamble | 10 |
| 10 | `client_random` | 32 |
| 42 | X-Wing encapsulation key | 1216 |

`SERVER_HELLO` body, 1162 bytes:

| Offset | Field | Size |
|---|---|---|
| 0 | preamble | 10 |
| 10 | `server_random` | 32 |
| 42 | X-Wing ciphertext | 1120 |

- The preamble is judged first. A wrong magic is `PROTOCOL`. A version
  other than 2 or a suite other than 1 is `VERSION`, whatever the hello's
  size. Non-zero flags are `PROTOCOL`.
- After the preamble, the body must be exactly the random and the key or
  ciphertext: nothing short, nothing more.
- The client makes its X-Wing key pair from 32 fresh random bytes made for
  this one connection. The key pair is used once: decapsulation consumes it.
- The server encapsulates to the client's key with 64 fresh random bytes
  used for nothing else. An encapsulation key that is not a valid X-Wing
  key is refused.
- Both sides then hold the same 32-byte shared secret `ss`.

Implemented in: `userland/nsh_wire/src/hello.rs`, `userland/nsh_wire/src/hello_preamble.rs`, `userland/nsh_crypto/src/kem.rs`, `userland/nsh_client/src/client/new.rs`, `userland/nsh_client/src/client/hello.rs`.

## The transcript

The transcript is a SHA-512 chain over every handshake frame, in the
order the frames crossed the wire.

```
TH0      = SHA-512("nsh/1 transcript\0")
TH(F)    = SHA-512(TH(previous) || n || type || body)
n        = u32 LE, 1 + body length
```

- `n` is the plaintext length of type and body, whether or not the frame
  was sealed. The tag is not included.
- `TH(F)` is the chain value after frame `F` is added.
- The frames added are `CLIENT_HELLO`, `SERVER_HELLO`, `HOST_PROOF`,
  `USER_AUTH` and `AUTH_OK`, in that order. Session frames are not added.
- `TH(F without its signatures)` is one link over the bytes of `F`'s body
  before its trailing `HybridSig`, with `n = 1 + that length`, made from
  the chain as it stood before `F`. It does not change the chain.
- `HOST_PROOF` is added only after the client has accepted the host.

Implemented in: `userland/nsh_crypto/src/transcript.rs`, `userland/nsh_client/src/client/proof.rs`, `userland/nsh_client/src/client/user.rs`, `userland/nsh_client/src/client/auth.rs`.

## Keys

HKDF is HKDF-SHA-512. `Extract(salt, ikm)` is `HMAC-SHA-512(salt, ikm)`.
`Expand(prk, info, 32)` gives 32 bytes. The info strings carry no
terminating zero byte.

```
hs        = Extract(salt = TH(SERVER_HELLO), ikm = ss)
c2s_hs    = Expand(hs, "nsh/1 c2s hs",   32)
s2c_hs    = Expand(hs, "nsh/1 s2c hs",   32)

ts        = Extract(salt = TH(AUTH_OK), ikm = hs)
c2s_data  = Expand(ts, "nsh/1 c2s data", 32)
s2c_data  = Expand(ts, "nsh/1 s2c data", 32)
```

| Keys | Seal |
|---|---|
| `s2c_hs` | `HOST_PROOF`, `AUTH_OK`, `AUTH_REFUSED`, and the server's `ABORT` before `AUTH_OK`. |
| `c2s_hs` | `USER_AUTH`, and the client's `ABORT` before `AUTH_OK`. |
| `s2c_data` | Every server frame after `AUTH_OK`. |
| `c2s_data` | Every client frame after `AUTH_OK`. |

- Each direction has its own key. A frame sealed one way does not open the
  other way.
- Taking the session keys consumes `hs`, which is then wiped. The
  handshake keys are dropped when the session keys are made.
- Each new key starts its own frame and byte counts at zero.

Implemented in: `userland/nsh_crypto/src/schedule.rs`, `userland/nsh_client/src/client/hello.rs`, `userland/nsh_client/src/client/auth.rs`. Vectors: `userland/nsh_crypto_proofs/vectors/schedule.txt`.

## Labels

Every digest a hybrid key signs is `SHA-512(label || inner)`. Each label
names one purpose and ends in a zero byte, so a signature made for one
purpose cannot be presented for another.

| Label | Signed by | `inner` |
|---|---|---|
| `nsh/1 host-sig\0` | host keys | `TH(HOST_PROOF without its signatures)` |
| `nsh/1 user-sig\0` | user key | `TH(USER_AUTH without its signatures)` |
| `nsh/1 record-sig\0` | host keys | `SHA-512(record bytes before the footer signature)` |
| `nsh/1 record-ckpt\0` | host keys | `SHA-512(checkpoint bytes before its signature)` |
| `nsh/1 grant\0` | role authority | the grant's bytes up to and including `issuer` |
| `nsh/1 approve\0` | approver | `SHA-512("nsh/1 grant\0" || the grant's bytes up to and including issuer)` |
| `nsh/1 enrol\0` | enrolment authority | the certificate's bytes before its signature |
| `nsh/1 policy\0` | role authority | the role table's bytes before its signature |
| `nsh/1 revoke\0` | role authority | the revocation list's bytes before its signature |

Other fixed strings:

| String | Use |
|---|---|
| `nsh/1 transcript\0` | Start of the transcript chain. |
| `nsh/1 attest\0` | Attestation challenge ([nsh-v1-attest.md](nsh-v1-attest.md)). |
| `nsh/1 record\0` | Start of the record chain ([nsh-v1-record.md](nsh-v1-record.md)). |
| `nsh-enrol challenge\0` | Challenge for an enrolment document ([nsh-v1-attest.md](nsh-v1-attest.md)). |

Both halves of a hybrid signature sign the same 64-byte digest.

Implemented in: `userland/nsh_crypto/src/labels.rs`, `userland/nsh_crypto/src/challenge.rs`, `userland/nsh_client/src/record/walk.rs`, `tools/nsh-enrol/src/challenge.rs`. Vectors: `userland/nsh_crypto_proofs/vectors/schedule.txt` (all labels but `policy` and `revoke`).

## Sealed frames

```
len  = u32 LE (1 + body length + 16)
out  = len || ChaCha20-Poly1305(key, nonce, aad = len, type || body) || tag
```

| Nonce byte | Value |
|---|---|
| 0 to 3 | zero |
| 4 to 11 | `seq`, `u64` LE |

- `seq` is the number of frames sealed before this one in this direction
  under this key, from 0. It is never sent. Both sides count.
- The associated data is the four bytes of the length field.
- The length field must equal the number of sealed bytes after it, and
  that number must exceed 16. A mismatch is refused even when the tag is
  good.
- A frame with no type byte is never sealed and never opened.

### Limits per direction

| Limit | Value |
|---|---|
| Frames | 2^32: the last frame has `seq` = 4 294 967 295. |
| Bytes | 2^32 bytes of type and body, summed over the direction. |

- A frame that would pass either limit is refused and is not counted.
- A spent direction cannot seal even an `ABORT`. The session then closes
  without a frame.
- Nothing is counted until a frame seals or opens.

### Failure

- Once a frame fails to open, or arrives past a limit, the receiver opens
  no later frame. Nothing after a forged, dropped or reordered frame is
  trusted.
- The client does not answer a frame that does not open. It ends the
  session without a frame.

Implemented in: `userland/nsh_crypto/src/sealer.rs`, `userland/nsh_crypto/src/opener.rs`, `userland/nsh_crypto/src/count.rs`, `userland/nsh_client/src/client/send.rs`. Vectors: `userland/nsh_crypto_proofs/vectors/frames.txt` (key `00..1f`, `seq` 0, 1, 300 and 4 294 967 295).

## HOST_PROOF

The server's proof of the host. Sealed with `s2c_hs`.

| Field | Size |
|---|---|
| host key | `HybridPublic`, 1984 |
| enrolment certificate | `bytes16(1..4205)`, an `nsh-enrol/1` ([nsh-v1-attest.md](nsh-v1-attest.md)) |
| attestation document | `bytes32(1..1 114 112)`, `NONOSATT` version 5 ([nonosatt-v5.md](nonosatt-v5.md)) |
| registry entries | `bytes32(45..46 080)`, a whole number of 45-byte entries, 1 to 1024 of them |
| signature | `HybridSig`, 3373 |

- The certificate is parsed as `nsh-enrol/1` when `HOST_PROOF` is parsed.
- The document is parsed by the attestation checker.
- The entries are held to their 45-byte size here, and checked in step 7
  ([nsh-v1-attest.md](nsh-v1-attest.md)). They are sent in the order the
  kernel folds them: ascending pid.
- `MAX_ENROL` = 4205: the fixed fields, a 253-byte host name and eight
  boot applications.
- `MAX_DOC` = 1 048 576 + 65 536 = 1 114 112.

The host keys sign `SHA-512("nsh/1 host-sig\0" || TH(HOST_PROOF without its signatures))`.
The signed bytes are every field before `signature`. The client verifies
both halves before any other check of the host. A failure is answered
`ABORT(POLICY)`.

Implemented in: `userland/nsh_wire/src/host_proof.rs`, `userland/nsh_client/src/client/proof.rs`, `userland/nsh_attest/src/evidence_verify.rs`, `src/security/attest_registry/order.rs`.

## USER_AUTH

The client's proof of the user. Sealed with `c2s_hs`. It is sent only
after the client has accepted the host.

| Field | Size |
|---|---|
| user key | `HybridPublic`, 1984 |
| grant | `bytes16(1..11 756)`, an `nsh-grant/1` base grant ([nsh-v1-roles.md](nsh-v1-roles.md)) |
| signature | `HybridSig`, 3373 |

- The grant must be a base grant. An escalation grant here is refused
  (`PROTOCOL`).
- `MAX_GRANT` = 11 756: every list full and every name at its cap.
- The user key signs `SHA-512("nsh/1 user-sig\0" || TH(USER_AUTH without its signatures))`.
  The transcript at that point holds the accepted `HOST_PROOF`.
- Before it connects, the client checks that the grant parses, is a base
  grant, and names its own key as `subject`. Before it seals `USER_AUTH`,
  it reads the body back with the server's parser.

Implemented in: `userland/nsh_wire/src/user_auth.rs`, `userland/nsh_client/src/client/user.rs`, `userland/nsh_client/src/client/grant.rs`.

## AUTH_OK

Sealed with `s2c_hs`. It closes the handshake.

| Field | Size | Range |
|---|---|---|
| `session_id` | 16 | |
| `idle_s` | `u32` | 1 to 86 400 |
| `caps` | capability list | 0 to 41 names |

- `caps` names the capabilities the session capsule was given.
- `AUTH_OK` is added to the transcript. The session keys are derived under
  `TH(AUTH_OK)`.

- `idle_s` is the server's idle timeout. When no `INPUT`, `WINDOW` or
  `SIGNAL` has arrived for that long, the server stops the shell and the
  session ends with end reason 3.

Implemented in: `userland/nsh_server/src/server/admit.rs`, `userland/nsh_server/src/server/tick.rs`.

Implemented in: `userland/nsh_wire/src/auth_ok.rs`, `userland/nsh_client/src/client/auth.rs`.

## AUTH_REFUSED

Sealed with `s2c_hs`.

| Field | Size | Range |
|---|---|---|
| reason | `u16` | 16 to 24 |

Only login reasons may be carried ([nsh-v1-wire-session.md](nsh-v1-wire-session.md),
"Reason codes"). Both sides close after it.

Implemented in: `userland/nsh_wire/src/session_codes.rs`, `userland/nsh_wire/src/reason_sets.rs`, `userland/nsh_client/src/client/auth.rs`.

## ABORT

| Field | Size |
|---|---|
| reason | `u16` |

| When | Protection | Reasons allowed |
|---|---|---|
| Before keys: the server's answer to `CLIENT_HELLO` | plaintext | `VERSION` (2), `BUSY` (5), `UNATTESTED` (6), `UNENROLLED` (7) |
| After `SERVER_HELLO` | sealed | 1 to 5: `PROTOCOL`, `VERSION`, `POLICY`, `LIMIT`, `BUSY` |

- Before keys exist, only `VERSION`, `BUSY`, `UNATTESTED` and `UNENROLLED`
  can be told: nothing about the peer is known.
- The client sends `ABORT` only once keys exist. A refusal of
  `SERVER_HELLO` itself is not answered.
- A refused host is told `ABORT(POLICY)` and nothing more.
- `ABORT` never carries `OK` (0).
- After `ABORT`, the sender closes.

- `BUSY` (5) says the host cannot serve the connection now. It is sent
  in plaintext, before keys, by a host at its connection limit. Sealed,
  it is sent when the kernel cannot attest, when the host could not start
  an admitted session or escalation, and never for anything the client
  did.
- `LIMIT` (4) from the server: the handshake ran past the host's deadline.
- `UNATTESTED` (6) says the host has no TPM, so it has no host keys and no
  attestation, and serves no one. It is sent in plaintext only, before
  keys; nothing is faked in the attestation's place.
- `UNENROLLED` (7) says the host's machine does not hold the records a
  connection needs, or holds ones that do not open: its enrolment
  certificate, its role table and anchor, its revocation list, or a table
  or list older than one it already served under. Plaintext only, before
  keys. A stale revocation list is not this: the host admits the
  connection and refuses the login `REVOCATION_STALE`.

Implemented in: `userland/nsh_server/src/server/busy.rs`, `userland/nsh_server/src/why.rs`.

Implemented in: `userland/nsh_wire/src/session_codes.rs`, `userland/nsh_wire/src/reason_sets.rs`, `userland/nsh_client/src/client/end.rs`, `userland/nsh_client/src/client/hello.rs`.

## Constants

| Name | Value |
|---|---|
| `LEN_BYTES` | 4 |
| `TAG_LEN` | 16 |
| `MAX_DATA` | 16 384 |
| `HELLO_CAP` | 2048 |
| `AUTH_CAP` | 1 179 648 |
| `SESSION_CAP` | 16 385 |
| `CLIENT_HELLO_LEN` | 1258 |
| `SERVER_HELLO_LEN` | 1162 |
| `MAX_ENROL` | 4205 |
| `MAX_DOC` | 1 114 112 |
| `ENTRY_LEN` | 45 |
| `MAX_ENTRIES` | 1024 |
| `MAX_GRANT` | 11 756 |
| `SESSION_ID_LEN` | 16 |
| `MAX_IDLE_S` | 86 400 |
| `MAX_CAPS` | 41 |
| `MAX_CAP_NAME` | 32 |
| `MAX_SEALED_FRAMES` | 2^32 |
| `MAX_SEALED_BYTES` | 2^32 |

Implemented in: `userland/nsh_wire/src/limits.rs`, `userland/nsh_wire/src/lib.rs`, `userland/nsh_crypto/src/count.rs`.
