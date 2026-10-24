// NONOS Operating System (AGPL-3.0-or-later)

//! The access point's four-way handshake: message 2 from the station, under
//! the PMK SAE made, is answered with message 3 carrying the group keys.

use super::ap::SaeAp;
use super::frames::{eapol, AP, SNAP_EAPOL, STA};
use crate::ap_sim::{gtk_kde, igtk_kde, message3, ANONCE, AP_RSNE_MIXED, GTK};
use nonos_wifi_core::wpa::akm::Akm;

impl SaeAp {
    pub(super) fn on_eapol(&mut self, f: &[u8]) {
        let Some(at) = f.windows(8).position(|w| w == SNAP_EAPOL) else { return };
        let is_msg2 = self.ptk.is_empty() && f.len() > at + 8;
        if is_msg2 {
            let pmk = self.sae.pmk().expect("SAE accepted").0;
            // The SNonce sits 17 octets into the EAPOL-Key frame.
            let snonce: [u8; 32] = f[at + 8 + 17..at + 8 + 49].try_into().unwrap();
            self.ptk = Akm::Sae.derive_ptk(&pmk, &AP, &STA, &ANONCE, &snonce).to_vec();
            let kd = [
                AP_RSNE_MIXED.to_vec(),
                vec![244, 1, 0x20],
                gtk_kde(1, &GTK),
                igtk_kde(&[0x77; 16]),
            ]
            .concat();
            self.queue.push_back(eapol(&message3(Akm::Sae, 2, &ANONCE, &self.ptk, &kd)));
        }
    }
}
