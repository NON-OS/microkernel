# Ethernet drivers: images and tests

Which build carries which Ethernet driver, and what the host tests show on this commit. The drivers themselves are listed on the [Ethernet drivers](README.md) page.

## Which image carries which driver

- The desktop base, which every desktop image builds on, carries virtio-net (`mk/20-build.mk:1115-1118`, `DESKTOP_BASE_SLUGS`).
- The full image that `make` builds adds e1000, RTL8139 and RTL8169, with the two Wi-Fi drivers (`Cargo.toml:627-638`, `microkernel-full-gui`).
- The air-gapped [build profile](../../overview/glossary.md#build-profile) drops every network driver and the whole stack (`tools/nix/config.nix:86-93`, `networkFeatures`).
- `nonos-mk-ethernet-prod` builds the desktop with the three wired drivers (`mk/20-build.mk:1166-1175`, `ETHERNET_DRIVER_ARTIFACTS`). No QEMU run target attaches an e1000 or RTL8139 device, so this release has no QEMU run for them.
- The build includes the capsule makefiles of virtio-net, e1000, RTL8139 and RTL8169 and of no other Ethernet driver (`mk/20-build.mk:540-550`, `capsule_driver_e1000`). The e1000e, igc and USB capsules are therefore in no image.

## Tests on this commit

Each [proof crate](../../overview/glossary.md#proof-crate) compiles its driver's own source with `#[path]` and runs it on the host against models. The flake runs `cargo test --release` with overflow checks on, then clippy with warnings denied (`tools/nix/checks.nix:85-94`, `testArgs`, `clippy`). `net_core_proofs` now runs `net.core`'s receive-batch probe against drivers that answer operation 6 with counters, the exchange where [the receive fault](README.md#the-receive-fault) sat; no test runs a driver and `net.core` together.

| Check | Result |
|---|---|
| `proofs-e1000_proofs` | passed, 20 tests |
| `proofs-e1000e_proofs` | passed, 48 tests |
| `proofs-igc_proofs` | passed, 58 tests |
| `proofs-rtl8139_proofs` | passed, 15 tests |
| `proofs-rtl8169_proofs` | failed: the tests pass, then a clippy lint stops the check; see [the RTL8169 family](realtek-rtl8169.md#tests) |
| `proofs-virtio_net_proofs` | passed, 20 tests |
| `usbnet_proofs` | 38 tests pass under `cargo test` on the host |
| `cdc_ecm_proofs` | 13 tests pass under `cargo test` on the host |
| `cdc_ncm_proofs` | 36 tests pass under `cargo test` on the host |
| `rndis_proofs` | 22 tests pass under `cargo test` on the host |
| `ax88179_proofs` | 27 tests pass under `cargo test` on the host |
| `rtl8153_proofs` | 31 tests pass under `cargo test` on the host |
| `net_core_proofs` | 33 tests pass under `cargo test` on the host |

The last seven crates changed on this commit; their rows are host `cargo test` runs, and their flake checks have not been run on it.

## See also

- [Ethernet drivers](README.md)
- [How a frame travels](frame-path.md)
- [USB networking](usb-net.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
