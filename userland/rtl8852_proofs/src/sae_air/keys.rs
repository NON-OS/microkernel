// NONOS Operating System (AGPL-3.0-or-later)
//! The WPA3 AP's four-way handshake: message 2 gives the SNonce, the PTK
//! comes from the SAE PMK (`Akm::Sae`), message 3 carries the RSNE and RSNXE
//! the beacon did with the GTK and IGTK; message 4 installs the pairwise
//! key, under which the station's data is read.

use super::{SaeAp, RSNE_SAE, RSNXE_H2E};
use crate::ap_sim::{gtk_kde, igtk_kde, message3, AA, ANONCE, GTK};
use crate::soft::frames::eapol_from;
use nonos_wifi_core::dot11::ccmp;
use nonos_wifi_core::wpa::akm::Akm;

impl SaeAp {
    pub(super) fn on_eapol(&mut self, sta: [u8; 6], e: Vec<u8>) -> Vec<Vec<u8>> {
        let Some(pmk) = self.sae.as_ref().and_then(|s| s.pmk()) else { return vec![] };
        if self.ptk.is_empty() {
            // The SNonce sits 17 octets into the EAPOL-Key frame.
            let Some(sn) = e.get(17..49) else { return vec![] };
            let snonce: [u8; 32] = sn.try_into().unwrap();
            self.ptk = Akm::Sae.derive_ptk(&pmk.0, &AA, &sta, &ANONCE, &snonce).to_vec();
            let kd =
                [RSNE_SAE.to_vec(), RSNXE_H2E.to_vec(), gtk_kde(1, &GTK), igtk_kde(&[0x77; 16])]
                    .concat();
            return vec![eapol_from(AA, sta, &message3(Akm::Sae, 2, &ANONCE, &self.ptk, &kd))];
        }
        self.tk = self.ptk.get(32..48).map(|k| k.try_into().unwrap());
        vec![]
    }

    pub(super) fn protected(&mut self, f: &[u8]) -> Vec<Vec<u8>> {
        if let Some(body) = self.tk.and_then(|tk| ccmp::decrypt(f, &tk)) {
            self.got.push(body);
        }
        vec![]
    }
}
