# acpi_aml_proofs

Host tests for the kernel's ACPI AML resource extractor: the AML scanner, the
`_CRS` decoder and the controller lookup under `src/arch/x86_64/acpi/aml`, and
the SDT entry-count helper, mounted by `#[path]` in a directory tree that
mirrors the kernel's so their `crate::` paths resolve unchanged. The
firmware-facing `enumerate` and `tables` modules are left out; the pure parsers
are what is tested, against fixtures in `src/fixtures.rs`, plus the
HID-over-I2C enumeration in `src/arch/x86_64/acpi/devices/i2c` (`_HID`/`_CID`,
`_DSM`, interrupt and bus speed).

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-acpi_aml_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
