# Checking the security claims yourself: Every capability and control is enforced, continued

Part of [Checking the security claims yourself](checking-the-claims.md).

## Every capability and control is enforced, continued

It printed `cap-parity: 36 capabilities, kernel and 2 mirrors agree` and exited
0. A difference prints `cap-parity: the capability tables disagree`, one line
per mismatch, and exits 1.

`scripts/cap_audit.py --strict` holds each capsule's mask to what its code can
be shown to need. It fails a capsule whose own code makes a call its mask has no
bit for, a capsule that requires `Debug` outside `DEBUG_REQUIRED`, and a
crates.io tool whose manifest lacks part of the tool sandbox or that is spawned
without the bits the [file store](../overview/glossary-dma-pool.md#file-store)
asks for; the `attack` and `toolkit` capsules are exempt (`strict_failures`,
`scripts/cap_audit.py:310-335`). It can miss a use reached through a function
pointer, a computed system call number or a kernel check not listed in
`cap_audit_kernel`, so a bit with no evidence is one to read, not one to drop
(`scripts/cap_audit.py:40-49`).

```sh
python3 scripts/cap_audit.py --strict
```

It ended with `cap-audit: 107 capsules, 8 granted bits with no evidence, 1
capsules missing a bit a call needs, 0 strict failures (attack, toolkit exempt)`
and exited 0. The one capsule missing a bit is `toolkit`, which is exempt.

`make nonos-mk-check-caps` and `make nonos-mk-static` run `check_cap_parity`
before anything is built (`mk/40-run.mk:345-356`). The flake check `static-abi`
runs it, `check_unenforced` and the next two tools in one list after
`check_prebuilt` (`tools/nix/checks.nix:221-232`). `cap_audit.py --strict` is
one gate of `run-static-checks.sh` (`cap_audit_out`,
`nonos-ci/run-static-checks.sh:4825-4830`), which the flake check `static-tree`
runs.


## The assumption register is complete

[`verification/ASSUMPTIONS.md`](../../verification/ASSUMPTIONS.md) has one row
per thing NONOS trusts rather than proves. `tools/nonos-assumptions` finds what
it can in the tree: every third-party crate linked into the kernel or the
bootloader, the in-tree cryptography, the hardware features the kernel relies
on, the toolchains, and any Lean axiom or Verus assumption. It fails for a found
assumption with no row and for a found-kind row nothing matches (`unlisted` and
`stale`, `tools/nonos-assumptions:55-63`). Rows of kind `stated` have no
detector.

```sh
python3 tools/nonos-assumptions
python3 tools/nonos-assumptions --list
```

The first printed `[assumptions] 123 found, 10 stated, 0 unlisted, 0 stale` and
exited 0; `--list` printed the 123 found entries. A failure prints
`::error::assumption <id> is not in verification/ASSUMPTIONS.md`, or that the
register lists an entry nothing in the tree still rests on, and exits 1. `make
nonos-mk-check-assumptions`, `make nonos-mk-static` and the flake check
`static-abi` run it. [Threat model](../overview/threat-model.md) states the
assumptions in prose.
