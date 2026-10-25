# nonos-ci

The scripts NONOS CI lanes call between checkout and verdict. Each one
does a single job and is invoked by name from the workflows in the
kernel repository, so what a lane does is readable here rather than
inlined in YAML.

## Provisioning

`setup-signing-key.sh` resolves the Ed25519 dev signing seed for a
lane: the production secret when present, a clearly marked
deterministic fork seed otherwise, so fork CI stays honest without
access to anything real. `scratch-trust-bootstrap.sh` mints a complete
throwaway trust chain, anchors, publisher keys, certificates,
manifests, enrollment, for scratch lanes that build and boot the whole
system without touching the committed keystore. Production lanes run
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

## Lanes

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

## Benchmarks

The `bench_*.py` suite measures boot and runtime against the pinned
baselines under `baselines/`; `bench_compare.py` turns two runs into a
verdict. Regressions fail the lane rather than becoming folklore.

## License

AGPL-3.0, like the rest of NONOS.
