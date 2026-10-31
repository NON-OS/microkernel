# Benchmark Runs

The benchmark job writes its run directory here on the runner,
`benchmarks/runs/<run id>-<attempt>/`, and uploads it as the artifact
`nonos-benchmarks-<run id>-<attempt>`.

On `main`, nightly and release runs a compact copy is committed to the
`bench-results` branch as `benchmarks/runs/<YYYY-MM-DD>-<sha12>/`, beside
`benchmarks/history.csv` and `benchmarks/LATEST.md` (see ../README.md).
Nothing generated is committed to `main`.
