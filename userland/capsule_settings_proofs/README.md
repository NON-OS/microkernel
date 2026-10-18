# capsule_settings_proofs

Host test crate for the WiFi settings panel and the saved-network record it relies on. It
compiles the panel and WiFi client source through `#[path]` and drives it with synthetic
input, with no WiFi hardware present. It depends on `userland/nonos_seal` for sealing
(`Cargo.toml`).

## What is under test

From `src/lib.rs`, `src/wifi/mod.rs` and `src/saved/mod.rs`:

- `../../capsule_settings/src/wifi/interface.rs` as `interface`: adapter discovery.
- `../../../nonos_wifi_client/src/network.rs` and `.../wire.rs` as `wifi::network` and
  `wifi::wire`: the network model and the scan/connect wire codec.
- `../../../nonos_wifi_client/src/saved/{error,file,list,list_codec}.rs` as `saved`, and
  `../../nonos_wifi_client/src/wipe.rs` as `wipe` (test builds only).

Per the comments in `src/lib.rs`, adapter enumeration (`adapters`) is left out because it
calls the broker, and the vfs and TPM halves of the saved record are left out because they
are syscalls.

## What the tests check

- `interface_tests.rs`, `interface_names_tests.rs`, `interface_driver_tests.rs`: WiFi is recognised by PCI class, not vendor; wired NICs and non-PCI
  devices are excluded; `discover` does not overrun its output buffer; known parts get
  specific names (`recognises_wifi_by_class_not_brand`,
  `discover_never_overruns_the_output_buffer`).
- `wire_tests.rs`, `wire_ssid_tests.rs`, `connect_request_tests.rs`: the parser for untrusted scan bytes refuses a lying count, truncated
  entries and oversized or overrunning SSID lengths (`a_lying_count_stops_at_the_real_data`,
  `an_ssid_running_past_the_buffer_is_refused`).
- `saved/tests.rs`, `saved/seal_tests.rs`: slot replacement and eviction, a sealed record opening only under its key,
  and decode refusing lengths no slot was written with
  (`a_sealed_record_opens_under_its_key_only`).
- `screens_tests/keep_tests.rs` and `replay_tests.rs`: a change on the displays' rows is kept
  under the approved field ids, and the shell's replay (`capsule_desktop_shell/src/state/
  display_keep.rs`, `display_told.rs`) sends back the very bytes Settings sent live, once per
  change in the store and again for a new compositor, never a value out of range.

## Running

```sh
cd userland/capsule_settings_proofs
cargo test --release
```

At the time of writing this runs 31 tests. This crate is not named in
`.github/workflows/verify.yml`, so CI does not run it.

## Not covered

Adapter enumeration through the broker, the WiFi service itself, vfs persistence, TPM key
derivation, and panel painting are not exercised. Other settings panels in `capsule_settings`
are not mounted.
