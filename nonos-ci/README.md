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

## Benchmarks

The `bench_*.py` suite measures boot and runtime against the pinned
baselines under `baselines/`; `bench_compare.py` turns two runs into a
verdict. Regressions fail the lane rather than becoming folklore.

## License

AGPL-3.0, like the rest of NONOS.
