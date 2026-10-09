# wifi_panel_proofs

Host-runnable proofs for the Wi-Fi section of Settings: the detect,
select, enter passphrase and connect flow and the scan and connect wire
format, driven against crafted scan buffers with no device and no
renderer. The panel and wire source, the client's join and link messages
and its saved-network list are included through `#[path]`, and
`../nonos_wifi_client/src/wipe.rs` with them.

| Tests | What they hold |
|---|---|
| `panel_tests` | detection and selection through the panel |
| `join_tests` | the join and link messages |
| `saved/saved_tests.rs` | the saved-network list, including the flags that keep a WPA3 network off WPA2 |

Run: `cargo test` in this directory. Settings is described in
[System apps and services](../../docs/handbook/apps/system-apps.md).
