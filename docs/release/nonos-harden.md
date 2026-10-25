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

## What is trusted, and what is not

- **The release directory is untrusted.** That covers everything in it: the
  record (`nonos-release.json`), the receipt, the kernel, the ESP and the
  images. Nothing in it signs the record.
- **The tree is the operator's own checkout.** Its git store (`.git`) is
  trusted to hold the commits it holds. `tree.source` then shows the files on
  disk are that commit's, except where the seal stages what it writes.
- **The tree is never taken from inside the release.** A tree at or inside
  the release directory is refused (exit 2), whether it was given with
  `--tree` or found from where the command runs. A release cannot bring the
  verifiers that judge it.

## What GO means

GO means all of these were true when the command ran:

1. **Every check passed.** All 13 checks below ran, and every one passed.
2. **The sealed bytes match the record.** Every artifact the release record
   seals has exactly the bytes the record names. A sealed directory holds
   exactly the files listed, no more and no fewer. Links, FIFOs and other
   special files are refused, and every file is opened once, without
   following a link.
3. **The kernel that boots is signed.** The kernel carries the release
   signature the loader checks, and both halves of it, Ed25519 and
   ML-DSA-65, verify under the kernel public keys in the tree, read as the
   seal writes them: `kernel_signing_ed25519.pub` is the raw 32-byte key (the
   loader's build takes its last 32 bytes), and `kernel_mldsa65.pub` is a
   nonos-sign NONOSPK1 file. The bytes
   verified are the bytes the record seals as `kernel`. The ESP's
   `kernel.bin` is those same bytes. The rollback index is the one the record
   names, and a record that names none fails. The NONOSIMG footer is exactly
   what embed-trailer writes, reserved bytes included, and the attestation
   trailer is not empty.
4. **The kernel keys are the commit's.** For a release, the tree's kernel
   public keys (`nonos-data/trust/keys/kernel_*.pub`) are the ones in the
   receipt's commit. A development image's own throwaway keys pass only as a
   stated limit.
5. **The tree is the release's source.** HEAD is the receipt's commit or a
   later one. Every tracked file on disk has the bytes, kind and executable
   bit that commit records, and no file it lacks is present. Differences are
   allowed only under `nonos-data/trust/`, `nonos-data/market/` and
   `nonos-data/models/`, which is where the seal stages what it writes.
   - The files are hashed by nonos-harden itself, as git hashes a blob, so
     git's filters, index flags and rename detection cannot hide a change.
     Git runs with no system or global configuration.
   - Two kinds of directory are not walked: those named `target` (cargo build
     output) and those named `__pycache__`. The verifiers nonos-harden starts
     never read them: cargo builds into the run's own scratch space, and
     Python caches its bytecode there.
6. **The tree's own verifiers pass.** These are:
   - `nonos-verify` hygiene, trust-chain and supply-chain (built with
     `--locked`)
   - `tools/nix/inputs.py --check`
   - the verify matrix's static-checks and proof-crates rows

A GO always carries at least one limit, so it reads with a count. For a
release, exiting 0:

```
GO, 1 declared limit
  limit: release.sealed: does not cover the loader, the ESP's other files and the USB and ISO images, past being the bytes the release record names: the record is not signed
```

A development image that passes exits 5, not 0, and reads `GO (development
image, not a release)`, followed by its count of limits:

```
GO (development image, not a release), 4 declared limits
  limit: release.sealed: does not cover the loader, the ESP's other files and the USB and ISO images, past being the bytes the release record names: the record is not signed
  limit: release.receipt: does not cover the sealing tree held changes beyond <commit> (the seal stages its trust set first)
  limit: tree.source: does not cover the kernel's public keys are this development tree's own, not commit <commit>'s
  limit: verify.trust-chain: does not cover per-capsule field assertions (it needs: expose DecodedManifest fields …)
```

The count is what makes a limited GO impossible to mistake for a plain one.
Each limit is listed right under the verdict line.

## What GO does not mean

- **The image was not booted.** No QEMU run and no device are involved. The
  verdict says so in its `scope` field, every time. The boot gate is a
  separate step.
- **The loader and the images are not tied to the tree.** The loader
  (`BOOTX64.EFI`), the ESP's other files, and the USB and ISO images are
  checked only against the record, and the record is not signed. Only the
  kernel is tied to something trusted, through its signature. Every GO says
  this in its first limit.

  What someone who can write the release directory could do with this:
  - replace the loader, the ESP's other files (its trailer, approvals and
    `boot.cfg`), or the USB or ISO image;
  - rewrite the record and the receipt to match;
  - and still get GO.

  The kernel cannot be replaced this way. The ESP's kernel must be the signed
  kernel. A substituted loader, however, runs before any check the kernel
  makes: it decides what it loads and what it measures, and on a machine
  without Secure Boot, nothing stops it. So treat a GO as saying nothing
  about the loader or the images, and use media whose provenance you control
  in some other way.

  The planned fix is for the seal to sign the record and for this command to
  verify that signature. It is on the seal list for 0.9.4; until then this
  limit stands.
