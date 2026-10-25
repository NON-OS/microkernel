# Manifests and capabilities: The binary format

Part of [Manifests and capabilities](manifests-and-capabilities.md).

## The binary format

The manifest is schema version 3
(`src/security/capsule_manifest/schema/constants.rs:17`,
`MANIFEST_SCHEMA_VERSION`). Multi-byte integers are big-endian. The fields, in
wire order, as `decode` reads them
(`src/security/capsule_manifest/decode/mod.rs:25-47`, `decode`):

| Field | Size | Rule |
|---|---|---|
| schema version | 2 bytes | must be 3 |
| certificate id | 32 bytes | BLAKE3 of the [NONOS ID certificate](../overview/glossary-linux-personality.md#nonos-id-certificate) |
| namespace | 1 length byte, then 1 to 96 bytes | UTF-8 |
| version | 3 × 4 bytes | major, minor, patch |
| target triple | 1 length byte, then 1 to 64 bytes | UTF-8 |
| payload hash | 32 bytes | BLAKE3 of the whole ELF |
| required caps | 8 bytes | |
| optional caps | 8 bytes | no bit may also be required |
| endpoint count | 1 byte | at most 16 |
| each endpoint | kind byte, 4-byte port, name length byte, name | kind 1 is a service, 2 a reply; name 1 to 48 bytes; no two with the same kind and name |
| signature count | 1 byte | 1 to 4 |
| each signature | algorithm byte, 16-byte key id, 2-byte length, signature | algorithm 1 is Ed25519 (64 bytes), 3 is ML-DSA-65 (3309 bytes); the length must match the algorithm |

The decoder refuses a required and optional set that overlap
(`src/security/capsule_manifest/decode/header.rs:45-49`, `OverlappingCaps`), a
duplicate endpoint (`src/security/capsule_manifest/decode/endpoints.rs:44-48`,
`DuplicateEndpoint`), and any byte after the last signature
(`src/security/capsule_manifest/decode/mod.rs:30-32`, `TrailingBytes`). The
publisher signs every byte before the signature count
(`src/security/capsule_manifest/verify/signed_region.rs:20-41`, `compute`).
Signature sizes come from `src/crypto/asymmetric/alg_id/lengths.rs:19-29`
(`MLDSA65_SIG_BYTES`), and the algorithm byte from
`src/crypto/asymmetric/alg_id/types.rs:39-47` (`from_u8`). The decoder also
knows ML-DSA-44 (2) and ML-DSA-87 (4), but the policy below asks only for
Ed25519 and ML-DSA-65.

The schema in `abi/capsule_manifest.schema.json` names the same fields as a JSON
object. Some of its descriptions are older than the decoder; where they differ,
the decoder is right.
