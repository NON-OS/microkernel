# Note format and cipher

What a note is, how it is sealed to a recipient, and what a sender or an observer can do with one.
What is checked and what is not is in [20-security-status.md](../../../docs/20-security-status.md).

## What this owns

What a note is, and how one travels from a sender to a recipient. The plaintext is four fields
in a fixed 80-byte encoding (`plaintext.rs`): value, asset id, blinding and spend key. A value is
at most `MAX_VALUE`, which is p - 2 over the Goldilocks prime.

The note the pool carries, on the production pool as on every pool before it, is the X-Wing blob of
`xwing.rs`, 1,186 bytes, version `0x01`:

| Bytes | Content |
|---|---|
| 0 | version, `0x01` |
| 1 | view tag, the first byte of SHA3-256 over `NOX-NOTE-VIEW-TAG-v1` followed by the shared secret |
| 2 to 1,121 | the X-Wing ciphertext: ML-KEM-768, then X25519 |
| 1,122 to 1,185 | ChaCha20-Poly1305 of the 48-byte opening. The key is SHA3-256 over `NOX-NOTE-SEAL-KEY-v1` followed by the shared secret, the nonce is twelve zero bytes, and the associated data is bytes 0 and 1 followed by the leaf commitment |

The note commitment (`commit.rs`) is the nested commitment the pool computes and the circuit
proves, recomputed here and never taken on trust.

A second record format stays in the tree and in its tests: one X25519 key agreement per note, a
view tag, and ChaCha20-Poly1305, 129 bytes (`wire.rs`, `cipher/`). Neither pool carries
it, and the wallet does not produce it.

## What it trusts

X-Wing (`x-wing`, pinned by exact version) for the hybrid key encapsulation: a note stays sealed
while either ML-KEM-768 or X25519 holds. There is no classical-only version.

SHA3-256 for the view tag and the seal key, and ChaCha20-Poly1305 for the opening. The fixed nonce
is safe because every key seals a single message.

The pool hash, for recomputing the commitment that becomes the associated data.

## What it hands its neighbours

To `discovery`: an outcome for each blob, which is a tag miss, a blob that did not authenticate, or
an opening. They stay separate so a scan can count them.

To `wallet`: sealing, for the two outputs a spend creates, one to the payee and one to this
wallet. The payee blob is sealed to the encapsulation key in their `nox1` address.

## What an attacker who controls a neighbour can do

**A sender.** They choose the value, the blinding and the spend key in the note they seal to this wallet.
They can write a note this wallet cannot spend, and `discovery` refuses it: an opening must
recompute to the leaf the pool stored and carry a spend key this account derives. `open_xwing`
also discards a value above `MAX_VALUE` and an asset id that is not canonical.

**A sender who wants to prove later what they sent.** They can: they chose the opening, and an
opening they reveal recomputes to the leaf the pool stored, which anybody can check against the
chain. That is true of any sender in any such design. It reveals the spend key the note was paid
to, which is already in the address they paid, and nothing about any other note of the recipient.

**Anybody who moves a blob.** The leaf commitment is associated data, so a blob moved onto another
leaf fails to authenticate and never opens to something wrong.

**An observer counting bytes.** Every blob is 1,186 bytes, so a value, a recipient or whether a
note is change is not visible in its length. The number of outputs, two per spend, is public
anyway.

**An adversary with a quantum computer later.** A blob recorded today stays sealed while ML-KEM-768
holds. That is the reason the format is hybrid and not X25519 alone.