- **The source is the receipt's commit, not necessarily the newest one.**
  The verdict names the commit the receipt names and the tree's HEAD. An old
  commit from the tree's history passes if the tree holds it. The kernel's
  rollback index is what protects a device from an older image.
- **The verifiers are only as strong as the tree.** GO is only as good as
  the verifiers in the tree it runs.
- **A declared gap is a limit, not a pass.** A declared gap does not mean
  what it names was checked. Read the limits under the verdict line. A check
  that could not run is never a pass: it makes the verdict NO-GO, and so does
  a verifier that is missing.
- **A development GO exits 5, not 0.** It reads
  `GO (development image, not a release), …` and has
  `"kind": "development"`. Only a release's GO exits 0, so a script that
  deploys on exit status 0 alone can never deploy a development image.
  Whether the record says development or release is taken from the record,
  but `release.kind` fails a record that claims a release while carrying
  `nonos-dev-attest`.
- **A signature says who signed which bytes.** It does not make NO-GO into
  GO, and `verify-signature` does not report the verdict.

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

nonos-harden also states three limits of its own, fixed in its code:

- `release.sealed`: the parts only the unsigned record holds. This one is on
  every GO.
- `release.receipt`: a sealing tree that was dirty, or a receipt that
  measured no kernel attestation root. Real seals are always dirty, because
  the seal stages its trust set before it writes the receipt.
- `tree.source`: a development tree's own kernel keys.

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
| `release.sealed` | Every sealed artifact has the bytes the record names, and no file is over 64 GiB. A sealed directory holds exactly the listed files. Links are refused anywhere on the path, and files are read through one handle that does not follow a link. It carries the limit for what only the record holds. |
| `release.kind` | The record says release or development. If the record claims a release but carries `nonos-dev-attest`, that is a contradiction and the check fails. |
| `release.receipt` | `build-receipt.json` (`nonos.build.receipt.v1`) found its roots embedded. It measured no missing artifact, and everything it measured is among the sealed bytes. |
| `tree.source` | As in items 4 and 5 above. |
| `tree.ledger` | `nonos-data/trust/MANIFEST.sha256` names every file under the trust set's `capsules/`, `keys/` and `policy/`, each with its digest. No file there is missing from the ledger or added beside it. |
| `release.kernel-signature` | As in item 3 above. |
| `tree.build-inputs` | `python3 tools/nix/inputs.py --check` exits 0, within 15 minutes. |
| `verify.hygiene` | The tree's declared-limits list reads. Then every check in `nonos-verify hygiene`'s report passes, every gap it states is declared, and the tool exits 0, all within 1 hour. |
| `verify.trust-chain` | The same, for `nonos-verify trust-chain`. |
| `verify.supply-chain` | The same, for `nonos-verify supply-chain`. |
| `matrix.static-checks` | Every step of the verify matrix's `static-checks` row exits 0, within 2 hours. |
| `matrix.proof-crates` | Every step of the `proof-crates` row exits 0, within 10 hours. |

