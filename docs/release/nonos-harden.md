# nonos-harden: one GO or NO-GO before an image touches a machine

`nonos-harden` is a host command. It runs the verifiers this tree holds over
one sealed release and prints one verdict: **GO** or **NO-GO**, with every
check behind it and the reason each one gives. You can sign the verdict with
your own Ed25519 key. It is never signed with a NONOS key.

```
nonos-harden check --release DIR [--tree DIR] [--format text|json]
                   [--out FILE [--sign-key KEY]]
nonos-harden verify-signature --verdict FILE --signature FILE --public-key KEY
nonos-harden --help | --version
```

Build it with `cargo build --release --manifest-path nonos-harden/Cargo.toml`.
Run it from the checkout that built the release, or pass `--tree`. A
development image from `tools/nonos-dev-image` is built in
`target/dev/tree`, a git repository with throwaway keys, so check it with
`--tree target/dev/tree`.

## What GO means

GO means all of these are true when the command ran:

1. All 13 checks below ran, and every one of them passed.
2. Every artifact the release record seals has exactly the bytes the record
   names. A sealed directory holds exactly the files listed, no more and no
   fewer.
3. The kernel carries the release signature the loader checks. Its Ed25519
   and ML-DSA-65 halves both verify under the kernel public keys in the tree
   (`nonos-data/trust/keys/`), which are the keys the loader is built with.
   The rollback index is the one the record names.
4. The build receipt agrees with the release, and the tree is the source of
   that receipt's commit. HEAD is that commit or a later one, and nothing
   differs from it outside `nonos-data/`.
5. The tree's own verifiers pass: `nonos-verify` hygiene, trust-chain and
   supply-chain, `tools/nix/inputs.py --check`, and the verify matrix's
   static-checks and proof-crates rows.

A release image that passes reads `GO`. A development image that passes
reads:

```
GO (development image, not a release)
```

When passing checks carry limits (see below), the line counts them, so a
limited GO never reads as a plain one. Each limit is listed right under the
verdict line:

```
GO (development image, not a release), 2 declared limits
  limit: release.receipt: does not cover the sealing tree held changes beyond 1a5ac13 (the seal stages its trust set first)
  limit: verify.trust-chain: does not cover per-capsule field assertions (it needs: expose DecodedManifest fields …)
```

## Declared limits

A `nonos-verify` module may report parts it does not cover, called *gaps*. A
gap is allowed with GO only if the tree being checked declares it in
`scripts/baselines/declared-limits.txt` (format
`nonos-harden-declared-limits/1`, one `<module>: <what>` line each). A
declared gap becomes a limit: it is counted in the verdict line and in the
JSON `limits` field, and listed under the verdict line.

These cases fail the verifier's check, and so the verdict is NO-GO:

- A gap that is not on the list, or is declared only for another module.
- A list that is missing or not in its format. The list is read before the
  verifier runs.

`scripts/check_declared_limits.py`, run by the static checks, holds the list
to two rules:

- Every line must name a gap that module's source still declares, so a line
  goes once its gap is closed.
- No line may be added compared with the previous commit, or with the
  merge-base against `NONOS_BASELINE_BASE` when that is set. The list only
  shrinks.

Today it holds one line, `trust-chain: per-capsule field assertions`. The
other gaps nonos-verify can declare always come with a check that did not
pass, which is NO-GO on its own.

## What GO does not mean

- **The image was not booted.** No QEMU run and no device are involved. The
  verdict says so in its `scope` field, every time. The boot gate is a
  separate step.
- GO is only as good as the verifiers in the tree it runs. It does not add
  checks of its own beyond reading the release.
- A declared gap is a limit, not a pass of what it names. Read the limits
  under the verdict line. A check that could not run is never a pass: it
  makes the verdict NO-GO, and so does a verifier that is missing.
- The receipt check adds two limits: when the receipt measured no kernel
  attestation root, and when the sealing tree was dirty. Real seals are always
  dirty, because the seal stages its trust set before it writes the receipt.
- A signed verdict proves only who signed which bytes. It does not make
  NO-GO into GO, and `verify-signature` does not report the verdict.

## NO-GO

