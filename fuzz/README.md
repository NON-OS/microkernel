# Fuzzing

The fuzz targets live beside the code they fuzz, each crate's own
`fuzz/` directory (`cargo fuzz`, libFuzzer). This directory holds the list CI
fuzzes and describes where the results are kept.

## Which targets CI runs

`targets.json` names every fuzz target in the tree, under `ci` (fuzzed by
`.github/workflows/ci-fuzz.yml`) or `not_in_ci`. `seed: true` runs the crate's
`fuzz/seed.py --out corpus/<target>` before the first run. The workflow builds
its matrix from the `ci` list, and `nonos-ci/fuzz_targets.py` fails when a
target in any tracked `*/fuzz/Cargo.toml` is missing from the file or a listed
one no longer exists, so the list cannot drift from the tree. The lint job
runs that check on every change under `.github/`, `nonos-ci/` or `fuzz/`; the
`not_in_ci` targets are listed in each fuzz plan's job summary.

| trigger | minutes per target | corpus |
|---|---|---|
| pull request (`pr.yml`) | 2 | restored, not saved |
| push to `main` or `develop` (`main.yml`) | 2 | restored and saved |
| nightly (`nightly.yml`) | 30 | restored and saved |
| by hand (`ci-fuzz.yml`) | chosen, at most 320 | chosen |

The short runs on pull requests and pushes build every target and run it
briefly over the corpus the nightly runs grew, so a target that no longer
compiles, or an input already in the corpus that now crashes, fails the
change that caused it.

## Where results are kept

**Corpus, per target, between runs.** Each target's
`<crate>/fuzz/corpus/<target>/` is saved in the Actions cache under
`fuzz-corpus-<crate slug>-<target>-<run id>` and restored from the newest
such key on the next run, so coverage carries over and grows.

**Crashes, per run.** libFuzzer writes a failing input (`crash-*`, `leak-*`,
`timeout-*`, `oom-*`) under `<crate>/fuzz/artifacts/<target>/`. Any of them
fails the job with an error naming the file and the command that replays it:

```
cd <crate>/fuzz && cargo fuzz run <target> artifacts/<target>/crash-<hash>
```

The log and the whole `artifacts/` directory are uploaded as
`fuzz-<crate slug>-<target>`, kept 90 days, pass or fail. A `slow-unit-*`
input is kept but fails nothing.

**Summary, per run.** Each target writes `fuzz-summary-<crate slug>-<target>`:
status, run length, executions and executions per second, coverage (`cov`),
features (`ft`), corpus size before and after, new corpus entries, and its
crash inputs, read from libFuzzer's final stats (`nonos-ci/fuzz_record.py`).
The `fuzz summary` job joins them into `fuzz/results/<YYYY-MM-DD>-<sha12>/`
(`summary.json`, `summary.md`), uploads it as `fuzz-results` and writes the
table to the job summary.

**History, for main and nightly runs.** `ci-publish-results.yml` commits the
run's record to the `bench-results` branch, never to `main`:

```
fuzz/
  results/<YYYY-MM-DD>-<sha12>/   one folder per run (-2, -3 ... for another
    summary.json                  run of the same commit on the same day)
    summary.md
  history.csv                     every target of every kept run, oldest first
  LATEST.md                       the latest run, coverage, features and corpus
                                  against the previous run of the same length
```

`history.csv` and `LATEST.md` are rebuilt from the run folders on every
publish (`python3 nonos-ci/fuzz_record.py rebuild <checkout of bench-results>`).
Publishing never fails the run; the artifacts hold the same data.

## Running one by hand

```
cd userland/tls_proofs/fuzz
cargo fuzz run tls_cert corpus/tls_cert -- -max_total_time=600
```

or the CI wrapper, which also writes the summary:

```
bash nonos-ci/fuzz-run.sh userland/tls_proofs tls_cert 10 true
```
