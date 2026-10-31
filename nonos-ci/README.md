# nonos-ci

The scripts NONOS CI jobs call between checkout and verdict. Each one
does a single job and is invoked by name from the workflows in the
kernel repository, so what a job does is readable here rather than
inlined in YAML.

## Provisioning

`setup-signing-key.sh` resolves the Ed25519 dev signing seed for a
job: the production secret when present, a clearly marked
deterministic fork seed otherwise, so fork CI stays honest without
access to anything real. `scratch-trust-bootstrap.sh` mints a complete
throwaway trust chain, anchors, publisher keys, certificates,
manifests, enrollment, for scratch jobs that build and boot the whole
system without touching the committed keystore. Production jobs run
neither bootstrap: they verify the committed set instead.

## Checks

The `check-*` and `scan-*` scripts are the static battery: baseline
comparisons, capsule endpoint collision checks, feature profile
consistency, the pqclean pin, binary hygiene, and the microkernel
symbol scan. `run-static-checks.sh` drives the set.

`check-capsule-targets.py` asks make, without building, which target each
capsule is built for under each user target, and fails if any NONOS capsule
does not follow the one asked. It runs as the flake's `capsule-targets`
check and in `make nonos-mk-static`.

## Jobs

`build-driver-capsules.sh <user target>` builds every driver capsule for
that target through make's own rule, and any driver make leaves out with the
rule's cargo line, then checks each ELF's machine. ci-build-aarch64 runs it
for `aarch64-nonos-user`.

`build-every-capsule.sh <user target> [k/n] [--recheck]` does the same for
every NONOS capsule make knows (`list-capsules.sh` asks make for them),
checking each ELF with `elf-machine.py`. A capsule that cannot build for a
target is named, with why, in `arch-reach/capsules.txt`; `--recheck` builds
only those and fails on any that builds. `check-proofs-every-arch.sh
<triple>` type-checks every proof crate, tests included, for aarch64 or
riscv64 Linux. ci-every-arch-capsules and ci-every-arch-proofs run them for
aarch64 and riscv64.

## Every architecture

`arch-reach/cfg-x86_64.txt` and `arch-reach/arch-x86_64-uses.txt` name every
kernel file outside `src/arch` that gates on x86_64 or reaches into
`crate::arch::x86_64`, with the line count and why; a row starting `GAP:` is
a feature the other architectures lack. `userland/arch_reach_proofs` holds
the tree to them exactly, so new x86_64-only code cannot land unexplained.

## Pipelines

The workflows under `.github/workflows/` call these rather than carry
shell inline; `docs/build/ci.md` describes the layout.

- `provision-trust.sh <scratch|production>`: the trust material a job
  signs and boots with, over the flake's capsules from the binary cache.
- `flake-check-shard.sh <k>/<n>`: one shard of this host's flake checks,
  dealt round-robin from the list the flake gives, with a per-check verdict.
- `enable-kvm.sh`, `boot-smoke.sh`: KVM for the runner user and the boot
  gate's steps (keys, seal, boot, USB boot).
- `wsl-nix.sh`, `windows-reclaim-disk.ps1`, `nix-cache-config.sh`: the
  Windows job inside WSL2 (Nix at the devcontainer's pinned version), and
  the binary cache for a Nix the cachix action does not set up.
- `bench-run.sh`, `bench_record.py`: the benchmark job's run, and its
  records with `history.csv` and `LATEST.md`.
- `fuzz_targets.py`, `fuzz-run.sh`, `fuzz_record.py`: the fuzz matrix from
  `fuzz/targets.json` held to the tree, one target's run, and the records.
- `publish-results.sh`: commit the benchmark and fuzz records to the
  `bench-results` branch; never fails the run.
- `ci_summary.py`: one verdict per workflow run, written to the job summary.
- `lint-ci.sh`: actionlint, shellcheck, SHA pins and the fuzz list.
- `release-assets.sh`, `enrollment-set.sh`, `require-green.sh`,
  `starks-bump.sh`, `lean-evidence.sh`, `kani-proof-crates.sh`,
  `install-verifiers.sh`, `run-proofs-on-arch.sh`, `reclaim-disk.sh`,
  `windows-virtualbox.ps1`: one job's steps each, named by what they do.

## Benchmarks

The `bench_*.py` suite measures boot and runtime against the pinned
baselines under `baselines/`; `bench_compare.py` turns two runs into a
verdict. Regressions fail the job rather than becoming folklore. Where
every run's results are kept is in `benchmarks/README.md`.

## License

AGPL-3.0, like the rest of NONOS.
