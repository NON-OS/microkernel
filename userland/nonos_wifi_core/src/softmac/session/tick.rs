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

//! The clock side of a session: the SA Query's retries and expiry, and the
//! loss of an AP that has gone silent.

use super::pump::Ended;
use super::state::{Session, LOSS_MS};
use crate::softmac::radio::{Env, Radio};
use crate::station::SaTick;

impl Session {
    pub(super) fn tick<R: Radio, E: Env>(&mut self, r: &mut R, env: &mut E) -> Option<Ended> {
        let now = env.now_ms();
        let mut fresh = [0u8; 2];
        // Without randomness no new identifier goes out; the query waits on
        // the ones already sent and still ends at its maximum.
        let have = env.random(&mut fresh);
        match self.station.sa_query_poll(now, fresh) {
            SaTick::Send(f) if have => {
                r.send(&f);
            }
            SaTick::Ended => return Some(Ended::Lost),
            _ => {}
        }
        if now.saturating_sub(self.heard_ms) > LOSS_MS {
            self.station.deassociate();
            return Some(Ended::Lost);
        }
        None
    }
}
