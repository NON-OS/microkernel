// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

use super::types::CryptoVerifyResult;
use crate::log::logger::{log_error, log_info};

pub(super) fn verify_kernel_self_attestation(
    parsed: &crate::image_format::ParsedImage<'_>,
    result: &mut CryptoVerifyResult,
) {
    let Some(trailer) = parsed.proof_bytes else {
        log_info("kernel_verify", "no self-attestation trailer present");
        return;
    };
    result.proof_present = true;
    result.proof_len = super::self_attest::proof_len(trailer);
    #[cfg(feature = "dev-attest")]
    if trailer.starts_with(&nonos_attest_path::MAGIC) {
        result.path_attested = super::self_attest::verify_kernel_path_only(parsed.kernel_bytes, trailer);
        if result.path_attested {
            log_info("kernel_verify", "development kernel: path verified, no STARK proof, never a release");
        } else {
            log_error("kernel_verify", "kernel self-attestation path FAILED");
        }
        return;
    }
    if !trailer.starts_with(&nonos_attest_path::MAGIC_V4) {
        log_error("kernel_verify", "proof footer is not a v4 self-attestation trailer");
        return;
    }
    if super::self_attest::verify_kernel_self_attestation(parsed.kernel_bytes, trailer) {
        result.path_attested = true;
        log_info("kernel_verify", "kernel self-attestation path and STARK verified");
    } else {
        result.path_attested = false;
        log_error("kernel_verify", "kernel self-attestation FAILED");
    }
}
