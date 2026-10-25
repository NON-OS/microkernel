# Randomness and cryptography: the services and calls

Part of [Randomness and cryptography](randomness-and-cryptography.md).

## The two services

| | `entropy_pool` | `crypto_pool` |
|---|---|---|
| Ports | service 4100, reply 4101 | service 4102, reply 4103 |
| The capsule's own bits | `IPC`, `Memory`, `Crypto`: 0x38 | `IPC`, `Memory`: 0x18 |
| Wire header | magic `NOEN`, version 1 | magic `NOCX`, version 1 |
| Started | by `spawn_after_ramfs`, after the keyring | right after `entropy_pool` |

The ports and bits are in each `Capsule.mk` (`CAPSULE_SERVICE_ENDPOINT` and `CAPSULE_REQUIRED_CAPS`, `userland/capsule_entropy/Capsule.mk:14-17`, `userland/capsule_crypto/Capsule.mk:13-16`), and the kernel's spawn sites ask for the same bits (`requested_caps`, `src/security/entropy_capsule/spawn.rs:54`, `src/security/crypto_capsule/spawn.rs:54`). `spawn_after_ramfs` starts the keyring, `entropy_pool`, `crypto_pool` and the policy capsule in that order (`src/userspace/init/spawn_plan/core.rs:22-27`). Both names are reserved, so no capsule can register them at run time (`RESERVED_NAMES`, `src/services/registry/reserved.rs:19-20`).

`crypto_pool` answers 18 operations: BLAKE3, SHA-256, SHA-384, SHA-512 and SHA3-256 hashes; Ed25519, P-256 ECDSA, P-384 ECDSA and RSA verification; ChaCha20-Poly1305 and AES-256-GCM seal and open; the X25519 public key and shared secret; HMAC-SHA256 and HKDF-SHA256; and a health check (`dispatch`, `userland/capsule_crypto/src/server/dispatch.rs:28-50`). It computes them in its own process with the `blake3`, `sha1`, `sha2`, `sha3`, `chacha20poly1305`, `aes-gcm`, `ed25519-dalek`, `x25519-dalek`, `p256`, `p384` and `rsa` crates (`userland/capsule_crypto/Cargo.toml:24-36`), and holds no `Crypto` because it makes no crypto system call (`CAPSULE_REQUIRED_CAPS`, `userland/capsule_crypto/Capsule.mk:1-16`). Sealing refuses an all-zero nonce with `EINVAL` (`nonce_is_degenerate`, `userland/capsule_crypto/src/server/handlers/aead_frame/parse.rs:29-35`), and the request buffer is wiped after each reply (`wipe`, `userland/capsule_crypto/src/server/runner.rs:42`).

RSA is verify only. Scheme 0 is PKCS#1 v1.5 with SHA-256, SHA-384, SHA-512 or SHA-1, scheme 1 is PSS with the three SHA-2 widths, and scheme 2 is PKCS#1 v1.5 over a bare digest with no DigestInfo, which the Anyone directory signatures use (`verify`, `userland/capsule_crypto/src/server/handlers/rsa_scheme.rs:54-70`). SHA-1 is offered only under schemes 0 and 2, and only to check signatures someone else already made (`digest_len`, `userland/capsule_crypto/src/server/handlers/rsa_scheme.rs:33-41`).

### Who may call them

Through the system calls below, a caller needs `Crypto`, and the kernel's client checks `CAP_CRYPTO` again before each round trip (`gate_hash`, `src/security/crypto_capsule/capability.rs:22-31`). The kernel's client for `entropy_pool` asks `Entropy` for its statistics and health check and `Admin` for reseed (`gate_read`, `src/security/entropy_capsule/capability.rs:23-44`), but no kernel code calls those three functions in this release.

