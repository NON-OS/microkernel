
Each row is one change to the shipping source, made alone and reverted after,
with the proof crate that ran against it (`mutants.py` here, from the
repository root; host-run, as the boot cannot reach most of these cases).
KILLED means at least one test failed; the test that caught it is named.
21 mutants, 21 killed. Run on hw/gop-refusal after 3dd09336f, with the
glue pin for the loader's `refusal()` added.

| Mutant | File | Crate | Result | Caught by |
|---|---|---|---|---|
| C1 said twice | `userland/compositor/src/refusal_note.rs` | compositor_proofs | KILLED | nothing_is_logged_before_the_bound_and_nothing_twice, said_at_once_then_once_more_at_the_bound_and_never_again |
| C2 the bound dropped | `userland/compositor/src/refusal_note.rs` | compositor_proofs | KILLED | nothing_is_logged_before_the_bound_and_nothing_twice, said_at_once_then_once_more_at_the_bound_and_never_again |
| C3 logged before the bound | `userland/compositor/src/refusal_note.rs` | compositor_proofs | KILLED | nothing_is_logged_before_the_bound_and_nothing_twice, said_at_once_then_once_more_at_the_bound_and_never_again |
