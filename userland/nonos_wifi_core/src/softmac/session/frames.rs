// NONOS Operating System (AGPL-3.0-or-later)

//! A data frame on an associated session, through the station's receive
//! checks: to the stack, or to the supplicant for a key handshake. A frame
//! from the AP that passed them is what the AP's signal is judged by.

use super::pump::Ended;
use super::state::{Session, RX_QUEUE};
use crate::softmac::radio::{Radio, RxInfo};
use crate::softmac::stats::LinkStats;
use crate::station::Rx;
use crate::vault::wipe;

impl Session {
    pub(super) fn on_data<R: Radio>(
        &mut self,
        r: &mut R,
        f: &[u8],
        rx: RxInfo,
        st: &mut LinkStats,
    ) -> Option<Ended> {
        match self.station.receive(f, rx.decrypted) {
            Rx::Ethernet(eth) => {
                // The AP's signal is judged only from frames that verified
                // under the session's keys: anyone can send a beacon in the
                // AP's name, and weak forged ones would force a look.
                if let (true, Some(k)) = (f[10..16] == self.bssid, self.kit.as_mut()) {
                    k.signal.heard(rx.rssi_dbm);
                }
                if self.queue.len() == RX_QUEUE {
                    self.queue.pop_front();
                    LinkStats::bump(&mut st.rx_err);
                }
                self.queue.push_back(eth);
                LinkStats::bump(&mut st.rx_eth);
            }
            Rx::Handshake { frame, mut group_key, mut pairwise_key } => {
                if let Some(reply) = frame {
                    self.put(r, &reply);
                }
                if group_key.is_some() || pairwise_key.is_some() {
                    LinkStats::bump(&mut st.rekeys);
                }
                let keyed = self.rekey_chip(r, group_key.map(|g| g.0), pairwise_key.is_some());
                // The station holds its own; these copies go.
                group_key.iter_mut().for_each(|g| wipe(&mut g.1));
                pairwise_key.iter_mut().for_each(|k| wipe(k));
                if !keyed {
                    return Some(Ended::Lost);
                }
            }
            // Only a management frame sends or ends from here, and those
            // come in through `on_mgmt`.
            Rx::Send(frame) => {
                self.put(r, &frame);
            }
            Rx::Lost(_) => return Some(Ended::Lost),
            Rx::Dropped(_) => LinkStats::bump(&mut st.rx_refused),
        }
        None
    }
}
