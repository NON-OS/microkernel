# Wire Formats

Three self-tagged binary schemas make up the trust chain. All integers
are big endian. The certificate and manifest are a signed region
followed by a signature trailer: the signer covers exactly the
region, byte for byte, and the count byte that opens the trailer is
the first unsigned byte. The policy stands alone, unsigned, for the
reason given below. Length-prefixed strings carry a one-byte length; nothing is
null-terminated; decoders refuse anything that does not consume
exactly its declared bytes.

## Schema 1 — trust-anchor policy

The root of the chain: the anchor's keys and the revocation state.
It carries no signature, deliberately. A signature on the root would
only chain to itself; the policy's integrity comes from being baked,
into the kernel image, the bootloader's verification, and the
keystore ledger, so replacing it means replacing artifacts whose
hashes are published and independently rebuilt.

| Field | Size | Notes |
|-------|------|-------|
| schema_version | u16 | 1 |
| epoch | u64 | monotonically increasing; certificates bind to it |
| key_count | u8 | at most 4 |
| keys[] | | per key: alg u8, pubkey_len u16, pubkey, valid_from_ms u64, valid_until_ms u64 |
| revoked_cert_serial_count | u16 | at most 256, then u64 serials |
| revoked_nonos_id_count | u8 | at most 64, then 32-byte ids |
| revoked_publisher_key_id_count | u16 | at most 256, then 16-byte key ids |
| flags | u32 | reserved, zero today |

## Schema 2 — NONOS-ID certificate

A publisher's identity, signed by the trust anchor. The certificate is
what bounds a publisher: which namespaces it may sign for and the
capability ceiling nothing it signs may exceed.

| Field | Size | Notes |
|-------|------|-------|
| schema_version | u16 | 2 |
| cert_serial | u64 | revocable individually |
| nonos_id | 32 | BLAKE3 of handle, domain, recovery |
| glob_count | u8 | at most 8 namespace globs, each len u8 + bytes (≤ 96) |
| allowed_caps_ceiling | u64 | the hard upper bound for every manifest under this cert |
| metadata_len + metadata | u8 + bytes | ≤ 256, informational |
| valid_from_ms, valid_until_ms | u64 ×2 | validity window |
| trust_anchor_epoch | u64 | must match the policy's epoch |
| publisher_key_count | u8 | at most 4, at most 2 per algorithm |
| publisher_keys[] | | per key: alg u8, key_id 16, pubkey_len u16, pubkey |
| signature trailer | | trust-anchor signatures |

## Schema 3 — capsule manifest

One capsule binary, bound to one certificate, signed by the publisher.

| Field | Size | Notes |
|-------|------|-------|
| schema_version | u16 | 3 |
| nonos_id_cert_id | 32 | BLAKE3 of the exact certificate bytes |
| namespace_len + namespace | u8 + bytes | ≤ 96, must match a cert glob |
| version major, minor, patch | u32 ×3 | |
| target_triple_len + triple | u8 + bytes | ≤ 64 |
| payload_hash | 32 | BLAKE3 of the exact capsule ELF |
| required_caps | u64 | granted at spawn |
| optional_caps | u64 | requestable |
| endpoint_count | u8 | at most 16 |
| endpoints[] | | per endpoint: kind u8, port u32, name_len u8 + name (≤ 48) |
| signature trailer | | publisher signatures |

`required_caps | optional_caps` must fit under the certificate's
ceiling; the verifier refuses a manifest that asks for more than its
publisher was ever allowed to grant.

## The signature trailer

Identical shape everywhere: a count byte, then per signature the
algorithm id, the 16-byte key id of the signing key, a u16 signature
length, and the signature bytes. Hybrid means both algorithms must be
present and both must verify:

| Alg id | Scheme | Public key | Signature |
|--------|--------|------------|-----------|
| 0x01 | Ed25519 | 32 | 64 |
| 0x02 | ML-DSA-44 | 1312 | 2420 |
| 0x03 | ML-DSA-65 | 1952 | 3309 |
| 0x04 | ML-DSA-87 | 2592 | 4627 |

The shipping chain signs with 0x01 and 0x03; 0x02 and 0x04 are
allocated so a strength migration is an id, not a schema change.

A verifier requires one valid signature from each required algorithm;
extra signatures are permitted for rotation overlap, at most 4 total.
The key id is BLAKE3 over the algorithm byte followed by the public
key, truncated to 16 bytes, so a trailer names its keys without
carrying them and an identical key under two algorithms still gets
two distinct ids.
