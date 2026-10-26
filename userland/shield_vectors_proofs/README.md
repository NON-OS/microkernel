# shield_vectors_proofs

Host proofs for `capsule_shield_vectors` (Test 5). The capsule's modules
are mounted by `#[path]` and run unchanged: `serial.rs`, `vectors.rs`,
`pinned.rs`, `cache.rs`, `run.rs`, `pool.rs`, `heap.rs` with
`heap/counted.rs`, and `prove.rs`. What stays target-only:

- `machine.rs` and `machine/watch.rs` ask the NONOS kernel (`MkProcStat`).
  `src/machine.rs` stands in for them: free memory 0, a watch that reads 0.
- `main.rs`, the capsule's entry. `src/full.rs` is the same run on the host.

`build.rs` is the capsule's, copied: it embeds the file
`NONOS_PERIODIC_CACHE` names as `cache::SHIPPED`, or nothing.

## The tests (`cargo test`): nothing proved

- `fixtures/periodic-synthetic.top`, 48 bytes, sha256
  `820b8d2098bb8315d1bc56dd1da19c57408650ac83fe94d5d16f629025647f6b`,
  pinned in `src/tests/top.rs`. Derivation: `NXTT`, then the little-endian u32s
  cut 0, one level, that level's length 1, then the one node,
  `nox_prover::PERIODIC_ROOT` at the STARKs commit `Cargo.lock` pins
  (a41bb8bb, `nox_prover/src/api.rs` `ROOT_32`). The test rebuilds it from
  that constant; a different prover root fails it.
- Every pinned vector: its `proof.json`'s proof and public words, through
  the capsule's `pinned::forms` over that top, are its
  `proof-format7.bin` byte for byte, and verify. `to_shared` and
  `verify_shared` read only the root of the top.
- One pinned line changed (a public word in `proof.json`, a byte of
  `proof-format7.bin`) fails, its FAIL line naming the vector and the file;
  the lines are captured through the `&mut impl Write` they take.
- The top with any one bit of its root flipped is refused.

## The real cache: `NONOS_PERIODIC_CACHE`

`tools/nix/periodic.nix`'s `periodic.top`, 8,388,660 bytes, never
committed. With the variable set at build time the tests pin its sha256
(`b2d95883f63dd439ac7bc5ea92b6c6e621aa37d2a67707b8342cce05b0bcd96f`,
derivation's `periodic.top.sha256`), run the capsule's `shipped_cache`
(taken whole, refused with one root bit changed) and `built_is_shipped`.

Without it each such test writes
`<test>: NONOS_PERIODIC_CACHE not set, real cache test skipped` to stderr,
past the harness's capture, as the TPM suites do; the lane exports the
variable and fails on that line (`the design notes`).

## The full proving run: the binary, not the tests

```text
NONOS_PERIODIC_CACHE=/path/to/periodic.top cargo run --frozen --release
```

About four minutes: transfer-eth from nothing, its tree held to the shipped
cache byte for byte, then the other three from the shipped cache, each proof
and its format 7 form held to the pinned bytes. Exit 0 only when no line
failed and all four held; without the variable it fails, naming it.
`cargo test` builds the binary and runs its tests, never its `main`, so the
test lane does not prove. Its own check:
`the design notes`.

## Lanes

```text
RUST_TEST_THREADS=1 cargo test --frozen --release --config profile.release.overflow-checks=true
cargo clippy --frozen --release --all-targets -- -D warnings
```
