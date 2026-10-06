# capsule_crypto

## Role

`capsule_crypto` is the userland cryptographic operation capsule. The kernel
forwards most crypto syscalls to it over IPC, so hashing, AEAD, X25519, HMAC
and HKDF for capsules run here and not in kernel code.

```text
capsule
    |
    | Crypto* syscall
    v
kernel client (src/security/crypto_capsule)
    |
    | crypto IPC
    v
crypto -- dispatch --> hash / verify / AEAD / X25519 / HMAC / HKDF
```

The handbook page is [Kernel crypto](../../docs/handbook/kernel/crypto.md).

## Microkernel contract

The capsule uses IPC and memory only:

- `MkIpcRecv` receives requests on `service:4102:crypto_pool`.
- `MkIpcSend` replies on `reply:4103:endpoint.4294967300`.
- `MkExit` terminates on fatal setup failure.
- The kernel mirror is `src/security/crypto_capsule`.

Not every crypto syscall reaches this capsule. The kernel serves
`CryptoRandom` from its own generator, `CryptoKeccak256` with its own SHA-3
code and `CryptoMachineKey` from the TPM. The kernel also keeps the
cryptography it needs for itself, such as capsule signature checks.

## Interface contract

`dispatch` serves these operations:

| Surface | Operations |
|---|---|
| hash | BLAKE3, SHA-256, SHA-384, SHA-512, SHA3-256 |
| verify | Ed25519, P-256 ECDSA, P-384 ECDSA, RSA |
| AEAD | ChaCha20-Poly1305 and AES-256-GCM, seal and open |
| key agreement | X25519 public key and shared secret |
| MAC and KDF | HMAC-SHA256, HKDF-SHA256 |
| health | healthcheck |

An unknown operation is answered with `EINVAL`. SHA-384 and the P-256,
P-384 and RSA verifiers have no syscall in front of them. Sealing refuses an
all-zero nonce.

## Authority

The manifest grants `IPC` and `Memory` through
`CAPSULE_REQUIRED_CAPS = 0x18`. It hashes, verifies and seals in its own code and draws no
randomness from the kernel, so it holds no Crypto. It has no driver, MMIO, IRQ, DMA, PIO,
filesystem, network, admin, or debug authority.

## Privacy and persistence

Request buffers are processed in capsule memory and replies are returned over
IPC. The capsule does not persist plaintexts, digests, signatures, or keys.
Long-lived secret storage belongs to `capsule_keyring`, not here.

## Failure model

Unsupported operation, malformed payload, oversized input, verification
failure, or crypto backend failure return explicit protocol errors. The kernel
client maps them to errnos: an authentication failure is `EBADMSG` for the AEAD
calls, and a dead or restarted capsule has an errno of its own.

## Wire format

Every message starts with the `NOCX` magic. Requests carry operation id, flags,
a request id and payload bytes. Replies carry the same three fields, a status
and the result bytes. The kernel side of the layout
is a mirror in `src/security/crypto_capsule/protocol.rs`, which reports drift
as `ProtocolMismatch`.

## State ownership

The capsule owns transient operation buffers only. `capsule_keyring` owns
long-lived key material.

## Build and image

`Capsule.mk` defines the capsule for the shared capsule rules, so
`make nonos-mk-crypto` builds it and `make nonos-mk-crypto-sign` signs it. The
kernel embeds it when built with the `nonos-capsule-crypto` feature, which
`microkernel-crypto`, `microkernel-desktop-offline` (and the desktop sets built
on it), `microkernel-terminal-only` and the smoke-test sets turn on. The kernel
spawns it at init after the entropy capsule.

## Explicit non-goals today

No filesystem key store, TLS stack, certificate database, hardware accelerator
driver, network protocol, or persistent audit log lives in this capsule.

## Verification

- Build: `make nonos-mk-crypto`
- Static gate: `bash nonos-ci/run-static-checks.sh`, which includes
  `scripts/check_mirror_caps.py` for the spawn mirror's capability word.
