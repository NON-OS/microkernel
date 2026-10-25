# agentproof_policy

An AgentProof box policy as a file: the `nonos-box 1` format, and the `agentproof` command
that checks one. The format's page is `docs/agentproof/policy-format.md`.

- `format/`: the reader. One meaning per file; every line it does not know is refused, with
  its number.
- `compile.rs`: the one `linux --box` word a file means, the box capsule_linux makes of it for
  a terminal run, and that box's hash.
- `explain.rs`: what the box allows, in plain words.
- `cli/`: what `agentproof policy check|word|hash` answers, as a pure function the proofs drive;
  `bin/agentproof.rs` only reads the file and prints.
- `boxed/`: capsule_linux's own box files, mounted by `#[path]`. The folder rule, the presets,
  the hash and the terminal's no-network rule are never restated here.

`no_std` with `alloc`; the binary uses std. Host only.

```sh
cargo build --release --locked
cargo clippy --all-targets --locked -- -D warnings
```

The proofs are in `../agentproof_proofs`.
