# NONOS bootloader

The NONOS bootloader is a UEFI loader for the signed kernel image. Before it
jumps, it checks three things:

- the kernel image's Ed25519 and ML-DSA-65 signatures, both required. A bad or
  missing signature stops the boot in every mode that requires signatures and
  only warns in development;
- the anti-rollback index, against the TPM monotonic counter. Hardened and
  Air-Gapped stop when no counter can be read; the other profiles boot with
  rollback protection off and say so;
- the kernel's self-attestation, in every mode, development included: the v4
  trailer in the image footer must fold from this kernel's measurement to the
  enrolled kernel root compiled into the loader (`nonos-attest-path`), and its
  STARK proof of the same slot must verify with `nox_verify` from STARKs.

On admission it extends PCR 9 with the kernel's BLAKE3 measurement and the root
it was admitted under, and hands the verdict, the root and the release approval
to the kernel in the boot handoff. It also hands over its own trailer, the
boot-root record and the TCG log, so the kernel can check the loader in turn.

The enrolled root comes from `nonos-stark-enroll kernel`; the build fails if a
non-development loader is built without one. `tools/sign-kernel` signs the
kernel and `tools/embed-trailer` carries the enrolled trailer into the signed
image. `nox_verify` follows STARKs main; the commit a build uses is the
flake's `starks` input in `flake.lock`.

The loader is built by the flake (`nix build`), which compiles in the kernel's
public signing keys from `nonos-data/trust/keys/` once the seal has written
them, or by `make nonos-mk-bootloader` in the flake's shell, with
`BOOTLOADER_POLICY` choosing the profile (default `standard-qemu`). The top of
the root `Makefile` and `tools/nix/README.md` list the entry points.

Internals: [docs/handbook/bootloader.md](../docs/handbook/bootloader.md).
The boot in order: [docs/handbook/boot.md](../docs/handbook/boot.md).
