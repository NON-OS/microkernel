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

//! Fresh randomness for a join, and whether a WPA3 join is tried again with
//! WPA2.

use nonos_libc::crypto_random;
use nonos_wifi_core::mlme::{Entropy, MlmeFailure, SAE_ENTROPY};
use nonos_wifi_core::sae::SaeFailure;

use crate::assoc::{self, Outcome};

use super::consts::AKM_SAE_TYPE;

// Fresh randomness for one join, or `None` if the kernel gave none.
pub(super) fn draw_entropy() -> Option<Entropy> {
    let mut e = Entropy { snonce: [0u8; 32], sae: [0u8; SAE_ENTROPY] };
    let a = crypto_random(e.snonce.as_mut_ptr(), e.snonce.len());
    let b = crypto_random(e.sae.as_mut_ptr(), e.sae.len());
    (a == e.snonce.len() as i64 && b == e.sae.len() as i64).then_some(e)
}

// Whether a join that ran WPA3 and did not finish is tried again with WPA2:
// not when the network was saved as WPA3, not when it joined, and not when
// the router's confirm said the password is wrong.
pub(super) fn retry_with_psk(report: &assoc::Report, wpa3_only: bool) -> bool {
    let ran_sae = report.akm.is_some_and(|a| a.suite_type() == AKM_SAE_TYPE);
    let wrong_password = matches!(report.failure, Some(MlmeFailure::Sae(SaeFailure::BadConfirm)));
    !wpa3_only && ran_sae && !wrong_password && !matches!(report.outcome, Outcome::Joined { .. })
}
