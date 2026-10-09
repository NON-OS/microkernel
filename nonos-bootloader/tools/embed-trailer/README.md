# embed-trailer

Host tool that carries the kernel's STARK self-attestation trailer into a signed
kernel image. It runs after `sign-kernel` and before the image goes on the ESP.

## What it does

`embed-trailer --input kernel_signed.bin --output kernel_attested.bin --proof-file kernel.zk_trailer.bin`

- reads the signed image and its 64-byte `NONOSIMG` footer, and requires the
  signature bundle to start right after the kernel;
- reads the trailer `nonos-stark-enroll kernel` wrote for this kernel, and
  refuses any file that does not start with the v4 magic `NATTV4`
  (`nonos_attest_path::MAGIC_V4`);
- writes a new image: the kernel, the signature bundle, the trailer, then a new
  footer with `FLAG_HAS_ZK_PROOF` set and the proof offset and size filled in.
  The rollback index and signature algorithm are copied from the signed image.

The trailer is carried byte for byte. The tool does not check it against the
kernel; the bootloader does, path and STARK both, before it jumps.

The make rule builds `kernel_attested.bin` from `kernel_signed.bin`; the seal
(`nix run .#seal`) runs the same step on the kernel the flake built.

## Build and test

The tool is its own Cargo workspace. From this directory:

```
cargo build --release
cargo test --release --test kernel_self_attest_poc
```

`tests/kernel_self_attest_poc.rs` runs the chain on the host: enroll kernel
bytes with `nonos-attest-path`, embed the trailer, parse the footer back and
fold the path as the loader does. It then tries a flipped kernel byte, a
foreign kernel with a stolen trailer, the genuine path for a foreign kernel, a
trailer under another root, and an old STARK trailer, and expects each to be
refused. It checks the path only, not the STARK proof. `nix flake check` runs
it as `attest-poc`.

## Related

- [The bootloader's host tools](../../../docs/handbook/bootloader.md)
- [The STARK layer](../../../docs/handbook/trust/stark.md)
- [Signing](../../../docs/handbook/trust/signing.md)
