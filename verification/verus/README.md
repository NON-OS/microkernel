# NONOS Verus proofs

Machine-checked theorems about the NONOS security algebra, verified by
[Verus](https://github.com/verus-lang/verus) (SMT deductive verification of
Rust). These are checked by the verifier, not executed at kernel time.

## What is proven

`src/capabilities.rs` restates `src/capabilities/bits.rs` by hand (a right is a
single power-of-two bit, a token is the OR of its rights) and proves, over all
`u64` values:

- **revoke is monotonic**: revoking never adds authority.
- **revoke drops the right**: the revoked bit is gone.
- **attenuation confines**: an attenuated token never gains a right the parent
  lacked (object-capability confinement).
- **grant preserves and adds**: granting keeps existing rights and adds exactly
  the requested one.
- **empty token grants nothing**: authority cannot be conjured from nothing.

`src/page_permissions.rs` mirrors the page-table permission bits used by
`PagePermissions::to_pte` and proves present-bit, writable-bit, user-bit,
executable/NX, non-WX, and permission-subset monotonicity properties.

`src/ipc_lengths.rs` mirrors the IPC `MAX_MESSAGE_SIZE` gate and proves that
zero-length and oversized messages are rejected while accepted lengths are
bounded by the shared `1..=1048576` rule.

`src/stark_attestation.rs` models a trailer reader: a length prefix capped at
the remaining bytes never over-reserves, the cursor never leaves the buffer,
and acceptance is the conjunction of the gate's checks.

The crate includes no kernel file. Each spec function is written out to match
the kernel's, so the theorems hold for the restatement; a change to the kernel
does not reach them, and a drift between the two is not caught here. The
proofs page says the same: [docs/handbook/verification/proofs.md](../../docs/handbook/verification/proofs.md).

## Verify

Install the Verus toolchain (`0.2026.06.28.1847ab3` in the `verus` job of `.github/workflows/verify.yml`), then:

```sh
verus --crate-type=lib src/lib.rs
```

A clean run prints `verification results:: N verified, 0 errors`.
