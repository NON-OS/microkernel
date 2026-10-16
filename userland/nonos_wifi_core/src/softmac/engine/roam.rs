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

//! Roaming within the session's ESS, between the session's pumps. When
//! its AP has been heard weak for a while, a passive look runs (`roam`);
//! while it is away, the session waits. When the look is done, the best AP
//! heard is chosen, and the station moves to it (`roam_go`). If none
//! qualifies, or the move fails, the station stays where it is and does not
//! look again until the cooldown has passed.

use super::station::SoftStation;
use crate::softmac::radio::{Env, Radio};
use crate::softmac::roam::{pick, Look, Step};

impl<R: Radio, E: Env> SoftStation<R, E> {
    /// One step of roaming; true while the station is off its channel.
    pub(super) fn roam_away(&mut self) -> bool {
        let now = self.env.now_ms();
        let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_mut()) else { return false };
        let ssid = &s.ssid[..usize::from(s.ssid_len)];
        let Some(kit) = s.kit.as_mut() else { return false };
        let Some(look) = self.look.as_mut() else {
            if kit.due(now) {
                let (home, link) = ((s.band, s.channel), (r.mac(), s.bssid));
                self.look = Some(Look::new(self.domain, r.bands(), home, link.0, link.1));
            }
            return false;
        };
        match look.step(r, ssid, now) {
            Step::Away => return true,
            Step::Home => return false,
            Step::Done => {}
        }
        kit.rest(now);
        let current = (s.bssid, kit.signal.avg_dbm());
        let heard = self.look.take().map(|l| l.heard).unwrap_or_default();
        if let Some(i) = pick(current, ssid, &kit.runs, kit.policy, &heard) {
            self.roam_to(&heard[i]);
        }
        false
    }
}
