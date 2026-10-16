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

//! The end of a move: the session on the new AP takes over, with the kit
//! of the old one, its signal started over and its cooldown begun. Or the
//! station stays on the old AP, which never let it go.

use super::station::SoftStation;
use crate::softmac::join::{Outcome, Report};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::roam::Heard;
use crate::softmac::session::{Keys, Session};

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub(super) fn take_over(&mut self, h: &Heard, report: Report) {
        let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_mut()) else { return };
        let Outcome::Joined { bssid, channel, tk, akm, supplicant } = report.outcome else {
            return;
        };
        let net = (s.ssid(), bssid, akm.suite_type());
        let at = (h.band, if channel != 0 { channel } else { h.channel });
        let now = self.env.now_ms();
        let mut next = Session::start(r.mac(), net, at, Keys { tk, supplicant }, now);
        next.kit = s.kit.take().map(|mut k| {
            k.signal.reset();
            k.runs = report.selection.unwrap_or(k.runs);
            k.rest(now);
            k
        });
        self.session = Some(next);
    }

    /// Back to the session's own AP, which still holds the association.
    pub(super) fn stay(&mut self) {
        let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_ref()) else { return };
        r.tune(s.band, s.channel);
        r.set_bss(Some(s.bssid));
    }
}
