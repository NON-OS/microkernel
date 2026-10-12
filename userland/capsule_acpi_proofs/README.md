# capsule_acpi_proofs

Host test crate for the pure parts of `userland/capsule_acpi`, mounted by
`#[path]` from the capsule's own sources so what is tested is what runs.

## What is under test

- `src/server/policy.rs`: which services each op of the ACPI service
  (port 4530) answers. The kernel's HELD line admits whole services to the
  endpoint; this table narrows each op to the services named for it, and
  answers no one for an op nobody is named for.
- `src/aei/rate.rs`: how many GPIO-signalled events (GPIO_EVENT, 13) one
  controller may run a second.
- `src/props/wire*.rs`: the PROPERTIES op's (12) wire, a device's _DSD
  tree as the driver holding it receives it.

## What the tests check

- Every op from 0 to 255, asked by each service the endpoint admits: the
  named ones are admitted, every other is refused (the capsule answers
  EPERM), as are a sender owning no service and pid 0.
- The table itself, op by op.
- The event rate: 64 a second admitted, the 65th refused and counted, a
  new second (or a clock that went back) starting afresh.
- The PROPERTIES wire: a tree with integers, strings, references,
  nested packages and nested nodes reads back as written; too many node
  levels, packages too deep, a count past 64, a name past 255 bytes or a
  tree past 16 KiB are not written; every truncation, a trailing byte, an
  unknown value kind, a count past 64 and packages too deep are not read.
- A named service that no one runs, or whose owner reads as pid 0,
  admits no one.
- Senders are threads (as MkIpcRecvFrom reports them): a worker thread
  of a named service is admitted for that service's ops and only those,
  whichever of its threads registered the service (`thread_tests.rs`).

Run with `cargo test --release` from this directory.
