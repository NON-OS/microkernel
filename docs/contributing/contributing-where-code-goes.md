# Contributing to NONOS: Where code goes

Part of [Contributing to NONOS](README.md).

## Where code goes

| Change | Where it goes |
|---|---|
| a kernel mechanism | `src/` |
| a driver | `userland/capsule_driver_<name>/`, with its proof crate in `userland/<name>_proofs/` |
| a service or a program | `userland/capsule_<name>/` |
| a host test of shipping code | `userland/<name>_proofs/` |
| a theorem | `verification/lean/`, or `verification/extraction/` for extracted code |
| an ABI number or [capability](../overview/glossary-boot-root-record.md#capability) bit | `abi/` |

The kernel does not host drivers. `src/drivers/` may hold only `pci`, `security`
and `virtio_rng`, and the static checks fail on anything else
(`unexpected_drivers`, `nonos-ci/run-static-checks.sh:179-190`). `src/services/`
may hold only `caps`, `lifecycle` and `registry` (`unexpected_services`,
`nonos-ci/run-static-checks.sh:363-371`). Every other service runs as a
[capsule](../overview/glossary-boot-root-record.md#capsule) in ring 3.

Every `userland/capsule_*` directory needs a non-empty `README.md` that states
its contract; for one without it, `fail_with` marks the static checks failed
(`nonos-ci/run-static-checks.sh:352-360`). The `README.md` of a [driver
capsule](../overview/glossary-dma-pool.md#driver-capsule) must also carry
sixteen named sections, an ASCII diagram in a `text` block, its
`CAPSULE_REQUIRED_CAPS` mask and at least one of the broker calls it makes
(`nonos-ci/run-static-checks.sh:198-244`). All 27 driver capsules meet that rule
at this commit. [Writing a driver](../drivers/writing-a-driver.md) walks through
one.

Every driver capsule needs a proof crate named after it, apart from the two
names that `ALIASES` maps to another crate
(`scripts/check_driver_proofs.py:31-47`). At this commit
`check_driver_proofs.py` reports 27 of 27 drivers with a proof crate.

## Pages in this section

- [Code style](code-style.md): formatting, the file header, comments, file size,
  and the panic, lint and dependency rules.
- [Tests and proofs](tests-and-proofs.md): proof crates, static checks, Kani,
  Lean, fuzzing, how to run each, and the state of the checks at this commit.
- [Review](review.md): what CI runs on a pull request, who is asked to review
  it, and what reviewers look for.
- [Commits](commits.md): the subject line, the body, and what a commit leaves
  out.

## See also

- [Build](../build/README.md)
- [The Nix flake](../build/nix-flake.md)
- [Make targets](../build/make-targets.md)
- [CI](../build/ci.md)
- [Architectures](../architectures/README.md)
- [The capsule model](../userland/README.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
