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

//! Drive an associated session: what the radio received goes through the
//! station's checks (data to the stack or the supplicant, management frames
//! under management frame protection), the SA Query advances on the clock,
//! and an AP silent past the loss interval ends the session.

use super::state::Session;
use crate::dot11::header::{fc_type, TYPE_DATA, TYPE_MGMT};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::stats::LinkStats;

/// Frames taken per pump, so a busy channel never starves the serve loop.
const PER_PUMP: usize = 32;

/// Why a session ended on its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ended {
    /// The AP deauthenticated or disassociated the station with this reason.
    Left(u16),
    /// The AP fell silent, or never answered an SA Query.
    Lost,
}

impl Session {
    pub fn pump<R: Radio, E: Env>(
        &mut self,
        r: &mut R,
        env: &mut E,
        st: &mut LinkStats,
    ) -> Option<Ended> {
        let mut buf = [0u8; 2400];
        for _ in 0..PER_PUMP {
            let Some(rx) = r.recv(&mut buf) else { break };
            LinkStats::bump(&mut st.rx_ring);
            let f = &buf[..rx.len.min(buf.len())];
            if f.len() < 24 {
                continue;
            }
            if f[10..16] == self.bssid {
                self.heard_ms = env.now_ms();
                // Only the AP's beacons weigh in: they come at a steady rate.
                if let (0x80, Some(k)) = (f[0], self.kit.as_mut()) {
                    k.signal.heard(rx.rssi_dbm);
                }
            }
            let ended = match fc_type(u16::from_le_bytes([f[0], f[1]])) {
                TYPE_DATA => self.on_data(r, f, rx.decrypted, st),
                TYPE_MGMT => self.on_mgmt(r, env, f, rx.decrypted),
                _ => None,
            };
            if ended.is_some() {
                return ended;
            }
        }
        self.tick(r, env)
    }
}