NO-GO means at least one check failed or could not run. The verdict line
names how many checks did not pass and which one was first. For example:

```
NO-GO: 1 of 13 checks did not pass; first release.sealed
```

Each check's reason names what it found, such as the file whose digest
differs or the verifier check that failed.

These cases are also NO-GO:

- No check ran at all.
- The kind of image (release or development) was not established.

## The checks, in the order they run

There is no way to skip a check. A check whose input was not established by
an earlier check reports `not-run`.

| id | passes when |
|---|---|
| `release.record` | `nonos-release.json` is a regular file under 16 MiB, in the `nonos-release/1` shape the seal writes. Every path is relative and plain (no `..`, no root, no `.`, no empty part, no backslash or control character). Every digest is lowercase SHA-256 hex. |
| `release.sealed` | Every sealed artifact has the bytes the record names. A sealed directory holds exactly the listed files. Symlinks are refused anywhere on the path. |
| `release.kind` | The record says release or development. If the record claims a release but carries `nonos-dev-attest`, that is a contradiction and the check fails. |
| `release.receipt` | `build-receipt.json` (`nonos.build.receipt.v1`) found its roots embedded. It measured no missing artifact, and everything it measured is among the sealed bytes. |
| `tree.source` | The receipt's commit exists in the tree and is HEAD or an ancestor of it. Tracked and untracked changes are allowed only under `nonos-data/`. |
| `tree.ledger` | `nonos-data/trust/MANIFEST.sha256` names every file under the trust set's `capsules/`, `keys/` and `policy/`, each with its digest. No file there is missing from the ledger or added beside it. |
| `release.kernel-signature` | The kernel's NONOSIMG footer, NKRSIG2 block, both signatures and the rollback index check out, as described in item 3 above. |
| `tree.build-inputs` | `python3 tools/nix/inputs.py --check` exits 0, within 15 minutes. |
| `verify.hygiene` | Every check in `nonos-verify hygiene`'s report passes, and the tool exits 0, within 1 hour. |
| `verify.trust-chain` | The same, for `nonos-verify trust-chain`. |
| `verify.supply-chain` | The same, for `nonos-verify supply-chain`. |
| `matrix.static-checks` | Every step of the verify matrix's `static-checks` row exits 0, within 2 hours. |
| `matrix.proof-crates` | Every step of the `proof-crates` row exits 0, within 10 hours. |

`nonos-verify` exits 0 even when a check could not run. So `nonos-harden`
reads the report the tool writes, not its exit status:

| report status | result |
|---|---|
| `fail`, or any status other than `pass`, `gap` or `skip` | the check fails |
| `gap` or `skip` | the check is not run |
| a report with no checks | the check is not run |

A verifier that runs past its time limit is stopped. One that cannot start
counts as not run.

## The verdict format, `nonos-harden-verdict/1`

`--format json` prints the verdict as one JSON object, and `--out` writes it
the same way. If the meaning of any field changes, the format gets a new
number. It is never changed silently.

| field | meaning |
|---|---|
| `format` | always `"nonos-harden-verdict/1"` |
| `tool` | `"nonos-harden <version>"` |
| `overall` | `"go"` or `"no-go"` |
| `verdict` | the one line: `GO`, `GO (development image, not a release)`, either followed by `, N declared limit(s)` when checks carry limits, or `NO-GO: …` |
| `kind` | `"release"`, `"development"` or `"unknown"` |
| `limits` | how many limits the checks carry, in all |
| `release` | the release directory as given |
| `record_sha256` | the SHA-256 of `nonos-release.json`, or `null` if it was not read |
| `tree` | the tree checked against, or `null` |
| `tree_head` | the tree's HEAD commit, or `null` |
| `signer` | the operator's Ed25519 public key in hex when the verdict is signed, else `null` |
| `scope` | what the verdict covers (host checks only; nothing booted) |
| `checks` | one object per check, in run order: `id`, `status` (`pass`, `fail`, `not-run`), `reason`, and `limits` (only when there are any) |

The text form holds the same facts: one line per check, then the verdict
line, then one `  limit: <check>: does not cover <what>` line per limit. Control characters and bidirectional marks in any reason are
replaced with U+FFFD, so a hostile file name cannot rewrite your terminal.

