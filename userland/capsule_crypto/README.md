# capsule_crypto

## Role

`capsule_crypto` is the userland cryptographic operation capsule: the process
the kernel forwards most crypto syscalls to, so hashing, signature
verification, AEAD, X25519, HMAC and HKDF for capsules run here and not in
kernel code. It computes every primitive in its own address space and makes no
crypto syscall of its own. The capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md).

```text
capsule
    |
    | Crypto* syscall
    v
kernel client (src/security/crypto_capsule)
    |
    | crypto IPC, service:4102:crypto_pool
    v
crypto -- dispatch --> hash / verify / AEAD / X25519 / HMAC / HKDF
    |
    `-- reply on reply:4103:endpoint.4294967300
```

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x18`: IPC (`0x8`) and Memory (`0x10`), and nothing
  else. The kernel mirror (`src/security/crypto_capsule`) requests the same two
  bits. There is no optional mask.
- Service `service:4102:crypto_pool`, reply `reply:4103:endpoint.4294967300`.
- The feature `nonos-capsule-crypto` is in `microkernel-crypto`,
  `microkernel-terminal-only`, `microkernel-desktop-offline` (and the desktop
  sets built on it) and the smoke sets. Init spawns it in `spawn_after_ramfs`
  after the entropy capsule (`src/userspace/init/spawn_plan/core.rs`).
- Not every crypto syscall reaches here. The kernel serves `CryptoRandom` from
  its own generator, `CryptoKeccak256` with its own SHA-3 code and
  `CryptoMachineKey` from the TPM, and keeps the cryptography it needs for
  itself, such as capsule signature checks.

## Interface contract

The capsule runs one receive loop (`src/server/runner.rs`) and serves these
operations through `src/server/dispatch.rs`:

| Surface | Operations (op id) |
|---|---|
| hash | BLAKE3 (`1`), SHA3-256 (`2`), SHA-256 (`4`), SHA-512 (`5`), SHA-384 (`20`) |
| verify | Ed25519 (`6`), P-256 ECDSA (`18`), P-384 ECDSA (`19`), RSA (`21`) |
| AEAD | ChaCha20-Poly1305 seal/open (`10`/`11`), AES-256-GCM seal/open (`12`/`13`) |
| key agreement | X25519 public key (`14`), shared secret (`15`) |
| MAC and KDF | HMAC-SHA256 (`16`), HKDF-SHA256 (`17`) |
| health | healthcheck (`3`) |

An unknown op id is answered with `EINVAL`. Inputs are bounded: hashes to
`MAX_INPUT_BYTES` (65536), verify messages to `MAX_VERIFY_MESSAGE_BYTES` (1 MiB),
AEAD plaintext to `MAX_AEAD_PT_BYTES` (1 MiB) and AAD to 256 bytes. Sealing
refuses an all-zero nonce. SHA-384 and the P-256, P-384 and RSA verifiers have
no syscall in front of them; callers reach them over IPC only.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18` is the whole authority the capsule asks for: IPC
for `mk_ipc_recv`/`mk_ipc_send`, and Memory for the heap. `CAP_CRYPTO` is the
caller-facing gate the kernel client checks; the capsule itself does not hold
it, and it draws no randomness from the kernel, so it holds no Crypto. It holds
no driver, MMIO, IRQ, DMA, PIO, filesystem, network, admin or debug capability.
The kernel installs the mask from the verified manifest at spawn; the capsule
cannot widen it.

## Privacy and persistence

Request buffers are processed in capsule memory and replies are returned over
IPC. The capsule persists no plaintexts, digests, signatures or keys. Transient
operation buffers are wiped (`src/server/wipe.rs`). Long-lived secret storage
belongs to `capsule_keyring`, not here.

## Runtime lifecycle

Spawned once at init through the verified path, after the entropy capsule. It
registers `service:4102:crypto_pool`, then loops: `MkIpcRecv` for a request,
dispatch, `MkIpcSend` for the reply. It keeps no session between requests. On a
fatal setup failure it calls `MkExit`; the supervisor restarts it, and the
kernel client reports a dead or restarted capsule with an errno of its own.

## Failure model

Unsupported op, malformed payload, oversized input, verification failure or a
crypto backend failure return explicit protocol errors (`src/protocol/errno.rs`).
The kernel client maps them to errnos: an authentication failure is `EBADMSG`
for the AEAD open calls. A reply never carries a partial or fabricated result.

## Current implemented surface

The operations in the table above, served by the handlers under
`src/server/handlers/`. Nothing else: no key store, no persistent state, no
second code path.

## Wire format

Every message starts with the `NOCX` magic (`0x4E4F4358`) and protocol version
1, in a 20-byte header: `u32` magic, `u16` version, `u16` op, `u16` flags,
`u16` reserved, `u32` request id, `u32` payload length, then the payload. The
reply uses the same header and carries an `i32` status ahead of the result
bytes. The kernel side of the layout is a mirror in
`src/security/crypto_capsule/protocol.rs`, which reports drift as
`ProtocolMismatch`.

## State ownership

Stateless across requests. The capsule owns transient per-operation buffers
only and wipes them. `capsule_keyring` owns long-lived key material.

## Operating rules

- Compute every primitive in process; make no crypto syscall and hold no Crypto
  capability.
- Reject an all-zero AEAD nonce and any input past its per-op bound before
  touching the backend.
- Wipe operation buffers after use; never persist a plaintext, digest or key.

## Release target

0.9.2.

## Release evidence

The static gate `nonos-ci/run-static-checks.sh` runs
`scripts/check_mirror_caps.py` over the spawn mirror's capability word and
enforces that `handle_crypto_encrypt`/`decrypt` and the hash shims route
through this capsule's client rather than kernel-resident AEAD or hash code.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x18`.
- [ ] Protocol header matches the kernel mirror (no `ProtocolMismatch`).
- [ ] The static gate passes, including the mirror capability check.

## Explicit non-goals today

No filesystem key store, TLS stack, certificate database, hardware accelerator
driver, network protocol or persistent audit log lives in this capsule. It does
not generate or hold keys and does not serve `CryptoRandom`.

## Verification

- Build: `make nonos-mk-crypto`; sign: `make nonos-mk-crypto-sign`.
- Static gate: `bash nonos-ci/run-static-checks.sh`.
- The op ids, bounds and `NOCX` header in `src/protocol/types.rs` are the whole
  contract and are mirrored by `src/security/crypto_capsule`.
