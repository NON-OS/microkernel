# nonos_wifi_client

`nonos_wifi_client` is the one Wi-Fi client that Settings, first-boot setup and
`net.core` share. It finds whichever Wi-Fi driver service is running, speaks
the control protocol both drivers answer, and keeps the networks a person chose
to remember, sealed under a TPM-derived key, in the NONOS store. It is
`no_std` and runs no radio itself: the 802.11 work is in the drivers and in
`nonos_wifi_core`.

## Finding a driver

`find()` returns the first registered service in `SERVICES`
(`src/driver/services.rs`): `driver.rtl8821ce0` (Realtek RTL8821CE), then
`driver.iwlwifi0` (Intel Wi-Fi). The RTL8821CE comes first so a machine with
both cards keeps the radio it joined with. Both run joins; `Driver::joins` says
so, and a driver that cannot join is never sent a passphrase. The iwlwifi
driver answers a join with `CANNOT_JOIN` on a part it has no air path for, or
without randomness.

The kernel lets only `net.core`, Settings (in any of its windows) and the setup
wizard reach a Wi-Fi driver (`src/services/registry/held.rs`).

## Talking to it

- `scan` returns the networks heard, as `ScanNetwork` entries.
- `connect(ssid, pass)` joins under the flags the saved list holds for the
  network; `connect_with` takes the flags from a caller that already holds the
  list (`net.core`'s autojoin). A join waits up to 20 s. The request body
  holding the passphrase is wiped before the call returns.
- `link` reports association, BSSID, SSID and, from a driver that runs WPA3,
  the AKM that ran.
- `join_text` turns a join result into a sentence for the panels.

The join and link messages are encoded and parsed in `src/join_wire.rs`. A join
body is `[ssid_len][ssid][pass_len][pass][flags]`. Flag bit 0 says the network
was saved as WPA3, so the driver joins it with SAE or not at all, never WPA2;
bit 1 says it is hidden, so the driver may name it in a probe request.

## Saved networks

- Up to `SLOTS`, four, networks, each with a passphrase of at most
  `PASS_MAX`, 64, bytes.
- The list is one file, `/nonos/wifi/saved`, sealed with an AEAD under a key
  from `machine_key("wifi/saved-networks")`. The key comes from the TPM and the
  boot state, and is wiped after each use. With no TPM, or under a changed boot
  state, the list cannot be opened (`SavedError::NoTpm`,
  `SavedError::BootChanged`), and `remember` replaces a record this machine can
  no longer open.
- `remember` saves a network the radio joined with WPA3-SAE as WPA3, so no
  later join accepts WPA2 for it. `remember_with` saves a hidden network.
- `forget` and `forget_all` remove entries; forgetting the last writes the
  withdrawn record, which vfs accepts in any mode.
- `keeps_state` and `sealing_ready` say whether this boot may save at all.

## Capabilities

A library adds no capability of its own. A capsule that loads or saves the list
needs FileSystem, since vfs serves only a holder of it; `net.core` holds it for
this alone.

## Tests

`userland/wifi_panel_proofs` and `userland/capsule_settings_proofs` include
`src/join_wire.rs`, `src/network.rs` and `src/wire.rs` by `#[path]` and run
them on untrusted bytes.

See [the network stack](../../docs/handbook/network/stack.md) and
[drivers](../../docs/handbook/drivers.md).
