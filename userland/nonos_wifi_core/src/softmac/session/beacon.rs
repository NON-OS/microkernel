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

//! The session AP's beacons. Without beacon protection they show only that
//! the AP is still there. With it, a verified beacon also feeds the signal
//! a look is judged by, so a link that carries no data can still roam, and
//! a beacon that fails is counted and otherwise ignored, liveness included.

use super::state::Session;
use crate::softmac::radio::{Env, RxInfo};
use crate::softmac::stats::LinkStats;
use crate::station::BeaconRx;

impl Session {
    pub(super) fn on_beacon<E: Env>(&mut self, f: &[u8], rx: RxInfo, env: &E, st: &mut LinkStats) {
        if f[10..16] != self.bssid {
            return;
        }
        match self.station.receive_beacon(f) {
            BeaconRx::Unprotected => self.heard_ms = env.now_ms(),
            BeaconRx::Verified => {
                self.heard_ms = env.now_ms();
                if let Some(k) = self.kit.as_mut() {
                    k.signal.heard(rx.rssi_dbm);
                }
            }
            BeaconRx::Refused(_) => LinkStats::bump(&mut st.beacons_refused),
        }
    }
}
