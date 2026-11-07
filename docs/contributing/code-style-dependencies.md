# Code style: Dependencies

Part of [Code style](code-style.md).

## Dependencies

The supply-chain job runs `cargo deny check` against `deny.toml` through
`run_logged` (`nonos-verify/src/supply_chain.rs:25-27`). On a machine without
`cargo-deny` it does not fail: it records the policy check as `Status::Gap`
(`nonos-verify/src/supply_chain.rs:28-31`). The flake's development shell
carries the tool, and CI runs the job inside that shell. The header of
`deny.toml` says the policy is written for the kernel crate.

- Licences: the `allow` list holds the project's own AGPL-3.0 and 0BSD,
  Apache-2.0 (also with the LLVM exception), BSD-2-Clause, BSD-3-Clause,
  CC0-1.0, ISC, MIT, MIT-0, MPL-2.0, Unicode-3.0, Unicode-DFS-2016, Unlicense
  and Zlib (`deny.toml:29-49`).
- Bans: the `deny` list refuses `openssl`, `openssl-sys` and `time` older than
  0.3, and `wildcards` refuses wildcard version requirements
  (`deny.toml:58-70`).
- Sources: the `sources` table admits crates.io and one git repository,
  `NON-OS/STARKs` (`deny.toml:74-78`).

Moving the STARKs pin goes through its own pull request;
[Review](review.md#who-reviews) says how.

## See also

- [Tests and proofs](tests-and-proofs.md)
- [Review](review.md)
- [Commits](commits.md)
- [Architectures](../architectures/README.md)
- [Writing a driver](../drivers/writing-a-driver.md)
- [Contributing](README.md)
