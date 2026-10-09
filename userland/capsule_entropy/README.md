# capsule_entropy

## Role

`capsule_entropy` is the userland entropy service: the process that reads the
CPU's random source on request and returns the bytes over IPC. Its main client
is the kernel. The generator behind the `CryptoRandom` syscall
(`src/security/entropy_capsule/fast.rs`) takes its seed from this capsule and
reseeds from it after each MiB of output. The capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md).

```text
CryptoRandom caller
    |
    v
kernel fast generator (ChaCha20)
    |
    | entropy IPC, on first seed and every 1 MiB
    v
entropy -- source::fill --> RDRAND (x86_64) / crypto_random (aarch64)
    |
    `-- reply on reply:4101:endpoint.4294967299
```

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x38`: IPC (`0x8`), Memory (`0x10`) and Crypto
  (`0x20`). The kernel mirror (`src/security/entropy_capsule`) requests the same
  three bits. There is no optional mask.
- Service `service:4100:entropy_pool`, reply `reply:4101:endpoint.4294967299`.
- The feature `nonos-capsule-entropy` is in `microkernel-entropy`,
  `microkernel-terminal-only`, `microkernel-desktop-offline` (and the desktop
  sets built on it) and the smoke sets. Init spawns it in `spawn_after_ramfs`
  before the crypto capsule (`src/userspace/init/spawn_plan/core.rs`).
- The kernel does not route each `CryptoRandom` call here. It serves those from
  its own ChaCha20 generator, keyed from this capsule, and falls back to raw
  hardware reads only when the generator was never seeded.

## Interface contract

One receive loop (`src/server/runner.rs`) serves four operations through
`src/server/dispatch.rs`:

| Operation (op id) | Behaviour |
|---|---|
| get random (`1`) | returns up to `MAX_RANDOM_BYTES` (4096) from the source, `EMSGSIZE` above that, `EIO` when the source fails |
| get stats (`2`) | 32 bytes: requests, bytes served, reseed calls, source failures |
| reseed (`3`) | checks the length, at most `MAX_RESEED_BYTES` (256), and counts the call; it mixes nothing in |
| healthcheck (`4`) | answers that the service is up |

The kernel client asks for no capability on get random. It requires `Entropy`
of the caller for get stats and healthcheck, and `Admin` for reseed.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x38` is the whole authority the capsule asks for: IPC
for `mk_ipc_recv`/`mk_ipc_send`, Memory for the heap, and Crypto so the aarch64
build can call `crypto_random`. The capsule is the entropy authority, so it does
not hold `Capability::Entropy`; callers carry that bit and reach the pool over
IPC. It holds no driver, MMIO, IRQ, DMA, PIO, filesystem, network, admin or
debug capability. The kernel installs the mask from the verified manifest at
spawn; the capsule cannot widen it.

## Privacy and persistence

The capsule keeps only counters. It persists no samples and writes nothing to
disk. Request payloads and response bytes are transient IPC data. There is no
pool in the mixing sense: each get-random request reads the source afresh
(`src/pool/fill.rs`).

## Runtime lifecycle

Spawned once at init through the verified path, before the crypto capsule. It
registers `service:4100:entropy_pool`, then loops: `MkIpcRecv` for a request,
dispatch, `MkIpcSend` for the reply. On a fatal startup failure it calls
`MkExit`; the supervisor restarts it.

## Failure model

Bad request sizes and source failure return protocol errors
(`src/protocol/errno.rs`). On x86_64 `source::fill` reads `RDRAND`, retrying up
to 32 times per word, and reports `EIO` rather than substituting a weaker
source. The service never fabricates entropy to satisfy a caller. On aarch64 a
first seed requested through `crypto_random` leads back into the lock the
kernel's caller holds, so that path is driven by the kernel, not a direct
userland reseed.

## Current implemented surface

The four operations above, served by the handlers under `src/server/handlers/`,
over the per-arch source in `src/pool/source/`. Nothing else: no second code
path, no persistent state.

## Wire format

Every message starts with the `NOEN` magic (`0x4E4F454E`) and protocol version
1, in a 20-byte header: `u32` magic, `u16` version, `u16` op, `u16` flags,
`u16` reserved, `u32` request id, `u32` payload length, then the payload. Get
random's payload is a little-endian `u32` length. The reply uses the same header
and carries an `i32` status in the first 4 bytes of its payload, then the result
bytes.

## State ownership

Stateless except for the four counters reported by get stats (requests, bytes
served, reseed calls, source failures). No heap state survives a request; no
files, no sessions.

## Operating rules

- Read the hardware source on every request; keep no mixed pool.
- Report `EIO` on source failure, never a weaker or fabricated source.
- `reseed` checks and counts only; it stores nothing.
- The x86_64 source is `RDRAND` only; the aarch64 source is `crypto_random`,
  which the kernel serves from the generator this capsule seeds.

## Release target

0.9.2.

## Release evidence

The static gate `nonos-ci/run-static-checks.sh` enforces that
`handle_crypto_random` routes through this capsule's client rather than a kernel
`fill_random` shim, and runs `scripts/check_mirror_caps.py` over the spawn
mirror's capability word.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x38`.
- [ ] Protocol header matches the kernel mirror.
- [ ] The static gate passes, including the mirror capability check and the
      `CryptoRandom` routing guard.

## Explicit non-goals today

No hardware driver, persistent seed file, TPM integration, remote entropy
source, cryptographic API or key generation policy lives here.

## Verification

- Build: `make nonos-mk-entropy`; sign: `make nonos-mk-entropy-sign`.
- Static gate: `bash nonos-ci/run-static-checks.sh`.
- The op ids, bounds and `NOEN` header in `src/protocol/types.rs` are the whole
  contract and are mirrored by `src/security/entropy_capsule`.
