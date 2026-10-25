# Tests and proofs: State of the checks at this commit

Part of [Tests and proofs](tests-and-proofs.md).

## State of the checks at this commit

The flake defines 143 checks for `x86_64-linux` at this commit. In a run of all
of them on x86_64 Linux, `proofs-model_fetch_proofs` did not evaluate, so it was
not built; the `inputs` failure below names the same crate. The other 142 were
built and 133 passed, among them every `kernel-features-*` and
`kernel-profile-*` check and 110 of the 113 `proofs-*` checks. Nine failed:

| Check | What failed |
|---|---|
| `inputs` | `tools/nix/inputs.json` is stale for `userland/capsule_market`, `userland/capsule_model_fetch` and `userland/model_fetch_proofs` |
| `static-abi` | `scripts/check_prebuilt.py` finds two binaries it cannot classify: `nonos-data/market/index.bin` and `nonos-data/models/catalogue.bin`. The scripts after it in the list did not run |
| `static-hygiene` | `scripts/check_stubs.py` finds 15 admissions that are not in its [baseline](../overview/glossary-amnesic-boot.md#baseline). The scripts after it did not run |
| `static-tree` | several gates: the `cfg(target_arch` count is 234 against a baseline of 116, the `crate::arch::x86_64::` count 135 against 100, and the end of its log shows the forbidden `read` and `write` import at `userland/capsule_driver_ahci/src/server/handlers/emmc/dispatch.rs:29` |
| `nonos-verify` | clippy passes, then the `hygiene` scan fails. It writes its findings to a file, so the log ends as the scan starts. Its comment patterns match shipping comments such as the "for now" in `src/hardware/inventory/family.rs` |
| `proofs-rtl8169_proofs` | its 67 tests pass; clippy's `manual_div_ceil` lint fails on `userland/capsule_driver_rtl8169/src/log/line.rs` |
| `proofs-usb_msc_proofs` | clippy's `new_without_default` lint fails on `new` at `userland/capsule_driver_usb_msc/src/state/types.rs:38` |
| `proofs-xhci_proofs` | clippy's `int_plus_one` and `assertions_on_constants` lints fail in two test files of the crate |
| `busybox-source` | failed; its log kept no lines |

`scripts/check_allows.py` runs after `check_stubs.py` inside `static-hygiene`,
so the failure above stops it. Run on its own against this tree, it passes its
self-test and then reports 31 lint switches that are not in its baseline.