## Signing

```
nonos-harden check --release out/ --out verdict.json --format json --sign-key ~/my.key
nonos-harden verify-signature --verdict verdict.json --signature verdict.json.sig --public-key <hex>
```

- `--sign-key` takes your Ed25519 key in one of three forms: a 32-byte seed
  file, 64 hex digits, or a nonos-sign `NONOSSK1` seed file. The key is read
  and vetted **before** any check runs.
- The key is refused if it lies under `nonos-data/` or
  `nonos-bootloader/keys/`, or in the `.keys/` directory of a NONOS checkout.
  Links are followed first. It is also refused if its public key is any
  Ed25519 key published in the tree's `nonos-data/trust/keys/`.
- The signature (64 bytes, hex) is written to `FILE.sig` and covers the exact
  bytes of `FILE`. The verdict names the signer.
- If the verdict or the signature cannot be written, nothing is delivered. No
  unsigned file is left behind, and the exit status is 3.
- `verify-signature` checks the signature with strict verification. It also
  checks that the verdict names the given key as its signer.

## Exit status

| code | meaning |
|---|---|
| 0 | GO (for `verify-signature`: the signature verifies) |
| 1 | NO-GO |
| 2 | bad command line, or a signing key refused |
| 3 | the verdict could not be written or signed |
| 4 | the signature does not verify |

The command line is read strictly. It refuses unknown, repeated or empty
flags, and it refuses `--sign-key` without `--out`.

## How it is proved

Run the proofs with `cargo test --manifest-path nonos-harden/Cargo.toml`.
They are in the tree's test lanes.

- **Fixtures.** The proofs build a sealed release with the seal's own code.
  `sign-kernel` makes the signature block and footer, and `embed-trailer`
  assembles the attested kernel, inside a real git tree that has its own
  keys.
- **GO cases.** That development release reads
  `GO (development image, not a release)`, and the release profile reads `GO`.
- **One flipped byte.** The tree's own `tools/nonos-flip-byte` flips one byte
  in a copy of the release, and the copy reads NO-GO:
  - A byte flipped in the image names `release.sealed`.
  - A byte flipped in the kernel names both `release.sealed` and
    `release.kernel-signature`.
- **Fuzzing (proptest).** Every parser of untrusted input gets arbitrary
  bytes and also mutated or cut copies of the genuine documents. The parsers
  covered are the record, the receipt, the ledger, the kernel image, the
  verifier report, the matrix summary, and the key file.
  - No parser panics.
  - A mutated record that is still accepted names only safe paths and
    well-formed digests.
  - No mutation of the kernel is accepted as another kernel or another
    rollback index.
  - Every cut of the kernel is refused.
- **Mutants.** Each line of `nonos-harden/mutants.json` breaks one rule and
  names the proof that catches it. `python3 nonos-harden/mutants.py` applies
  each mutant to a copy of the crate and checks that its proof fails. The crate
  itself is never changed. Three of the mutants drop a check from the plan, and
  each is caught. A separate proof checks that every mutant still applies
  exactly once.

### Mutant results

`python3 nonos-harden/mutants.py` runs on the crate as landed. All 31 mutants are caught, and none escaped.

