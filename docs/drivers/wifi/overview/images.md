## Which images carry them

The full image, the one `make` builds, adds both Wi-Fi drivers to the desktop (`Cargo.toml:627-638`, `microkernel-full-gui`). The `qemu` [build profile](../../../overview/glossary.md#build-profile), `make PROFILE=qemu`, builds the desktop without them (`tools/nix/config.nix:94-98`, `qemu`), and the air-gapped profile drops every network driver (`tools/nix/config.nix:86-93`, `networkFeatures`). The hardened profile keeps them but drops the [serial console](../../../overview/glossary.md#serial-console) for capsules, so neither driver writes a log line there (`tools/nix/config.nix:78-85`, `debugFeatures`).

## Firmware and its licence

Both drivers link their firmware into the capsule with `include_bytes!`, so no filesystem access is needed at boot (`userland/capsule_driver_rtl8821ce/src/fwload.rs:38`, `include_bytes`). The files sit in [nonos-bootloader/firmware/realtek/](../../../../nonos-bootloader/firmware/realtek/) and [nonos-bootloader/firmware/intel/](../../../../nonos-bootloader/firmware/intel/). They are vendor binaries from the linux-firmware project, not AGPL code. Their licences allow binary redistribution without modification, with the notice kept, and forbid reverse engineering: read [the Realtek licence](../../../../nonos-bootloader/firmware/realtek/LICENSE) and [the Intel licence](../../../../nonos-bootloader/firmware/intel/LICENSE).

## Tests on this commit

The flake runs each proof crate with `cargo test --release`, overflow checks on, and then clippy with warnings denied (`tools/nix/checks.nix:85-94`, `testArgs`, `clippy`). On this commit:

| Check | Result | What it covers |
|---|---|---|
| `proofs-rtl8821ce_proofs` | passed, 171 tests | the RTL8821CE driver against a modelled register file |
| `proofs-iwlwifi_proofs` | passed, 205 tests | the iwlwifi driver against a modelled device |
| `proofs-nonos_wifi_core_proofs` | passed, 70 tests | RSN, SAE vectors, handshakes against a simulated access point, receive checks |
| `proofs-wifi_panel_proofs` | passed, 23 tests | the panel flow, the join wire format, the saved list |
| `proofs-net_core_proofs` | passed, 31 tests | autojoin timing, the link watch, the DHCP lease wait |

To run one yourself:

```sh
cd userland/nonos_wifi_core_proofs && cargo test --release --config profile.release.overflow-checks=true
```

Not tested in this release.
