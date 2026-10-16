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
//! cached PMKSA when one is held. The join runs with the passphrase and
//! the AKM the session runs.
//!
//! On success the new session takes over, and keeps the kit. A move the
//! new AP never accepted leaves the old association as it was, so the
//! station returns to it. One the new AP accepted, but whose handshake
//! failed, may have cost the old association too. That session ends, as a
//! lost AP's does, and the scan starts again.

use super::station::SoftStation;
use crate::softmac::join::{first_attempt, keep, ConnectRequest, Outcome};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::roam::Heard;

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub(super) fn roam_to(&mut self, h: &Heard) {
        let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_mut()) else { return };
        let Some(kit) = s.kit.as_ref() else { return };
        let (ssid, from) = (s.ssid(), s.bssid);
        let req = ConnectRequest {
            ssid,
            pass: kit.secret.get(),
            wpa3_only: kit.wpa3_only,
            hidden: false,
        };
        r.tune(h.band, h.channel);
        r.set_bss(Some(h.bssid));
        let (env, cache, beacon) = (&mut self.env, &mut self.pmksa, &h.beacon);
        let tried = first_attempt(
            r,
            env,
            self.domain,
            &req,
            kit.policy,
            beacon,
            h.bssid,
            cache,
            Some(from),
        );
        let Ok(report) = tried else { return self.stay() };
        keep(cache, env, &req, &report);
        let accepted = report.accepted;
        match report.outcome {
            Outcome::Joined { .. } => self.take_over(h, report),
            _ if accepted => self.end_session(),
            _ => self.stay(),
        }
    }
}
