// NONOS Operating System (AGPL-3.0-or-later)
//! The AP's SAE commit: started on the station's, with the password element
//! for this station's address (`derive_pt`, `pwe_from_pt`), answered with
//! its own commit; its confirm is kept for the station's.

use super::{SaeAp, PASS, SSID};
use crate::ap_sim::AA;
use crate::soft::frames::mgmt_from;
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::{SaeStation, SaeStep};

impl SaeAp {
    pub(super) fn commit(&mut self, sta: [u8; 6], status: u16, body: &[u8]) -> Vec<Vec<u8>> {
        let pt = derive_pt(SSID, PASS, None).expect("password element");
        let pwe = pwe_from_pt(&pt, &AA, &sta).expect("PWE");
        let mut sae = SaeStation::start(pwe, true, &[0x33; 32], &[0x44; 32]).expect("SAE");
        let SaeStep::Send { body: confirm, .. } = sae.on_frame(1, status, body) else {
            return vec![];
        };
        self.confirm = confirm;
        let (_, st, commit) = sae.commit_frame();
        self.sae = Some(sae);
        vec![auth(sta, 1, st, &commit)]
    }
}

pub(super) fn auth(sta: [u8; 6], seq: u16, status: u16, body: &[u8]) -> Vec<u8> {
    let head = [3u16, seq, status].map(u16::to_le_bytes).concat();
    mgmt_from(AA, 11, sta, &[head, body.to_vec()].concat())
}
