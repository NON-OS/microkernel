// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! How the boot admits the kernel it is about to jump to.
//!
//! kernel_verify has already folded the kernel's self-attestation path against
//! the enrolled boot root and recorded the outcome on the crypto result. Here
//! that outcome becomes the boot attestation, and the admitted kernel is
//! measured into PCR 9.

use crate::display::{
    draw_boot_progress, show_error_screen, update_stage, StageStatus, STAGE_ZK_VERIFY,
};
use crate::kernel_verify::CryptoVerifyResult;
use crate::log::logger::{log_error, log_info, log_warn};
use uefi::prelude::*;

use super::result::BootAttestationResult;

/// Attest the loaded kernel and return the verdict carried into the handoff.
pub fn attest_kernel(
    st: &mut SystemTable<Boot>,
    crypto: &CryptoVerifyResult,
    gop: bool,
    tpm: bool,
) -> BootAttestationResult {
    use crate::boot::uefi::TOTAL_BOOT_STAGES;
    use crate::boot::util::fatal_reset;

    if crypto.kernel_attested() {
        log_info("attest", "kernel measurement enrolled under the boot root");
        if tpm {
            measure_admitted_kernel(st, crypto);
        }
        update_stage(STAGE_ZK_VERIFY, StageStatus::Success);
        draw_boot_progress(8, TOTAL_BOOT_STAGES);
        return BootAttestationResult::verified(crypto.kernel_hash_full);
    }

    /* Every mode, development included: a kernel that is not enrolled does not boot. */
    log_error("attest", "kernel self-attestation missing or invalid");
    update_stage(STAGE_ZK_VERIFY, StageStatus::Failed);
    if gop {
        show_error_screen(b"kernel attestation required");
    }
    fatal_reset(st, "kernel self-attestation missing")
}

/*
 * PCR 9 gets the kernel the gate admitted and the root it was admitted under,
 * once, and only here. Before this the path gate extended nothing, so PCR 9
 * held no kernel and every key bound to it was bound to a constant: the vault's
 * machine key did not change with the kernel, and no policy over PCR 9 said
 * anything about what ran. The value is the same on every machine for one
 * release, so the release can sign it:
 *
 *   PCR9 = SHA-256(0^32 || SHA-256(SHA-256(kernel_blake3 || kernel_attest_root)))
 *
 * the inner hash from `extend_pcr_measurement`, the outer from the TCG2
 * extend. A failed extend is not fatal: nothing bound to PCR 9 unseals, which
 * fails closed.
 */
fn measure_admitted_kernel(st: &mut SystemTable<Boot>, crypto: &CryptoVerifyResult) {
    use crate::security::{extend_pcr_measurement, PCR_KERNEL};

    let mut composite = [0u8; 64];
    composite[..32].copy_from_slice(&crypto.kernel_hash_full);
    composite[32..].copy_from_slice(&crypto.attest_policy().kernel_root);
    if !extend_pcr_measurement(st, PCR_KERNEL, &composite) {
        log_warn("tpm", "the admitted kernel was not measured into PCR 9");
    }
}
