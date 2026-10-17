// NONOS Operating System (AGPL-3.0-or-later)

//! The target network's beacon chosen, and authentication started.

use super::super::failure::MlmeFailure;
use super::super::state::{Mlme, MlmeOutput, MlmeState};
use super::judge::judge;
use crate::dot11::mgmt::auth_open;
use crate::wpa::akm::Akm;
use crate::wpa::psk::psk;
use crate::wpa::supplicant::ie::Ie;

impl Mlme {
    pub(in crate::mlme) fn on_beacon(&mut self, frame: &[u8]) -> MlmeOutput {
        let j = match judge(frame, self.ssid(), self.policy) {
            None => return MlmeOutput::none(),
            Some(Err(failure)) => return self.fail(failure),
            Some(Ok(j)) => j,
        };
        if self.secret_too_long {
            return self.fail(MlmeFailure::BadPassphrase);
        }
        let sel = j.selection;
        self.bssid = j.info.bssid;
        self.channel = j.info.channel;
        self.selection = Some(sel);
        self.ap_rsne = Ie::from_slice(j.rsne);
        self.ap_rsnxe = Ie::from_option(j.rsnxe);
        self.rates = Some(j.rates);
        if sel.akm == Akm::Sae {
            return self.start_sae(sel.sae_h2e);
        }
        let Some(key) = psk(self.secret(), self.ssid()) else {
            return self.fail(MlmeFailure::BadPassphrase);
        };
        self.pmk = key;
        let seq = self.next_seq();
        let mut out = [0u8; 64];
        match auth_open(&mut out, self.our_mac, self.bssid, seq) {
            Some(n) => {
                self.state = MlmeState::Authenticating;
                MlmeOutput::send(out[..n].to_vec())
            }
            None => self.fail(MlmeFailure::AuthRejected(0)),
        }
    }
}
