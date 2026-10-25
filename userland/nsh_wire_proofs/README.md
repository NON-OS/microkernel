# nsh_wire_proofs

Host tests for `nsh_wire`, the nsh/1 wire format. There are 82 tests.

- **Every refusal is a test.** The builders in `src/build` lay out a valid
  body field by field. Each test changes one field and shows that the
  parser refuses exactly that change.
- **Order.** The kinds each stage admits are written out from the
  specification. The parser must admit exactly those kinds.
- **Sizes.** The largest legal `HOST_PROOF`, `USER_AUTH` and `ESCALATE` must
  fit their stage's cap.
- **Robustness.** Noise, and valid bodies with one byte changed or cut, must
  never make a parser panic.

Each rule was also broken on purpose, one at a time, in `nsh_wire`'s source:
59 mutants, all caught. The list and the test that caught each one are in
`HANDOFF/letters/server/04-wire-parsers.md`.

Run the tests with `cargo test` from this directory.

The fuzz targets, one per parser, are in `fuzz/`. Each target checks what a
successful parse must mean, not only that it does not panic. To fuzz:

1. Seed the corpus with `python3 -I fuzz/seed.py --out <dir>`.
2. Run `cargo +nightly-2026-01-16 fuzz run <target> <dir>/<target>`.