`nonos-verify` exits 0 even when a check could not run. So nonos-harden
reads the report the tool writes, not its exit status:

| report status | result |
|---|---|
| `fail`, or any status other than `pass`, `gap` or `skip` | the check fails |
| `gap` or `skip` | the check is not run |
| a report with no checks | the check is not run |

How each verifier is run:

- It runs in its own process group, with no input.
- Cargo builds into the run's own scratch directory (`CARGO_TARGET_DIR`), and
  Python caches into it too (`PYTHONPYCACHEPREFIX`), never into the tree.
- If it runs past its time limit, it is stopped, with everything it started.
- One that cannot start counts as not run.

Because cargo builds afresh each run, the matrix rows rebuild their crates
every time.

## The verdict format, `nonos-harden-verdict/1`

`--format json` prints the verdict as one JSON object, and `--out` writes it
the same way. If the meaning of any field changes, the format gets a new
number. It is never changed silently.

| field | meaning |
|---|---|
| `format` | always `"nonos-harden-verdict/1"` |
| `tool` | `"nonos-harden <version>"` |
| `overall` | `"go"` or `"no-go"` |
| `verdict` | the one line: `GO` or `GO (development image, not a release)`, followed by `, N declared limit(s)` when checks carry limits; or `NO-GO: …` |
| `kind` | `"release"`, `"development"` or `"unknown"` |
| `limits` | how many limits the checks carry, in all |
| `release` | the release directory, as an absolute path |
| `record_sha256` | the SHA-256 of `nonos-release.json`, or `null` if it was not read |
| `tree` | the tree checked against, as an absolute path, or `null` |
| `tree_head` | the tree's HEAD commit, or `null` |
| `signer` | the operator's Ed25519 public key in hex when the verdict is signed, else `null` |
| `scope` | what the verdict covers (host checks only; nothing booted) |
| `checks` | one object per check, in run order: `id`, `status` (`pass`, `fail`, `not-run`), `reason`, and `limits` (only when there are any) |

The text form holds the same facts, in this order:

1. one line per check
2. the verdict line
3. one `  limit: <check>: does not cover <what>` line per limit

Control characters and bidirectional marks in any reason are replaced with
U+FFFD, so a hostile file name cannot rewrite your terminal.

## Signing

```
nonos-harden check --release out/ --out verdict.json --format json --sign-key ~/my.key
nonos-harden verify-signature --verdict verdict.json --signature verdict.json.sig --public-key <hex>
```

- **Key forms.** `--sign-key` takes your Ed25519 key in one of three forms: a
  32-byte seed file, 64 hex digits, or a nonos-sign `NONOSSK1` seed file. The
  key is read and vetted **before** any check runs. It is read once, at most
  4 KiB of it, and the bytes read are cleared.
- **Refused keys.** The key is refused if it lies under `nonos-data/` or
  `nonos-bootloader/keys/`, or in the `.keys/` directory of a NONOS checkout.
  Links are followed first. It is also refused if its public key is any
  Ed25519 key published in the tree's `nonos-data/trust/keys/`, whether in a
  NONOSPK1 file or raw, as the kernel key is kept. Each published key file is
  read bounded and without following a link.
- **The signature file.** The signature (64 bytes, hex) is written to
  `FILE.sig` and covers the exact bytes of `FILE`. The verdict names the
  signer.
- **How files are written.** Each file is written into a new file created
  beside it under an unpredictable name, then renamed into place. Nothing is
  ever written through a name or link someone planted there.
- **If writing fails.** If the verdict or the signature cannot be written,
  nothing is delivered. No unsigned file is left behind, and the exit status
  is 3.
