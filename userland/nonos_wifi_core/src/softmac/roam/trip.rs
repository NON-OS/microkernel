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

//! The trips of a look: the next channel to visit, going away, moving on,
//! and coming home.

use super::look::{Look, Phase};
use super::null::pm_null;
use crate::regdom::{Band, PASSIVE_DWELL_MS};
use crate::softmac::radio::Radio;

impl Look {
    /// The next channel of the plan that is not home.
    pub(super) fn next_channel(&mut self) -> Option<(Band, u8)> {
        let home = self.home;
        self.plan.by_ref().map(|s| (s.band, s.channel)).find(|&c| c != home)
    }

    pub(super) fn leave<R: Radio>(&mut self, r: &mut R, at: (Band, u8), left: usize, now: u64) {
        let (sta, bssid) = self.bss;
        r.send(&pm_null(sta, bssid, true));
        r.set_bss(None);
        r.tune(at.0, at.1);
        let until_ms = now.saturating_add(u64::from(PASSIVE_DWELL_MS));
        self.phase = Phase::Away { left, until_ms, at };
    }

    pub(super) fn come_home<R: Radio>(&mut self, r: &mut R) {
        let (sta, bssid) = self.bss;
        r.tune(self.home.0, self.home.1);
        r.set_bss(Some(bssid));
        r.send(&pm_null(sta, bssid, false));
    }

    /// Already away: only retune.
    pub(super) fn leave_to<R: Radio>(&mut self, r: &mut R, at: (Band, u8), left: usize, now: u64) {
        r.tune(at.0, at.1);
        let until_ms = now.saturating_add(u64::from(PASSIVE_DWELL_MS));
        self.phase = Phase::Away { left, until_ms, at };
    }
}
