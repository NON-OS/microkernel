## Reading what the driver did

On a build that grants Debug, the driver writes one [serial console](../../../overview/glossary.md#serial-console) line per bring-up step through `mk_debug` (`userland/capsule_driver_rtl8821ce/src/status.rs:32-33`, `line`). The standard profile keeps the `capsule-serial-debug` feature that grants it, and the hardened profile drops it (`tools/nix/config.nix:69-85`, `debugFeatures`). The Terminal's `log` command shows the kernel's recent lines and keeps only those naming the words you give (`userland/capsule_terminal/src/command/builtin/log.rs:18-21`, `mk_log_tail`):

```sh
log rtl8821ce
```

Not tested in this release.

Without Debug those lines are dropped, and the Settings Wi-Fi panel's stage line is the record.

## Tests

`rtl8821ce_proofs` compiles the driver's own source with `#[path]` and runs it against a modelled register file: power sequence, efuse, firmware staging and download, MAC and PHY tables, the rings, the scan, the hidden-network probe, association, key install and the refusal of unknown frames. The flake check `proofs-rtl8821ce_proofs` passed with 171 tests on this commit, and `proofs-nonos_wifi_core_proofs` with 70.

```sh
cd userland/rtl8821ce_proofs && cargo test --release --config profile.release.overflow-checks=true
```

Not tested in this release.
