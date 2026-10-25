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

//! A management frame on an associated session, through the station's
//! management frame protection: the AP leaving, an SA Query to answer or to
//! start.

use super::pump::Ended;
use super::state::Session;
use crate::softmac::radio::{Env, Radio};
use crate::station::MgmtRx;

impl Session {
    pub(super) fn on_mgmt<R: Radio, E: Env>(
        &mut self,
        r: &mut R,
        env: &mut E,
        f: &[u8],
        hw: bool,
    ) -> Option<Ended> {
        match self.station.receive_mgmt(f, hw) {
            MgmtRx::Left(reason) => return Some(Ended::Left(reason)),
            MgmtRx::Reply(frame) => {
                r.send(&frame);
            }
            MgmtRx::SaQueryNeeded => {
                let mut txid = [0u8; 2];
                if env.random(&mut txid) {
                    if let Some(req) = self.station.sa_query_start(env.now_ms(), txid) {
                        r.send(&req);
                    }
                }
            }
            MgmtRx::Confirmed | MgmtRx::Dropped(_) => {}
        }
        None
    }
}
