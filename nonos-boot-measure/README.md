# nonos-boot-measure

The kernel's check of the bootloader that started it, as a `no_std` crate the
kernel links. It also lends the bootloader its TCG log reader and the enroll
tool its Authenticode digest, so all three read the same bytes the same way.
The check is described on [the STARK layer page](../docs/handbook/trust/stark.md#the-kernels-check-of-the-bootloader)
and the TPM side on [the TPM page](../docs/handbook/trust/tpm.md).

## What is in it

- `authenticode`: the PE Authenticode SHA-256 of a loader image, the digest
 the firmware extends into PCR 4. `nonos-stark-enroll bootloader` enrolls this
 digest.
- `tcg`: the TCG event log. `replay` extends a zero PCR 4 with every PCR 4
 event's SHA-256 in log order, skipping `EV_NO_ACTION`, and keeps the last
 `EV_EFI_BOOT_SERVICES_APPLICATION` digest. The loader uses the walkers to copy
 the log for the kernel.
- `record`: the 104-byte boot-root record, `boot_root.approval`: the
 bootloader tree's root, an epoch, and a P-256 signature over
 `SHA-256("NONOS-BOOT-ROOT-v1" || root || epoch)` under the device policy key.
 An all-zero key verifies nothing.
- `gate`: the verdict. `measured` needs the log, the live PCR 4 and the TPM's
 rollback floor: the replay must give the live PCR 4, the record must verify
 with an epoch at or above the floor, and the last application's digest must
 be a bootloader slot under the record's root, path and STARK both.
 `self_reported` hashes the loader image the loader handed over and checks it
 with a floor of 0; nothing measured those bytes. `BootError::code` groups
 refusals by hundreds, and 400 plus the `nox_verify` code is a STARK refusal.

Every length is read from the bytes and bounded before use; the crate is meant
to panic on no input. `nox_verify` comes from STARKs main, at the commit the
flake's `starks` input locks.

## Checks

- 28 host tests: known answers, hostile PE images, hostile and oversized logs,
 record and gate cases, and the record format as `tools/nonos-policy-approve`
 writes it.
- Five cargo-fuzz targets in `fuzz/`: `v4_parse`, `tcg_log`, `pe_digest`,
 `boot_record` and `v4_gate`. The nightly fuzz workflow does not list this
 crate's targets.

## What is not done

- The kernel's check has not yet admitted a loader at boot. The crate is built and tested on the host.
- The kernel reads each module the loader hands over with a 4 MiB bound. A
 full image's loader was larger, so on a TPM with no rollback counter the
 verdict was `NoEvidence` and the kernel stopped before init
.
- Without a TPM or a log the verdict is `SelfReported`, with no epoch floor.
