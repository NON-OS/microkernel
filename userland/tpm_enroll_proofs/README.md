# tpm_enroll_proofs

Host test crate for the TPM side of device enrollment and the device secret.
It compiles the kernel's TPM wire code, its `MkEnroll` decoding and dispatch
(everything but the copy to and from user memory), libc's framing of the same
calls and the bootloader's rollback counter commands through `#[path]`, so the
bytes under test are the bytes that ship. The TPM and the device proof are
described in [docs/handbook/trust/tpm.md](../../docs/handbook/trust/tpm.md)
and [docs/handbook/trust/device-proof.md](../../docs/handbook/trust/device-proof.md).

## What the tests check

35 `#[test]` functions:

- Byte tests: the `MkEnroll` request decoding and libc's frames against the
  kernel's (`syscall/microkernel/enroll/`), and the parsing of TPM responses,
  NV reads, hashes and signatures (`security/tpm/parse*_tests.rs`).
- Live tests against a software TPM 2.0 on TCP (`security/tpm/live/`): the
  endorsement key and its certificate (RSA and ECC), credential activation,
  quotes, signing, command resend, the device secret, and the loader's
  rollback counter read.

The byte tests cannot show whether a real TPM takes each command or whether a
registrar's credential unwraps; the live tests do. Without `swtpm` they pass
and print that they were skipped.

## Running

```sh
cd userland/tpm_enroll_proofs
cargo test --release
```

CI runs it through `nix flake check` (`tools/nix/checks.nix`) on Linux only,
with `swtpm`, `tpm2-tools`, `openssl` and `gnutls` present; a log line saying
a live test was skipped fails the check.
