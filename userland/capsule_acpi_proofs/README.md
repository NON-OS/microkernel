# capsule_acpi_proofs

Host test crate for the pure parts of `userland/capsule_acpi`, mounted by
`#[path]` from the capsule's own sources so what is tested is what runs.

## What is under test

- `src/server/policy.rs`: which services each op of the ACPI service
  (port 4530) answers. The kernel's HELD line admits whole services to the
  endpoint; this table narrows each op to the services named for it, and
  answers no one for an op nobody is named for.

## What the tests check

- Every op from 0 to 255, asked by each service the endpoint admits: the
  named ones are admitted, every other is refused (the capsule answers
  EPERM), as are a sender owning no service and pid 0.
- The table itself, op by op.
- A named service that no one runs, or whose owner reads as pid 0,
  admits no one.
- Senders are threads (as MkIpcRecvFrom reports them): a worker thread
  of a named service is admitted for that service's ops and only those,
  whichever of its threads registered the service (`thread_tests.rs`).

Run with `cargo test --release` from this directory.
