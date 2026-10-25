# modes_proofs

Host proofs for `nonos_modes`, which is mounted by `#[path]` so the tests
run the shipping source.

- CVT 1.2: the results its formulas give for 1080p, 1440p and 2160p in
  standard, reduced v1 and reduced v2 blanking; the width cut to the cell;
  refused rates; and a sweep of sizes, rates and blankings that must never
  run faster than asked.
- DMT: every row the data file marks as CVT must be exactly what the CVT
  code computes (45 rows), which checks the table and the formulas against
  each other; every row runs near its nominal rate; lookups by blanking.
- The range limits check, DisplayPort and TMDS capacity at their edges,
  the fit rule, and EDID timings resolved to DMT or CVT.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
