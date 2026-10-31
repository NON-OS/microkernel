# NØNOS Benchmarks

This directory is the public benchmark evidence root.

## What a run measures

Every benchmark run (`.github/workflows/ci-benchmark.yml`, driven by
`nonos-ci/bench-run.sh` and `make nonos-mk-bench`) writes one run directory.
The job provisions scratch trust for the desktop GUI capsule fleet from the
binary cache before the boot harness runs. A manual dispatch can disable QEMU
boot evidence for host/build-only diagnosis, but the default job is expected
to produce boot artifacts.

The run directory contract is:

- `host.json`: host, toolchain, git, CPU, memory, and QEMU metadata
- `build-verify-fast.json`: build verification wall time and resource usage
- `boot-evidence.json`: bounded boot evidence harness timing
- `boot-log.json`: parsed serial markers, boot phases, and ZK/capsule status
- `results.jsonl`: normalized benchmark records
- `metrics.csv`: tabular metrics and parsed boot phase samples
- `manifest.json`: SHA-256 manifest for every artifact in the run directory
- `summary.md`: human-readable run summary

Boot latency claims must use parsed `[BENCH]` phase metrics from `boot-log.json`
and `metrics.csv`, not harness wall time alone.

## When it runs

| trigger | boots | gates |
|---|---|---|
| pull request (`pr.yml`) | 1 | yes (strict) |
| push to `main` or `develop` (`main.yml`) | 1 | yes (strict) |
| nightly (`nightly.yml`) | 3 | reported in the nightly summary |
| release tag (`release.yml`) | 3 | recorded, does not hold the release |
| by hand (`ci-benchmark.yml`) | chosen | chosen |

A change to `docs/` or Markdown alone runs no benchmark: no build reads those
files.

## Where results are kept

**Every run, the full directory.** The job uploads the whole run directory as
the artifact `nonos-benchmarks-<run id>-<attempt>`, kept 90 days, whether the
run passed or not. Its `summary.md` is also the job summary.

**Runs on main, nightly and release, as a record with history.** After the run,
`ci-publish-results.yml` commits a compact record to the `bench-results`
branch, never to `main`:

```
benchmarks/
  runs/<YYYY-MM-DD>-<sha12>/   one folder per run (-2, -3 ... for another
                               run of the same commit on the same day)
    results.jsonl              the run's normalized records
    summary.md                 the run's summary
    metrics.csv                the run's metrics
    host.json                  the host it ran on
    run.json                   commit, date, event, ref and workflow run
  history.csv                  every metric of every kept run, oldest first
  LATEST.md                    the latest run as a table, each metric against
                               the run before it
```

`history.csv` and `LATEST.md` are rebuilt from the run folders on every
publish, so they can always be regenerated with
`python3 nonos-ci/bench_record.py rebuild <checkout of bench-results>`. The
commit is `bench: <date> <sha>` by `github-actions[bot]`. A failed push is
retried from the branch tip and, failing that, left as a warning: publishing
never fails the run, and the artifact still holds the data.

Read the history with:

```
git fetch origin bench-results
git show origin/bench-results:benchmarks/LATEST.md
git show origin/bench-results:benchmarks/history.csv
```

Pull-request runs are kept as artifacts only.

## Comparing two runs

`make nonos-mk-bench-compare NONOS_BENCH_BASELINE=<dir> NONOS_BENCH_CANDIDATE=<dir>`
fails when a metric is more than `NONOS_BENCH_REGRESSION_LIMIT` percent
(default 15) slower than the baseline (`nonos-ci/bench_compare.py`).
