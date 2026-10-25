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

//! SAE authentication: the password element (hash-to-element if the AP
//! advertises it, else hunting and pecking), the scalars, the commit frame.

use super::failure::MlmeFailure;
use super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::auth::auth_frame;
use crate::sae::frame::AUTH_ALG_SAE;
use crate::sae::SaeStation;
use crate::vault::wipe;
use crate::wpa::sha256::hmac_sha256_parts;

/// rand and mask straight from the entropy; in the (2^-32) case that either
/// is not a usable scalar, replacements drawn from it by HMAC.
fn scalars(entropy: &[u8], attempt: u8) -> ([u8; 32], [u8; 32]) {
    if attempt == 0 {
        let (mut r, mut m) = ([0u8; 32], [0u8; 32]);
        r.copy_from_slice(&entropy[..32]);
        m.copy_from_slice(&entropy[32..64]);
        return (r, m);
    }
    (
        hmac_sha256_parts(&entropy[..64], &[b"rand", &[attempt]]),
        hmac_sha256_parts(&entropy[..64], &[b"mask", &[attempt]]),
    )
}

impl Mlme {
    pub(super) fn start_sae(&mut self, h2e: bool) -> MlmeOutput {
        // Each exchange's randomness is used once: taken, then wiped.
        let Some(mut entropy) = self.sae_entropy.take() else {
            return self.fail(MlmeFailure::NoEntropy);
        };
        let pwe = self.pwe(h2e, &entropy);
        self.forget_secret();
        let Some(pwe) = pwe else {
            return self.fail(MlmeFailure::BadPassphrase);
        };
        let mut sae = None;
        for attempt in 0u8..4 {
            let (rand, mask) = scalars(&entropy, attempt);
            sae = SaeStation::start(pwe, h2e, &rand, &mask);
            if sae.is_some() {
                break;
            }
        }
        wipe(&mut entropy);
        let Some(sae) = sae else {
            return self.fail(MlmeFailure::SaeStart);
        };
        let (seq, status, body) = sae.commit_frame();
        self.sae = Some(sae);
        self.state = MlmeState::Authenticating;
        let sc = self.next_seq();
        MlmeOutput::send(auth_frame(self.our_mac, self.bssid, sc, AUTH_ALG_SAE, seq, status, &body))
    }
}
