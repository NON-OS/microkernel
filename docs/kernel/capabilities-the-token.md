# Capabilities: The token

Part of [Capabilities](capabilities.md).

## The token

Each process holds one `CapabilityToken`: the owning module id, the list of
capabilities, an optional expiry, a nonce, a 64 byte signature, and the
authority fields token id, subject pid, subject ASID, measurement, boot session
nonce, revocation epoch and delegation depth
(`src/capabilities/token/types/defs.rs:22-39`).

`new_token` mints a process token with no expiry, bound to the pid, its address
space id, the boot session nonce and the process's current revocation epoch, and
signs it (`src/process/caps.rs:38-59`). The mint fails closed: `new_token`
returns `None` until the boot session nonce exists, so no token is bound to a
zero nonce (`src/process/caps.rs:24-26`).

The token's `signature` field is a message authentication code, not a public key
signature: `mac64` over the 128 byte `token_material`, two keyed BLAKE3 hashes,
one of the material and one of the material followed by `CAP2`
(`src/capabilities/token/material.rs:41-52`). Only the kernel holds the key, so
only the kernel can make or check one. `init_token_signing_key` draws the 32 key
bytes at boot with `get_bytes_secure` and halts if the draw reports a failure
(`src/kernel_core/init/platform/token_signing_key.rs:19-31`). The draw never
does: `get_bytes_secure` always returns success
(`src/crypto/random_api/basic.rs:38-42`), so on a machine with no hardware
random source the key comes from the cycle counter, as [Randomness and
cryptography](../security/randomness-and-cryptography.md#when-a-source-is-missing)
explains. `set_signing_key` refuses a second key
(`src/capabilities/token/signing_key.rs:21-33`).

The `[token]` and `[mac]` tables of `abi/caps.toml` describe a different design,
a SHA3-256 MAC under the context `NONOS_CAP_V1` (`abi/caps.toml:58-70`). Nothing
under `src/` uses that context; the kernel token is the keyed BLAKE3 one above.
