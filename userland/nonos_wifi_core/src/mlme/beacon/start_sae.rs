// NONOS Operating System (AGPL-3.0-or-later)

//! An SAE commit from the PWE and the join's randomness.

use super::super::failure::MlmeFailure;
use super::super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::auth::auth_frame;
use crate::sae::frame::AUTH_ALG_SAE;
use crate::sae::h2e::{derive_pt, pwe_from_pt};
use crate::sae::hnp::derive_pwe;
use crate::sae::SaeStation;
use crate::wpa::sha256::hmac_sha256_parts;

impl Mlme {
    // Derive the PWE and send the SAE commit.
    pub(super) fn start_sae(&mut self, h2e: bool) -> MlmeOutput {
        // Drawn by the driver before the join began; a driver that could not
        // draw it refused the join there (its `draw_entropy`).
        let entropy = self.sae_entropy;
        let (pw, ssid) = (self.secret(), self.ssid());
        let pwe = if h2e {
            derive_pt(ssid, pw, None).and_then(|pt| pwe_from_pt(&pt, &self.our_mac, &self.bssid))
        } else {
            derive_pwe(pw, &entropy[64..64 + pw.len()], &self.our_mac, &self.bssid)
        };
        let Some(pwe) = pwe else {
            return self.fail(MlmeFailure::BadPassphrase);
        };
        // rand and mask straight from the entropy. Either is unusable when it
        // is at or above the P-256 order (about 2^-32 each), so a try fails
        // about once in 2^31; each retry derives new values by HMAC-SHA256 of
        // the 64 bytes under its own label and number, never the same bytes
        // again, so all four fail about once in 2^124.
        let mut sae = None;
        for attempt in 0u8..4 {
            let (rand, mask) = if attempt == 0 {
                let mut r = [0u8; 32];
                let mut m = [0u8; 32];
                r.copy_from_slice(&entropy[..32]);
                m.copy_from_slice(&entropy[32..64]);
                (r, m)
            } else {
                (
                    hmac_sha256_parts(&entropy[..64], &[b"rand", &[attempt]]),
                    hmac_sha256_parts(&entropy[..64], &[b"mask", &[attempt]]),
                )
            };
            sae = SaeStation::start(pwe, h2e, &rand, &mask);
            if sae.is_some() {
                break;
            }
        }
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
