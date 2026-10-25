// NONOS Operating System (AGPL-3.0-or-later)

//! A WPA3 access point behind this driver's `Radio`, answering what the
//! driver sends: its SAE half is the shared SAE code's own station, and its
//! four-way handshake the simulator's messages under the PMK that SAE made.

use super::frames::{eapol, mgmt, sae_auth, AP, PASS, SSID, STA};
use crate::ap_sim::{message1, ANONCE};
use crate::assoc::Radio;
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::{SaeStation, SaeStep};
use nonos_wifi_core::wpa::akm::Akm;
use std::collections::VecDeque;

pub struct SaeAp {
    pub(super) sae: SaeStation,
    confirm: Vec<u8>,
    pub ptk: Vec<u8>,
    pub sent_by_sta: Vec<Vec<u8>>,
    pub(super) queue: VecDeque<Vec<u8>>,
}

impl SaeAp {
    pub fn new() -> Self {
        let pwe = pwe_from_pt(&derive_pt(SSID, PASS, None).unwrap(), &AP, &STA).unwrap();
        let sae = SaeStation::start(pwe, true, &[0x33; 32], &[0x44; 32]).unwrap();
        let (confirm, ptk, sent_by_sta, queue) =
            (Vec::new(), Vec::new(), Vec::new(), VecDeque::new());
        Self { sae, confirm, ptk, sent_by_sta, queue }
    }

    fn answer(&mut self, f: &[u8]) {
        let body = &f[24..];
        match (f[0], u16::from_le_bytes([body[2], body[3]])) {
            (0xB0, 1) => {
                let status = u16::from_le_bytes([body[4], body[5]]);
                let SaeStep::Send { body, .. } = self.sae.on_frame(1, status, &body[6..]) else {
                    return;
                };
                self.confirm = body;
                let (_, st, commit) = self.sae.commit_frame();
                self.queue.push_back(sae_auth(1, st, &commit));
            }
            (0xB0, 2) => {
                if matches!(self.sae.on_frame(2, 0, &body[6..]), SaeStep::Accepted { .. }) {
                    let confirm = self.confirm.clone();
                    self.queue.push_back(sae_auth(2, 0, &confirm));
                }
            }
            (0x00, _) => {
                self.queue.push_back(mgmt(0x10, STA, &[0x11, 0x04, 0, 0, 0x01, 0xC0]));
                self.queue.push_back(eapol(&message1(Akm::Sae, 1, &ANONCE)));
            }
            _ => self.on_eapol(f),
        }
    }
}

impl Radio for SaeAp {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.sent_by_sta.push(mpdu.to_vec());
        self.answer(mpdu);
        true
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<usize> {
        let f = self.queue.pop_front()?;
        out[..f.len()].copy_from_slice(&f);
        Some(f.len())
    }
}
