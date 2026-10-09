# nonos_equix

`nonos_equix` is HashX and Equi-X, the client puzzle an onion service can ask
for before it answers an introduction (Tor proposal 327, carried unchanged by
the Anyone fork in `src/ext/equix`). `no_std`, `alloc`, no dependencies.

It is a port of the reference C, interpreter only. The reference can compile
each HashX program to x86 or arm64 machine code; net.anon never maps memory
executable, so that half has no counterpart here. Both compute the same
function, and a service cannot tell which one a client used.

## Public surface

- `HashX::new(seed)` builds the hash function a seed selects, or returns
  `None` for the roughly one seed in ten thousand whose program fails the
  reference's uniformity rules. `HashX::hash(u64) -> u64` is the reference's
  eight-byte output read little endian (Tor builds HashX with `HASHX_SIZE=8`).
- `Solver::new()` reserves the solver's 1.9 MB fallibly and returns `None` if
  the heap cannot give it. `solve(&hashx, &mut out)` finds up to eight
  solutions in one call. `start`, `hash_some(&hashx, budget)` and `finish(&mut
  out)` do the same work in slices, which is how the capsule keeps its event
  loop answering while it solves.
- `verify(challenge, &solution)` checks the canonical order first, without
  hashing, and then the partial and full sums. `verify_with` reuses a
  `HashX` that has already been built.
- `Solution::to_bytes` and `from_bytes` handle the 16-byte wire form, with
  each index little endian.
- `Blake2b` is unkeyed BLAKE2b with a salt, for HashX's seed expansion and
  for the onion service's effort check.

## What holds it

`userland/equix_proofs` checks it against the fork's own C build
(`vectors/equix_gen.c` writes `vectors/equix.expect` from
`src/ext/equix`). The checks are:

- HashX outputs for four seeds and five inputs each, including the upstream
  suite's published values.
- The seeds the reference refuses.
- The exact solution lists, in order, for 32 challenges.
- Every way of breaking a solution, including the upstream proof that only
  one of the 40320 orderings of a solution verifies.
- BLAKE2b against RFC 7693 and the reference.
- Slice-by-slice solving gives the same answer as solving in one call.

The Tor-specific challenge and effort check live in
`capsule_net_anon/src/onion/pow` and are checked in `anon_ntor_proofs` and
`anon_onion_proofs`.
