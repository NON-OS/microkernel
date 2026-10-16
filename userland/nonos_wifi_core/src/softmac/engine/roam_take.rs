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

//! The end of a move: the old session's keys wiped, then the session on
//! the new AP keyed and taking over, with the kit of the old one, its
//! signal started over and its cooldown begun. Or the station stays on
//! the old AP, which never let it go. And the candidates that failed,
//! barred for this boot.

use super::station::SoftStation;
use super::station_new::BARRED_MAX;
use crate::softmac::join::{Outcome, Report};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::roam::Heard;
use crate::softmac::session::{Keys, Session};

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub(super) fn take_over(&mut self, h: &Heard, report: Report) {
        let (Some(r), Some(mut old)) = (self.radio.as_mut(), self.session.take()) else { return };
        let Outcome::Joined { bssid, channel, tk, akm, supplicant } = report.outcome else {
            return;
        };
        let mut kit = old.kit.take();
        let (name, len) = (old.ssid, usize::from(old.ssid_len).min(32));
        // Nothing keyed under the old AP outlives the move.
        drop(old);
        let net = (&name[..len], bssid, akm.suite_type());
        let at = (h.band, if channel != 0 { channel } else { h.channel });
        let now = self.env.now_ms();
        let mut next = Session::start(r.mac(), net, at, Keys { tk, supplicant }, now);
        if let Some(k) = kit.as_mut() {
            k.signal.reset();
            k.runs = report.selection.unwrap_or(k.runs);
            k.rest(now);
        }
        next.kit = kit;
        self.session = Some(next);
        self.last_roam = 0;
    }

    /// Back to the session's own AP, which still holds the association.
    pub(super) fn stay(&mut self) {
        let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_ref()) else { return };
        r.tune(s.band, s.channel);
        r.set_bss(Some(s.bssid));
    }

    pub(super) fn bar(&mut self, bssid: [u8; 6]) {
        if self.barred.len() == BARRED_MAX {
            self.barred.remove(0);
        }
        self.barred.push(bssid);
    }
}
