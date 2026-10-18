# nonos_proof_abi

The bytes a NONOS proof travels in, and the order every verifier checks them
in (PLAN interface 1, the plan and the plan):

- `Record`: NFSR v1, the kernel's 240-byte seal of one supervisor family, and
  its two names, `ctx` (the device proof's context words) and `record_id`.
- `encode` and `parse`: the NPRF v1 container and its section tags.
- `hist_commit` and `syscall_root`: the Poseidon chains behind Hc and A, the
  isolation statement's commitments to the private histogram and the
  declared syscall set.
- `scope`, `anchor`, `meas_chain` and `Delivery`: the device proof's scope for
  a listing, the 32 bytes that go on chain, the measurement chain M and the
  delivery body.
- `isolation::STATEMENT`: the pinned isolation program image, its MANIFEST
  and the numbers beside it, written by isolation_proofs from the circuit.
- `wire`: nonos.prove's frames: market_proto's 20-byte header under
  "NPRV", PROVE, STATUS, GET and HEALTH, and the named refusals beyond errnos.
- `verify`: parse, device proof bound to the record under trusted roots and
  an accepted day, measurements, app_id, publics, then the isolation proof.

no_std. Tested on the host by `userland/proof_abi_proofs`.
