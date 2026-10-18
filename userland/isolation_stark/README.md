# isolation_stark

The isolation statement of the plan: a sealed family's histogram H, under its
salt, commits to Hc; the declared syscall set commits to A; and no call
outside the set succeeded. It carries no counts.

- `prove(witness, entropy)`: the statement the witness proves and its
  format 7 proof. The proof is blinded by hide_wired and leaves only after it
  verifies and passes check_fri_rank_rounds. A missed certificate is
  `Error::Rank`: prove again with fresh entropy.
- `verify(statement, proof)`: the engine's check of the same circuit.
- `shape(statement)`: the verifier's circuit, from which isolation_proofs
  writes the program image that nox_verify and the owner's evaluator read.

Every region is a ShieldRegion kind: the H and A Poseidon chains
(Membership), one LiveGate per bucket with live = a_b and value = [ok_b, a
free limb], and the eight public words, tied by copy constraints. A clear
bit forces ok_b to zero. The `forge` feature is a forger's prover for the
host proofs only. Tested on the host by `userland/isolation_proofs`.
