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

//! The move to another AP of the session's ESS: a reassociation naming the
//! AP the station leaves (IEEE Std 802.11-2020, 11.3.5.4), on that AP's
//! cached PMKSA when one is held. The join runs with what the session kept
//! and the security the session runs, on fresh randomness, against the
//! candidate's own beacon elements, which message 3 must match.
//!
//! On success the new session takes over, and keeps the kit. A move that
//! fails bars the candidate for this boot and reports its own code, never
//! "wrong password" (`roam_code`). A move the new AP never accepted leaves
//! the old association as it was, so the station returns to it. One the new
//! AP accepted, but whose handshake failed, may have cost the old
//! association too. That session ends, as a lost AP's does, and the scan
//! starts again.

use super::station::SoftStation;
use crate::softmac::control::roam_code::roam_code;
use crate::softmac::join::{first_attempt, keep, ConnectRequest, Joining, Outcome};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::roam::Heard;

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub(super) fn roam_to(&mut self, h: &Heard) {
        let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_mut()) else { return };
        let Some(kit) = s.kit.as_ref() else { return };
        let (ssid, from) = (s.ssid(), s.bssid);
        let pass = kit.secret.password();
        // The radio must still hold the address the session joined from: a
        // chip reset since may have put its own back, which the move would
        // show the new AP. The session ends instead, sending nothing.
        if r.mac() != s.mac() {
            return self.end_session();
        }
        let req = ConnectRequest { ssid, pass, wpa3_only: kit.wpa3_only, hidden: false };
        r.tune(h.band, h.channel);
        r.set_bss(Some(h.bssid));
        let j = Joining { tag: kit.tag.as_ref(), kept: Some(&kit.secret), from: Some(from) };
        let (env, cache, beacon, d) = (&mut self.env, &mut self.pmksa, &h.beacon, self.domain);
        let tried = first_attempt(r, env, (d, kit.policy), &req, (beacon, h.bssid), cache, &j);
        let Ok(report) = tried else { return self.stay() };
        keep(cache, env, r.mac(), &req, kit.tag.as_ref(), &report);
        if !matches!(report.outcome, Outcome::Joined { .. }) {
            self.last_roam = roam_code(&report);
            self.bar(h.bssid);
            return if report.accepted { self.end_session() } else { self.stay() };
        }
        self.take_over(h, report);
    }
}