- **Verifying.** `verify-signature` checks the signature with strict
  verification. It also checks that the verdict names the given key as its
  signer.

## Exit status

| code | meaning |
|---|---|
| 0 | GO over a release (for `verify-signature`: the signature verifies) |
| 1 | NO-GO |
| 2 | bad command line, a signing key refused, or a tree inside the release |
| 3 | the verdict could not be written or signed |
| 4 | the signature does not verify |
| 5 | GO over a development image: every check passed, but it is not a release |

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
  `GO (development image, not a release), 1 declared limit` and exits 5. The
  release profile reads `GO, 1 declared limit` and exits 0.
- **One flipped byte.** The tree's own `tools/nonos-flip-byte` flips one byte
  in a copy of the release, and the copy reads NO-GO:
  - A byte flipped in the image names `release.sealed`.
  - A byte flipped in the kernel names both `release.sealed` and
    `release.kernel-signature`.
- **The adversarial review's attacks.** Each attack from the review is a
  proof, and each now fails as it should:
  - an unsigned kernel on the ESP;
  - kernel bytes changed after the record was written;
  - other kernel keys in a release tree;
  - a tree planted inside the release;
  - a file git ignores;
  - a rename into the trust set;
  - a skip-worktree flag;
  - a clean filter;
  - a changed executable bit;
  - a link in place of a tracked file;
  - a link planted beside the verdict;
  - a FIFO, a link or an oversized file where a file is read;
  - a grandchild process past the time limit.
- **Fuzzing (proptest).** Every parser of untrusted input gets arbitrary
  bytes and also mutated or cut copies of the genuine documents. The parsers
  covered are the record, the receipt, the ledger, the kernel image, the
  verifier report, the matrix summary, the key file, and the declared-limits
  list.
  - No parser panics.
  - A mutated record that is still accepted names only safe paths and
    well-formed digests.
  - No mutation of the kernel is accepted as another kernel or another
    rollback index.
  - Every cut of the kernel is refused.
- **Mutants.** Each line of `nonos-harden/mutants.json` breaks one rule and
  names the proof that catches it. `python3 nonos-harden/mutants.py` applies
  each mutant to a copy of the crate and checks that its proof fails. The crate
  itself is never changed. A separate proof checks that every mutant still
  applies exactly once.

### Mutant results

`python3 nonos-harden/mutants.py` was run on the crate as it lands. All 55 mutants are caught, and none escaped.

