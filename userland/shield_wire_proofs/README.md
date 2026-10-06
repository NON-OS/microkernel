# shield_wire_proofs

The NOX Shield's wire end to end on the host: the wallet job, the service's
reply, and the follow to landed, with a mocked pool and lander.

The service's reply builders (`capsule_shield/src/reply.rs`) and the
wallet's readers (`capsule_wallet_nonos/src/wallet/shield/reply.rs`) are
mounted unchanged with `#[path]`. Each test builds a reply as the service
does, wraps a job's in the RESULT envelope (`shield_wire::Values::put_job`),
frames and unframes it, and reads it as the wallet does. `src/pool.rs`
drives a spend minute by minute: published, published again after 15
minutes, offered to its owner after 30, and landed by the lander or by the
owner's own settlement.

What it holds:

- the wallet reads the follow's own state every minute, so a field named as
  the envelope's (the bug that hid every landing) fails here, not on a boot;
- every value a follow writes reaches the wallet under its name;
- the owner is offered a settlement at 30 minutes, never twice, and their
  settlement is followed to landed without being published again;
- each spend kept before the last is read by its number, and a failed read
  of them lets none go;
- the history a state reply carries is the wallet's history, and an entry
  of a kind this build does not know is left out.

`cargo test --release` runs it; `cargo clippy --release --all-targets -- -D
warnings` is clean.
