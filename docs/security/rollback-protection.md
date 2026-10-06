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

## Where the floor lives

The [rollback floor](../overview/glossary.md#rollback-floor) is kept in TPM NV, and it is not the counter itself. It is how far the counter at `0x01000020` has risen above a base at `0x01000021`, the counter's value when this machine's floor began, as `ROLLBACK_INDEX` and `BASE_INDEX` say (`src/security/tpm/boot_reads/floor.rs:17-32`). A TPM starts every new counter above the highest value any counter on it has held, even after `TPM2_Clear`, so a counter alone would start high on a used TPM; the base makes the floor start at 0, as `a_tpm_whose_counters_counted_starts_the_floor_at_zero_through_a_clear` shows against swtpm (`userland/tpm_enroll_proofs/src/security/tpm/live/floor_tests.rs:44-67`).

The loader defines both indices, writes the base once and raises the counter. The kernel only reads them, each with `TPM2_NV_Read` under an empty password, eight bytes from offset 0, in `read` (`src/security/tpm/boot_reads/floor.rs:37-56`). `rollback_floor` returns 0 for a counter never incremented, and an error for a counter without its base or a base above its counter (`src/security/tpm/boot_reads/floor.rs:58-66`).

The loader's own NV code sits in its verification module, which these pages do not describe. Its behaviour is pinned by host tests in `nonos-bootloader/boot_proofs` that run it against a scripted TPM:

- On a new TPM the loader defines the counter, increments it once, defines the base, writes the counter's value into it and write-locks it; the floor is 0 (`a_new_tpm_starts_the_floor_at_zero_and_locks_its_base`, `nonos-bootloader/boot_proofs/src/floor_seq_tests.rs:35-44`).
- A TPM whose counters already stood high still starts at 0, after a `TPM2_Clear` too (`a_tpm_whose_counters_counted_still_starts_at_zero`, `nonos-bootloader/boot_proofs/src/floor_seq_tests.rs:46-58`).
- A counter that is deleted and defined again reads one above its old floor after `undefine`, never 0 (`nonos-bootloader/boot_proofs/src/floor_seq_tests.rs:77-86`).
- A base that is deleted is set again from the counter, and the floor starts again at 0 (`undefine_base`, `nonos-bootloader/boot_proofs/src/floor_seq_tests.rs:88-94`).
- A locked base takes no write, as `base_write` against it shows (`nonos-bootloader/boot_proofs/src/floor_seq_tests.rs:96-103`).
- A base above its counter, or one that could not be written, gives no floor (`fail_base_write`, `nonos-bootloader/boot_proofs/src/floor_seq_tests.rs:105-111`).
- Any other TPM answer to the read gives no floor and increments nothing (`floor_with`, `nonos-bootloader/boot_proofs/src/floor_seq_tests.rs:122-131`).
- Raising increments the counter until the floor reaches the target and never lowers it; a failed increment reports the floor as not raised (`raise_with`, `nonos-bootloader/boot_proofs/src/floor_raise_tests.rs:20-49`).

`tpm_enroll_proofs` runs the same sequence against a software TPM, including a raise to 5 and a deleted counter that comes back at 6 (`floor_with`, `userland/tpm_enroll_proofs/src/security/tpm/live/floor_tests.rs:31-42`). In this release's flake checks, `proofs-boot_proofs` passed 39 tests and `proofs-tpm_enroll_proofs` passed 37, the second against swtpm; that check fails when a live TPM test is skipped (`needsTpm`, `tools/nix/checks.nix:32-34`).