| mutant | file | caught by |
|---|---|---|
| `plan-skips-kernel-signature` | `src/plan.rs` | `plan_tests::every_check_runs_in_order_every_time` |
| `plan-skips-sealed` | `src/plan.rs` | `plan_tests::every_check_runs_in_order_every_time` |
| `plan-skips-proof-crates` | `src/plan.rs` | `plan_tests::every_check_runs_in_order_every_time` |
| `decide-not-run-counts-as-pass` | `src/verdict/decide.rs` | `verdict_tests::go_needs_every_check_run_and_passed_and_the_kind_known` |
| `decide-unknown-kind-is-go` | `src/verdict/decide.rs` | `verdict_tests::go_needs_every_check_run_and_passed_and_the_kind_known` |
| `decide-no-checks-is-not-refused` | `src/verdict/decide.rs` | `verdict_tests::go_needs_every_check_run_and_passed_and_the_kind_known` |
| `decide-dev-reads-plain-go` | `src/verdict/decide.rs` | `plan_go_tests::a_passing_development_release_reads_go_not_a_release_and_a_release_go` |
| `deliver-dev-go-exits-0` | `src/deliver.rs` | `plan_go_tests::a_passing_development_release_reads_go_not_a_release_and_a_release_go` |
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
| `source-allows-changes-outside-staging` | `src/tree/source.rs` | `source_tests::a_change_or_an_addition_outside_the_trust_set_fails_and_is_named` |
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
| `judge-accepts-undeclared-gap` | `src/tree/judge.rs` | `declared_tests::a_gap_the_tree_does_not_declare_is_no_go` |
| `judge-gap-declared-for-any-module` | `src/tree/judge.rs` | `declared_tests::a_gap_declared_for_one_module_is_undeclared_for_another` |
| `verify-missing-list-reads-as-empty` | `src/tree/verify.rs` | `declared_tests::without_a_declared_list_in_its_format_no_verifier_passes` |
| `decide-go-line-hides-one-limit` | `src/verdict/decide.rs` | `limits_tests::what_the_unsigned_record_alone_holds_is_a_limit_on_every_go` |
| `decide-go-line-hides-many-limits` | `src/verdict/decide.rs` | `limits_tests::a_go_over_a_declared_gap_counts_and_lists_it_under_the_verdict` |
| `text-does-not-list-limits` | `src/verdict/text.rs` | `limits_tests::a_go_over_a_declared_gap_counts_and_lists_it_under_the_verdict` |
| `kernel-verified-bytes-not-bound` | `src/release/kernel_bind.rs` | `kernel_bind_tests::a_kernel_changed_after_the_record_was_written_fails` |
| `kernel-esp-copy-not-bound` | `src/release/kernel_bind.rs` | `kernel_bind_tests::an_esp_kernel_other_than_the_signed_one_fails_though_the_record_agrees` |
| `kernel-rollback-optional` | `src/release/kernel_check.rs` | `kernel_bind_tests::a_record_without_an_esp_kernel_or_a_rollback_index_fails` |
| `footer-reserved-bytes-unchecked` | `src/release/kernel.rs` | `kernel_footer_tests::reserved_footer_bytes_not_as_embed_trailer_writes_them_are_refused` |
| `source-blob-hash-ignored` | `src/tree/blob_same.rs` | `source_tests::a_change_or_an_addition_outside_the_trust_set_fails_and_is_named` |
| `source-exec-bit-ignored` | `src/tree/blob_same.rs` | `source_hide_tests::a_changed_executable_bit_or_a_link_in_place_of_a_file_is_a_change` |
| `source-untracked-files-ignored` | `src/tree/untracked.rs` | `source_hide_tests::a_file_git_ignores_is_still_a_file_the_commit_does_not_hold` |
| `source-release-accepts-other-kernel-keys` | `src/tree/source.rs` | `source_keys_tests::a_release_tree_with_other_kernel_keys_than_its_commit_fails` |
| `choose-allows-tree-inside-release` | `src/tree/mod.rs` | `tree_choice_tests::a_tree_at_or_inside_the_release_is_refused` |
| `sealed-drops-unbound-limit` | `src/release/sealed.rs` | `limits_tests::what_the_unsigned_record_alone_holds_is_a_limit_on_every_go` |
| `write-uses-a-predictable-partial-name` | `src/write.rs` | `fs_tests::a_link_planted_beside_the_verdict_is_not_written_through` |
| `read-follows-a-final-link` | `src/release/read.rs` | `fs_tests::a_fifo_a_link_or_an_oversized_file_is_refused_at_once` |
| `runner-kills-only-the-child` | `src/tree/system.rs` | `runner_group_tests::a_process_the_verifier_started_is_stopped_with_it` |
| `runner-builds-in-the-trees-target` | `src/tree/system.rs` | `runner_group_tests::cargo_and_python_write_into_the_runs_own_space` |
| `choose-keeps-a-relative-tree` | `src/tree/mod.rs` | `boot_gate_tests::a_relative_release_and_tree_reach_every_verifier_as_absolute_paths` |
| `kernel-ed-key-any-length` | `src/release/kernel_key_files.rs` | `kernel_key_tests::a_key_file_of_the_wrong_kind_is_refused` |
| `refusal-misses-the-raw-kernel-key` | `src/sign/refuse.rs` | `boot_gate_tests::the_raw_kernel_key_cannot_sign_a_verdict` |
