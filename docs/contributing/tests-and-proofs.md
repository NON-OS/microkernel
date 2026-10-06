# Tests and proofs

How NONOS checks its own code, how to run each kind of check, and which checks fail at this commit.

## What runs where

| Kind | Where it lives | What runs it |
|---|---|---|
| proof crates | `userland/*_proofs`, and a few host crates elsewhere | `nix flake check` |
| kernel feature and profile checks | the kernel crate | `nix flake check` |
| static checks | `scripts/`, `nonos-ci/`, `tools/` | `nix flake check` |
| drift checks | `tools/nix/` | `nix flake check` |
| Kani harnesses | `kani_proofs.rs` and similar files in proof crates | `.github/workflows/verify.yml` |
| Lean specification | `verification/lean/` | `.github/workflows/lean.yml`, `.github/workflows/verify.yml` |
| Charon and Aeneas extraction | `verification/extraction/` | `.github/workflows/verify.yml` |
| Verus | `verification/verus/` | `.github/workflows/verify.yml` |
| fuzz targets | the `fuzz/` directory of a few crates | `.github/workflows/fuzz.yml` |
| QEMU boot | the `qemu` profile | `.github/workflows/ci-boot-smoke.yml` |

The flake's checks are `proofChecks`, `cargoChecks`, `profileChecks`, `staticChecks` and `driftChecks`, each check its own derivation, so a failure names itself and a pass is cached until its inputs change (`tools/nix/checks.nix:303`). The comment at the top of `tools/nix/checks.nix` says why Kani, Verus, the extraction and fuzzing stay outside the flake: each needs a network or a toolchain no lock pins yet.

## Run the checks

```
make check
```

Not tested in this release.

The `check` target runs `nix run .#check-report`, which builds every check for this machine and prints what each one proved (`Makefile:58-61`). One check can be built and its log streamed on its own:

```
nix build .#checks.x86_64-linux.proofs-ps2_input_proofs -L
```

Not tested in this release.

On a macOS host the two TPM proof crates are left out, because the software TPM tools they drive build only for Linux (`needsTpm`, `tools/nix/checks.nix:99-102`).

## State of the checks at this commit

The flake defines 143 checks for `x86_64-linux` at this commit. In a run of all of them on x86_64 Linux, `proofs-model_fetch_proofs` did not evaluate, so it was not built; the `inputs` failure below names the same crate. The other 142 were built and 133 passed, among them every `kernel-features-*` and `kernel-profile-*` check and 110 of the 113 `proofs-*` checks. Nine failed:

| Check | What failed |
|---|---|
| `inputs` | `tools/nix/inputs.json` is stale for `userland/capsule_market`, `userland/capsule_model_fetch` and `userland/model_fetch_proofs` |
| `static-abi` | `scripts/check_prebuilt.py` finds two binaries it cannot classify: `nonos-data/market/index.bin` and `nonos-data/models/catalogue.bin`. The scripts after it in the list did not run |
| `static-hygiene` | `scripts/check_stubs.py` finds 15 admissions that are not in its baseline. The scripts after it did not run |
| `static-tree` | several gates: the `cfg(target_arch` count is 234 against a baseline of 116, the `crate::arch::x86_64::` count 135 against 100, and the end of its log shows the forbidden `read` and `write` import at `userland/capsule_driver_ahci/src/server/handlers/emmc/dispatch.rs:29` |
| `nonos-verify` | clippy passes, then the `hygiene` scan fails. It writes its findings to a file, so the log ends as the scan starts. Its comment patterns match shipping comments such as the "for now" in `src/hardware/inventory/family.rs` |
| `proofs-rtl8169_proofs` | its 67 tests pass; clippy's `manual_div_ceil` lint fails on `userland/capsule_driver_rtl8169/src/log/line.rs` |
| `proofs-usb_msc_proofs` | clippy's `new_without_default` lint fails on `new` at `userland/capsule_driver_usb_msc/src/state/types.rs:38` |
| `proofs-xhci_proofs` | clippy's `int_plus_one` and `assertions_on_constants` lints fail in two test files of the crate |
| `busybox-source` | failed; its log kept no lines |

`scripts/check_allows.py` runs after `check_stubs.py` inside `static-hygiene`, so the failure above stops it. Run on its own against this tree, it passes its self-test and then reports 31 lint switches that are not in its baseline.

## Proof crates

