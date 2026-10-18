# Tests and proofs: Static checks

Part of [Tests and proofs](tests-and-proofs.md).

## Static checks

Four checks read the whole tree, each built by `static`:

- `static-hygiene` runs the stub, lint-switch, unreachable-export, driver-proof
  and dark-feature scans, each after its own self-test where it has one
  (`scripts`, `tools/nix/checks.nix:213-219`).
- `static-abi` holds the syscall ABI to its declaration and to its stable set,
  and checks capsule ports, the handoff mirror, capability tables, prebuilt
  binaries, the assumption list, Linux and Wayland coverage and the mutation
  results, among others (`scripts`, `tools/nix/checks.nix:221-234`).
- `static-tree` runs `nonos-ci/run-static-checks.sh`, the grep gates over the
  whole tree (`static`, `tools/nix/checks.nix:236-239`). Its `fail_with` records
  a failure and lets the script carry on, so one run reports every gate that
  fails (`nonos-ci/run-static-checks.sh:15-17`).
- `static-evidence` regenerates `verification/evidence/EVIDENCE.json`, fails on
  any difference, requires `sorry_count` to be zero and runs the
  proven-functions ratchet (`tools/nix/checks.nix:241-246`).

Many gates compare against a
[baseline](../overview/glossary-amnesic-boot.md#baseline). In `scripts/`, `run`
fails when a site is not in the committed list (`scripts/gate.py:58-91`), and
`identity` matches a site by its file and name, not its line, so editing code
above a site does not make it new (`scripts/gate.py:30-46`). The counting
baselines in `nonos-ci/baselines/` fail when a count grows and ask you to update
`baseline_file` in the same pull request when the growth is intended
(`nonos-ci/check-baseline.sh:37-43`).

Each gate in `scripts/` runs on its own:

```
python3 scripts/check_allows.py --self-test
python3 scripts/check_allows.py
python3 scripts/check_driver_proofs.py
```

The tree-wide script takes longer:

```
bash nonos-ci/run-static-checks.sh
```

Not tested in this release.

## Kani

Crates under `userland` hold 109 Kani harnesses, counted as `harnesses`
(`verification/evidence/EVIDENCE.json:22`); `nonos-attest-path` and
`nonos-bootloader/boot_proofs` hold five more. The `kani` job checks
`userland/fs_proofs` with Kani 0.67.0 (`.github/workflows/verify.yml:62-72`). A
second job runs `cargo kani` in fifteen more proof crates, from `kernel_proofs`
to `arch_paging_proofs`, then in `nonos-attest-path` and
`nonos-bootloader/boot_proofs` (`.github/workflows/verify.yml:106-122`). 28 of
the 109 sit in crates neither job runs: `stark_proofs`, `crypto_proofs`,
`nonos_mac`, the three `nonos_nox_*` crates and `shield_core`.

```
cd userland/fs_proofs
cargo kani --output-format terse
```

Not tested in this release.
