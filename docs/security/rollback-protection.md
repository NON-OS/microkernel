# Rollback protection

NONOS keeps an older signed kernel, an older bootloader tree and a certificate from an older trust anchor from coming back, each by holding a signed number to a floor that NONOS itself never lowers.

## What holds each floor

| What could come back | The number | Where the floor is kept | Who compares |
|---|---|---|---|
| an older signed kernel | the kernel's rollback index, signed into the image | a TPM NV counter | the loader, before the jump |
| an older bootloader tree | the epoch in the boot-root record | the same TPM NV counter | the kernel, at boot |
| a certificate from an older trust anchor | the certificate's trust-anchor epoch | the trust-anchor policy compiled into the kernel | the kernel, at every capsule spawn |
