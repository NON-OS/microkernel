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

//! A data frame on an associated session, through the station's receive
//! checks: to the stack, or to the supplicant for a key handshake.

use super::pump::Ended;
use super::state::{Session, RX_QUEUE};
use crate::softmac::radio::Radio;
use crate::softmac::stats::LinkStats;
use crate::station::Rx;

impl Session {
    pub(super) fn on_data<R: Radio>(
        &mut self,
        r: &mut R,
        f: &[u8],
        hw: bool,
        st: &mut LinkStats,
    ) -> Option<Ended> {
        match self.station.receive(f, hw) {
            Rx::Ethernet(eth) => {
                if self.queue.len() == RX_QUEUE {
                    self.queue.pop_front();
                    LinkStats::bump(&mut st.rx_err);
                }
                self.queue.push_back(eth);
                LinkStats::bump(&mut st.rx_eth);
            }
            Rx::Handshake { frame, group_key } => {
                if let Some(reply) = frame {
                    r.send(&reply);
                }
                if group_key.is_some() {
                    LinkStats::bump(&mut st.rekeys);
                }
            }
            Rx::Dropped(_) => LinkStats::bump(&mut st.rx_refused),
        }
        None
    }
}