| mutant | file | caught by |
|---|---|---|
| `plan-skips-kernel-signature` | `src/plan.rs` | `plan_tests::every_check_runs_in_order_every_time` |
| `plan-skips-sealed` | `src/plan.rs` | `plan_tests::every_check_runs_in_order_every_time` |
| `plan-skips-proof-crates` | `src/plan.rs` | `plan_tests::every_check_runs_in_order_every_time` |
| `decide-not-run-counts-as-pass` | `src/verdict/decide.rs` | `verdict_tests::go_needs_every_check_run_and_passed_and_the_kind_known` |
| `decide-unknown-kind-is-go` | `src/verdict/decide.rs` | `verdict_tests::go_needs_every_check_run_and_passed_and_the_kind_known` |
| `decide-no-checks-is-not-refused` | `src/verdict/decide.rs` | `verdict_tests::go_needs_every_check_run_and_passed_and_the_kind_known` |
| `decide-dev-reads-plain-go` | `src/verdict/decide.rs` | `plan_go_tests::a_passing_development_release_reads_go_not_a_release_and_a_release_go` |
| `judge-ignores-failed-checks` | `src/tree/judge.rs` | `judge_tests::a_failed_check_or_one_of_unknown_status_fails` |
| `judge-ignores-gaps` | `src/tree/judge.rs` | `judge_tests::a_check_that_could_not_run_does_not_pass_though_the_tool_exits_0` |
| `judge-ignores-exit-code` | `src/tree/judge.rs` | `judge_more_tests::no_report_an_empty_one_or_a_bad_exit_does_not_pass` |
| `sealed-accepts-any-digest` | `src/release/sealed.rs` | `sealed_tests::one_flipped_byte_anywhere_fails_and_is_named` |
| `sealed-dir-allows-extra-files` | `src/release/sealed_dir.rs` | `sealed_tests::a_file_missing_from_the_esp_or_one_added_to_it_fails` |
| `resolve-follows-symlinks` | `src/release/resolve.rs` | `sealed_link_tests::a_link_in_place_of_a_sealed_file_fails_even_to_the_same_bytes` |
| `kind-allows-dev-attest-release` | `src/release/kind.rs` | `kind_tests::a_release_profile_carrying_the_development_attestation_is_no_go` |
| `kernel-ed25519-not-checked` | `src/release/kernel_keys.rs` | `kernel_tests::a_changed_byte_of_either_signature_alone_is_caught` |
| `kernel-mldsa-not-checked` | `src/release/kernel_keys.rs` | `kernel_tests::a_changed_byte_of_either_signature_alone_is_caught` |
| `kernel-rollback-not-compared` | `src/release/kernel_check.rs` | `kernel_key_tests::a_rollback_index_the_record_does_not_name_is_refused` |
| `source-allows-changes-outside-trust-set` | `src/tree/source.rs` | `source_tests::a_change_or_an_addition_outside_the_trust_set_fails_and_is_named` |
| `source-allows-commit-not-in-history` | `src/tree/source.rs` | `source_fail_tests::a_commit_the_tree_lacks_or_a_head_before_it_fails` |
| `ledger-allows-unlisted-files` | `src/tree/ledger.rs` | `ledger_tests::a_changed_missing_or_added_file_fails_and_is_named` |
| `receipt-allows-unsealed-artifact` | `src/release/receipt_agree.rs` | `receipt_tests::a_receipt_that_disagrees_with_the_release_fails` |
| `receipt-drops-limits` | `src/release/receipt_check.rs` | `receipt_shape_tests::a_dirty_sealing_tree_is_stated_as_a_limit_and_not_hidden` |
| `matrix-ignores-failed-steps` | `src/tree/summary.rs` | `matrix_tests::a_row_whose_steps_pass_passes_and_one_that_fails_is_named` |
| `inputs-ignores-exit-code` | `src/tree/inputs.rs` | `matrix_tests::the_build_inputs_pass_only_when_inputs_py_does` |
| `sign-key-place-not-refused` | `src/sign/refuse.rs` | `sign_refuse_tests::a_key_where_nonos_keeps_its_keys_is_refused_before_any_check_runs` |
| `sign-nonos-public-key-not-refused` | `src/sign/refuse.rs` | `sign_refuse_more_tests::a_key_the_trust_set_publishes_is_refused` |
| `verify-signature-ignores-named-signer` | `src/sign/verify.rs` | `sign_tamper_tests::a_verdict_naming_another_signer_does_not_verify_though_signed` |
| `deliver-leaves-unsigned-verdict` | `src/deliver.rs` | `sign_tamper_tests::a_signature_that_cannot_be_written_leaves_no_unsigned_verdict` |
| `cli-allows-repeated-flag` | `src/cli/flags.rs` | `cli_tests::anything_else_is_refused` |
| `clean-passes-control-characters` | `src/verdict/text.rs` | `verdict_tests::the_text_verdict_cannot_be_steered_by_what_it_quotes` |
| `path-allows-dot-dot` | `src/release/path.rs` | `record_tests::a_path_that_could_leave_the_release_is_refused` |
