# Rollback protection

NONOS keeps an older signed kernel, an older bootloader tree and a certificate from an older trust anchor from coming back, each by holding a signed number to a floor that NONOS itself never lowers.

## What holds each floor

| What could come back | The number | Where the floor is kept | Who compares |
|---|---|---|---|
| an older signed kernel | the kernel's rollback index, signed into the image | a TPM NV counter | the loader, before the jump |
| an older bootloader tree | the epoch in the boot-root record | the same TPM NV counter | the kernel, at boot |
| a certificate from an older trust anchor | the certificate's trust-anchor epoch | the trust-anchor policy compiled into the kernel | the kernel, at every capsule spawn |

## The kernel's rollback index

The [rollback index](../overview/glossary.md#rollback-index) is signed: `signed_message` puts it after the kernel's BLAKE3 hash, so it cannot change without breaking both signatures (`nonos-bootloader/tools/sign-kernel/src/message.rs:17-22`). A copy sits in the image footer at offset 56 as `rollback_index` (`nonos-bootloader/tools/embed-trailer/src/footer/create.rs:42-43`). `check_rollback` gates on that field and not on the footer's `image_version`, because the image version is not signed and a replayed kernel could carry any value there (`nonos-bootloader/src/boot/crypto/rollback/check.rs:23-38`).

The number comes from `rollback_index` in `nonos.toml`, 1 by default; the comment there says to raise it only for a security release (`nonos.toml:27-29`). The flake refuses a `rollback_index` below 1 (`tools/nix/config.nix:162`). The make rules use `NONOS_ROLLBACK_INDEX`, also 1 by default (`mk/00-config.mk:111-115`), and sign the kernel again when it changes, through `NONOS_ROLLBACK_STAMP` (`mk/00-config.mk:117-119`). `sign-kernel` run by hand defaults its `rollback_index` to 0 (`nonos-bootloader/tools/sign-kernel/src/args.rs:45-46`).
