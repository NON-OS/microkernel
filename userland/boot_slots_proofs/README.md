# boot_slots_proofs

Host test crate for `MkBootSlots`, held from three sides: the kernel's footer
reader against the bootloader's own parser, the slots against the enroll
tool's tree, and the kernel's record against libc's reader. It compiles the
kernel's `src/security/boot/slots/` files (`footer.rs`, `record.rs`,
`slot.rs`), the bootloader's `image_format` parser and libc's
`boot_slots/record.rs` through `#[path]`, unchanged, and links
`nonos-attest-path` and `nonos-boot-measure`. The boot slots feed the
anonymous device proof described in
[docs/handbook/trust/device-proof.md](../../docs/handbook/trust/device-proof.md).

## What the tests check

12 `#[test]` functions under `src/slots/`: the footer the kernel reads is the
one the bootloader parses, on signed files and on mutated ones
(`footer_tests.rs`, `footer_mutation_tests.rs`); each slot matches the
enrollment tree (`slot_tests.rs`); and libc reads the kernel's record as the
kernel wrote it (`record_tests.rs`).

## Running

```sh
cd userland/boot_slots_proofs
cargo test --release
```

CI runs the tests through `nix flake check` (`tools/nix/checks.nix`); the crate
is in the `lintNone` list, so clippy is not run on it.
