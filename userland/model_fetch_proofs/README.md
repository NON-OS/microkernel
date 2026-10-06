# model_fetch_proofs

Host test crate for two pieces that bring a Qwen model onto a NONOS machine: the model fetcher
(`userland/capsule_model_fetch`, `tool.model-fetch`) and the Linux personality's install of a
shipped Qwen tier (`userland/capsule_linux/src/linux/install/`). It compiles the shipped source
files through `#[path]`, so a test holds the code as it ships and not a copy of it. The Cargo
package is `nonos_model_fetch_proofs`.

## What is mounted

- `catalogue/`: the fetcher's catalogue reader (`capsule_model_fetch/src/catalogue/parse.rs`,
  `read.rs`, `types.rs`) and its pins (`capsule_model_fetch/src/pins/`).
- `net/`: `nonos_route_link/src/pick.rs`, the rule the fetcher and every capsule holding
  Network use to choose the network a download leaves through.
- `install/`: from the personality, the tier tables (`install/apps.rs`, `apps_qwen25.rs`,
  `apps_qwen3.rs`, `apps_coder.rs`), the model dependency's rules (`install/model_dep.rs`),
  the installer's exit reasons (`install/why.rs`) and the family split (`file/family.rs`);
  the fetcher's exit statuses (`capsule_model_fetch/src/exit.rs`); and the kernel's naming of a
  listing (`src/userspace/capsule_linux/family.rs`).
- `store/`: how the App Store reads and words an install's status
  (`capsule_app_store/src/store/progress.rs`, `progress_text.rs`, `theme.rs`).

## What the tests check

- `tests.rs`: a catalogue written by `tools/nonos-qwen-tier.py`, under a key made for the test,
  reads back through the fetcher's parser with its Ed25519 signature verifying and every entry
  equal to a pin (`the_host_tool_writes_what_the_fetcher_takes`); every pin decodes to a
  digest (`every_pin_decodes_to_a_digest`). The test runs the Python tool, so it needs
  `python3` on the host.
- `route_tests.rs`: the download leaves through the network the person chose, or none; a
  network that is not running is no route and nothing falls back
  (`a_network_that_is_not_running_is_no_route_and_nothing_falls_back`).
- `install_tests.rs`: a `qwen-` listing is a shipped tier, its program and a model tier, never
  a package (`a_qwen_listing_is_its_program_and_a_model_tier_not_a_package`); a name no tier
  has is refused by name; a model already held under its pin is not fetched again; a model
  whose bytes are not its pin is never installed; no data volume (ENODEV, no retry), a volume
  that failed (EIO or EAGAIN, a retry) and no network are told apart; each reason reads in the store as its own sentence; a model the volume cannot keep is
  not offered a retry.

## Running

```sh
cd userland/model_fetch_proofs
cargo test --release
```

The crate holds 13 `#[test]` functions. `nix flake check` runs it as the check
`proofs-model_fetch_proofs` (`tools/nix/checks.nix`), with clippy over all targets and
`-D warnings`.

## Not covered

The fetcher's downloads, its IPC to the network services and the kernel's model import, and
the personality's reads of the store and the data volume, run only on a machine. What is held
here is every rule those steps apply to their answers.

Handbook: [Linux personality](../../docs/handbook/linux/personality.md),
[local AI](../../docs/handbook/apps/local-ai.md).