A [proof crate](../overview/glossary.md#proof-crate) is a host crate that compiles shipping source with `#[path]` and runs it under `cargo test`, with the system calls that source makes answered by a shim. `userland/ps2_input_proofs` is a small one. It mounts the PS/2 driver's `constants`, `discover`, `init` and `setup` modules from the [capsule](../overview/glossary.md#capsule) source (`userland/ps2_input_proofs/src/lib.rs:32-40`) and replaces `nonos_libc` with a shim that models the controller (`userland/ps2_input_proofs/Cargo.toml:19-22`). Its 38 tests pass at this commit.

There are 107 crates under `userland/*_proofs`, counted as `proof_crates` (`verification/evidence/EVIDENCE.json:2184`). The flake runs each one the same way: one test thread through `RUST_TEST_THREADS`, a release build with overflow checks on, then clippy (`tools/nix/checks.nix:85-94`). Overflow checks stay on because capsules ship without them, and a proof built the same way would agree with a wrapped value and pass (`RUST_TEST_THREADS`, `tools/nix/checks.nix:81-86`).

To run one crate the same way by hand:

```
nix develop
cargo test --release --manifest-path userland/ps2_input_proofs/Cargo.toml \
    --config profile.release.overflow-checks=true
```

Not tested in this release.

`tpm_enroll_proofs` and `tpm_key_proofs` run against a software TPM, and a live test that skips fails the check instead of passing quietly (`needsTpm`, `tools/nix/checks.nix:32-34`).

### Adding a proof crate

1. Put it at `userland/<name>_proofs` and commit its `Cargo.lock`. `proofDirs` picks up every directory under `userland` whose name ends in `_proofs` and that has a lock (`tools/nix/checks.nix:18-21`).
2. Mount the shipping source with `#[path]`; a copy would test code that does not ship.
3. Regenerate `tools/nix/inputs.json` with `python3 tools/nix/inputs.py`. The `inputs` drift check runs it with `--check` and fails when a path dependency or a `#[path]` was added without it (`tools/nix/checks.nix:265-269`).
4. Make it pass clippy with `-D warnings` over all targets. `lintLib` and `lintNone` take no new members (`tools/nix/checks.nix:44-54`).
5. A new driver capsule ships with its proof crate; [Contributing](README.md) gives the rule.

## Static checks

Four checks read the whole tree, each built by `static`:

- `static-hygiene` runs the stub, lint-switch, unreachable-export, driver-proof and dark-feature scans, each after its own self-test where it has one (`scripts`, `tools/nix/checks.nix:213-219`).
- `static-abi` holds the syscall ABI to its declaration and to its stable set, and checks capsule ports, the handoff mirror, capability tables, prebuilt binaries, the assumption list, Linux and Wayland coverage and the mutation results, among others (`scripts`, `tools/nix/checks.nix:221-234`).
- `static-tree` runs `nonos-ci/run-static-checks.sh`, the grep gates over the whole tree (`static`, `tools/nix/checks.nix:236-239`). Its `fail_with` records a failure and lets the script carry on, so one run reports every gate that fails (`nonos-ci/run-static-checks.sh:15-17`).
- `static-evidence` regenerates `verification/evidence/EVIDENCE.json`, fails on any difference, requires `sorry_count` to be zero and runs the proven-functions ratchet (`tools/nix/checks.nix:241-246`).

Many gates compare against a [baseline](../overview/glossary.md#baseline). In `scripts/`, `run` fails when a site is not in the committed list (`scripts/gate.py:58-91`), and `identity` matches a site by its file and name, not its line, so editing code above a site does not make it new (`scripts/gate.py:30-46`). The counting baselines in `nonos-ci/baselines/` fail when a count grows and ask you to update `baseline_file` in the same pull request when the growth is intended (`nonos-ci/check-baseline.sh:37-43`).

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

Crates under `userland` hold 109 Kani harnesses, counted as `harnesses` (`verification/evidence/EVIDENCE.json:22`); `nonos-attest-path` and `nonos-bootloader/boot_proofs` hold five more. The `kani` job checks `userland/fs_proofs` with Kani 0.67.0 (`.github/workflows/verify.yml:62-72`). A second job runs `cargo kani` in fifteen more proof crates, from `kernel_proofs` to `arch_paging_proofs`, then in `nonos-attest-path` and `nonos-bootloader/boot_proofs` (`.github/workflows/verify.yml:106-122`). 28 of the 109 sit in crates neither job runs: `stark_proofs`, `crypto_proofs`, `nonos_mac`, the three `nonos_nox_*` crates and `shield_core`.

```
cd userland/fs_proofs
cargo kani --output-format terse
```

Not tested in this release.

## Lean

`verification/lean` is a Lean 4 development with no mathlib. Its `defaultTargets` names one library, `Nonos` (`verification/lean/lakefile.toml:2-5`), and `verification/lean/lean-toolchain` pins Lean 4.15.0. At this commit it has 190 modules and 1499 `theorems`, and `sorry_count` is 0 (`verification/evidence/EVIDENCE.json:2176-2181`). The `lean` workflow runs `lake build`, then fails if the axiom profile shows `sorryAx` (`.github/workflows/lean.yml:49-60`).

```
cd verification/lean
lake build
```

Not tested in this release.

## Extraction and Verus

Charon and Aeneas lower Rust MIR into Lean. At this commit 979 functions are extracted; 492 are `substantive`, with a theorem about their behaviour, and 487 are `trivial`, with only the theorem that a generated wrapper is the method it forwards to (`verification/evidence/EVIDENCE.json:28-32`). `tools/ratchets/proven_functions.py` holds the proven count to a floor that may only rise.

`verification/verus` proves theorems about capability bit operations, page-table permission encoding and IPC length guards as restated in its own spec functions. It mounts no kernel file, so a kernel change does not reach it; `verification/README.md` says so. It has 5 `source_files` (`verification/evidence/EVIDENCE.json:2187`).

## Fuzzing

`fuzz.yml` runs cargo-fuzz every night on its `cron`, at 01:00 UTC, over the eleven targets in its `matrix`, from `v4_parse` in `nonos-attest-path` to `ipc_decoders` in `userland/driver_proofs` (`.github/workflows/fuzz.yml:12-65`). Each target runs for `SECONDS_PER_TARGET`, 1800 seconds unless a manual run asks for another length (`.github/workflows/fuzz.yml:66-67`). A crash fails the run, and the log and the crashing input, from the crate's `artifacts` directory, are kept with the run (`.github/workflows/fuzz.yml:93-101`). The workflow's header comment counts thirteen targets; the matrix holds eleven.

```
cd userland/kernel_proofs/fuzz
cargo fuzz run elf_header corpus/elf_header -- -max_total_time=60
```

Not tested in this release.

## What to run for a change

- Kernel or capsule code: the proof crate that mounts it, and `nix flake check`.
- A driver: its proof crate, extended to cover the change.
- A function a refinement theorem names: the extraction job and the Lean build.
- A changed ABI number or capability bit: `static-abi`.
- A parser of untrusted input that has a fuzz target: that target, for a few minutes.
