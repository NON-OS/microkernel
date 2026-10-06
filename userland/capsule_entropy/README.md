# capsule_entropy

## Role

`capsule_entropy` is the userland entropy service. It reads the CPU's random
source on request and returns the bytes over IPC. Its main client is the
kernel: the generator behind the `CryptoRandom` syscall
(`src/security/entropy_capsule/fast.rs`) takes its seed from this capsule and
reseeds from it after each MiB of output.

```text
CryptoRandom caller
    |
    v
kernel fast generator (ChaCha20)
    |
    | entropy IPC, on first seed and every 1 MiB
    v
entropy -- source::fill --> RDRAND (x86_64) / crypto_random (aarch64)
```

The handbook page is [Kernel crypto](../../docs/handbook/kernel/crypto.md).

## Microkernel contract

The capsule is reached through IPC:

- `MkIpcRecv` receives service requests on `service:4100:entropy_pool`.
- `MkIpcSend` replies on `reply:4101:endpoint.4294967299`.
- `MkExit` terminates on fatal startup failure.
- The kernel mirror is `src/security/entropy_capsule`.

The kernel does not route each `CryptoRandom` call here. It serves those from
its own ChaCha20 generator, keyed from this capsule, and falls back to raw
hardware reads only when the generator was never seeded.

## Interface contract

| Operation | Behaviour |
|---|---|
| get random | returns up to 4096 bytes from the source, `EMSGSIZE` above that, `EIO` when the source fails |
| get stats | 32 bytes: requests, bytes served, reseed calls, source failures |
| reseed | checks the length, at most 256 bytes, and counts the call; it mixes nothing in |
| healthcheck | answers that the service is up |

The kernel client asks for no capability on get random. It requires `Entropy`
of the caller for get stats and healthcheck, and `Admin` for reseed.

## Authority

The manifest grants `IPC`, `Memory` and `Crypto` through
`CAPSULE_REQUIRED_CAPS = 0x38`. `Crypto` is what lets the aarch64 build call
`crypto_random`. It has no driver, MMIO, IRQ, DMA, PIO, filesystem, network,
admin, or debug authority.

## Sources

On x86_64 `source::fill` reads `RDRAND`, retrying up to 32 times per word, and
reports failure rather than substituting a weaker source. On aarch64 it calls
`crypto_random`, because only EL1 can tell whether `RNDR` exists. There is no
pool in the mixing sense: each request reads the source afresh.

## Privacy and persistence

The capsule keeps only counters. It does not persist samples or write
anything to disk. Request payloads and response bytes are transient IPC data.

## Failure model

Bad request sizes and source failure return protocol errors. The service does
not fabricate entropy to satisfy callers.

## Wire format

Every message starts with the `NOEN` magic and protocol version 1. Requests
carry an operation id, flags, a request id and a payload; get random's payload
is a little-endian `u32` length. Replies carry the same fields, an `i32` status
and the result bytes.

## Build and image

`Capsule.mk` defines the capsule for the shared capsule rules, so
`make nonos-mk-entropy` builds it and `make nonos-mk-entropy-sign` signs it.
The kernel embeds it when built with the `nonos-capsule-entropy` feature, which
`microkernel-entropy`, `microkernel-desktop-offline` (and the desktop sets
built on it), `microkernel-terminal-only` and the smoke-test sets turn on. The
kernel spawns it at init before the crypto capsule.

## Limits

- The x86_64 source is `RDRAND` only.
- `reseed` stores nothing.
- On aarch64 the source is `crypto_random`, which the kernel serves from the
  generator this capsule seeds. A first seed requested on aarch64 leads back
  into the lock the kernel's caller holds.

## Explicit non-goals today

No hardware driver, persistent seed file, TPM integration, remote entropy
source, cryptographic API, or key generation policy lives here.

## Verification

- Build: `make nonos-mk-entropy`
- Static gate: `bash nonos-ci/run-static-checks.sh`