Capsules can also call both services directly. Each service [endpoint](../overview/glossary-dma-pool.md#endpoint) is registered with `IPC` as the only bit a sender needs, plus `Network` for the network services (`required_caps`, `src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:104-105`), and neither name is a [held endpoint](../overview/glossary-hardware-broker.md#held-endpoint) in `HELD` (`src/services/registry/held_table.rs:20-36`). A capsule holding `IPC` can therefore send either one a request with `MkIpcCall`, and the kernel routes the reply back to it (`redirect_reply`, `src/syscall/microkernel/ipc/send.rs:141-146`). The one exception is a capsule held to a [peer list](../overview/glossary-peer-list.md#peer-list), and in this release only `shield_prover` is, which may reach `shield.core` alone (`PEERS`, `src/services/registry/peers.rs:27-31`). The services check no capability themselves, so the `Crypto`, `Entropy` and `Admin` checks above do not apply to such a call. The TLS library uses this path for P-256, P-384 and RSA certificate checks (`crypto_status`, `userland/nonos_tls/src/verify_p256.rs:22`, `userland/nonos_tls/src/verify_p384.rs:22`, `OP_RSA_VERIFY` in `userland/nonos_tls/src/verify_rsa.rs:21`), and the Anyone service for its directory signatures (`OP_RSA_VERIFY`, `userland/capsule_net_anon/src/directory/verify/frame.rs:25`).

## The crypto system calls

Twelve calls make up the crypto family, each named by its [syscall tag](../overview/glossary-stark-proof.md#syscall-tag) (`ENTRIES`, `src/syscall/abi/registry/crypto.rs:22-35`). Every one requires `Crypto` (`can_crypto`, `src/syscall/contract/cap_table/crypto.rs:20-33`).

| Tag | Call | Served by | Limits |
|---|---|---|---|
| `CRND` | `CryptoRandom` | kernel, the fast generator | 1 to 4096 bytes |
| `CKEC` | `CryptoKeccak256` | kernel, its own SHA-3 code | 1 byte to 1 MiB in, 32 bytes out |
| `CMKY` | `CryptoMachineKey` | kernel, the TPM | a label of 1 to 64 bytes whose first byte is not 0 |
| `CHSH` | `CryptoHash` | `crypto_pool` | algorithm 0 BLAKE3, 1 SHA-256, 2 SHA-512, 3 SHA3-256; 1 byte to 64 KiB |
| `CENC`, `CDEC` | `CryptoEncrypt`, `CryptoDecrypt` | `crypto_pool` | algorithm 0 ChaCha20-Poly1305, 1 AES-256-GCM; 32-byte key, 12-byte nonce, up to 1 MiB at the call, 16-byte tag |
| `CEAD`, `CDAD` | `CryptoEncryptAad`, `CryptoDecryptAad` | `crypto_pool` | as above, with up to 256 bytes of associated data |
| `CXPK`, `CXSH` | `CryptoX25519Public`, `CryptoX25519Shared` | `crypto_pool` | 32-byte keys |
| `CHMC` | `CryptoHmacSha256` | `crypto_pool` | a key of up to 256 bytes, up to 64 KiB of data |
| `CHKF` | `CryptoHkdfSha256` | `crypto_pool` | a frame of up to 784 bytes, 1 to 512 bytes out |

The kernel keeps three: `handle_crypto_keccak256` hashes with the kernel's own `keccak256` (`src/syscall/dispatch/crypto/keccak/handler.rs:32-42`), and `handle_machine_key` refuses a label that starts with a zero byte, which the kernel keeps for its own keys (`is_user_label`, `src/syscall/dispatch/crypto/machine_key.rs:43-50`). The other nine go to `crypto_pool` (`dispatch_crypto`, `src/syscall/dispatch/router/crypto.rs:27-50`). `CryptoHash` accepts 1 MiB at the call (`handle_crypto_hash`, `src/syscall/dispatch/crypto/hash/handler.rs:26-32`), but the client refuses more than `MAX_INPUT_BYTES`, 65536, with `EMSGSIZE` (`src/security/crypto_capsule/client/hash_op.rs:33-36`). The AEAD sizes are `ALGO_CHACHA20_POLY1305` through `TAG_LEN` (`src/syscall/dispatch/crypto/aead/constants.rs:17-20`) and `MAX_AAD` (`src/syscall/dispatch/crypto/aead/frame.rs:20-30`); the HMAC and HKDF sizes are `MAX_KEY` and `MAX_DATA` (`src/syscall/dispatch/crypto/primitives/hmac.rs:25-26`) and `MAX_FRAME` and `MAX_OUT` (`src/syscall/dispatch/crypto/primitives/hkdf.rs:24-25`).

The 1 MiB AEAD limit is not quite reachable. The kernel carries each request to `crypto_pool` as one IPC message, and `IpcMessage::new` refuses a payload over `MAX_MESSAGE_SIZE`, 1 MiB (`src/ipc/nonos_channel/message.rs:35-38`, `src/ipc/nonos_channel/limits.rs:26`). The request adds a 20-byte header and 48 bytes of key, nonce and length to the text and the associated data, so a call whose text and associated data pass 1 MiB less 68 bytes fails in transport with `EIO`, 5 (`TransportFailure`, `src/syscall/dispatch/crypto/error.rs:41`).

`map_capsule_error` turns the capsule's failures into errnos: a tag that does not authenticate is `EBADMSG`, 74, an oversized request `EMSGSIZE`, 90, a capsule that is not running `ENODEV`, 19, and one restarted during the call `ESTALE`, 116 (`src/syscall/dispatch/crypto/error.rs:26-43`). [Syscalls](../abi/syscalls.md) has the numbers and [Errors](../abi/errors.md) the errno table.
