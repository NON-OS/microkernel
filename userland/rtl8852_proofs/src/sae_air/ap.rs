// NONOS Operating System (AGPL-3.0-or-later)
//! What the WPA3 access point answers. SAE starts on the station's commit,
//! with the password element for this station's address (H2E, as
//! `derive_pt` and `pwe_from_pt` make it), answers commit with commit and
//! confirm with confirm once the station's verifies; association opens the
//! handshake (`keys`).

use super::commit::auth;
use super::{RSNE_SAE, RSNXE_H2E, SSID};
use crate::ap_sim::{message1, AA, ANONCE};
use crate::soft::frames::{beacon_from, eapol_of, mgmt_from};
use nonos_wifi_core::sae::{SaeStation, SaeStep};
use nonos_wifi_core::wpa::akm::Akm;

pub struct SaeAp {
    pub channel: u8,
    pub(super) sae: Option<SaeStation>,
    pub(super) confirm: Vec<u8>,
    pub(super) ptk: Vec<u8>,
    /// The pairwise key, once message 4 came.
    pub tk: Option<[u8; 16]>,
    /// The plaintext of the station's protected data frames.
    pub got: Vec<Vec<u8>>,
}

impl SaeAp {
    pub fn new(channel: u8) -> Self {
        let (confirm, ptk, got) = (Vec::new(), Vec::new(), Vec::new());
        SaeAp { channel, sae: None, confirm, ptk, tk: None, got }
    }

    pub fn beacon(&self, subtype: u8, to: [u8; 6]) -> Vec<u8> {
        let ies = [&RSNE_SAE[..], &RSNXE_H2E].concat();
        beacon_from(AA, subtype, to, (SSID, self.channel, &ies))
    }

    pub fn on_frame(&mut self, f: &[u8]) -> Vec<Vec<u8>> {
        let sta: [u8; 6] = f[10..16].try_into().unwrap();
        let body = &f[24..];
        let field = |i: usize| u16::from_le_bytes([body[2 * i], body[2 * i + 1]]);
        match (f[0] >> 2 & 3, f[0] >> 4) {
            (0, 4) => vec![self.beacon(5, sta)],
            (0, 11) if field(0) == 3 && field(1) == 1 => self.commit(sta, field(2), &body[6..]),
            (0, 11) if field(0) == 3 && field(1) == 2 => {
                let ok = self.sae.as_mut().map(|s| s.on_frame(2, 0, &body[6..]));
                match ok {
                    Some(SaeStep::Accepted { .. }) => vec![auth(sta, 2, 0, &self.confirm)],
                    _ => vec![],
                }
            }
            (0, 0) => {
                self.ptk.clear();
                vec![
                    mgmt_from(AA, 1, sta, &[0x11, 0x04, 0, 0, 1, 0xc0]),
                    crate::soft::frames::eapol_from(AA, sta, &message1(Akm::Sae, 1, &ANONCE)),
                ]
            }
            (2, _) if f[1] & 0x40 != 0 => self.protected(f),
            (2, _) => eapol_of(f).map_or(vec![], |e| self.on_eapol(sta, e.to_vec())),
            _ => vec![],
        }
    }
}
