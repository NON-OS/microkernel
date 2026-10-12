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

//! When a host's wanted size is asked for (section 12, Size): a size held
//! 250 ms, so a window being dragged is not chased frame by frame; never
//! one already asked for, so a size the driver refused or `fit` turned
//! down is not asked again until the host wants another. At start the
//! preferred mode is wanted with no hold.

pub const HOLD_MS: i64 = 250;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Resize {
    /// The size the host wants now, 0 x 0 none; since when.
    pub want: (u32, u32),
    pub since: i64,
    /// The last size asked for (taken through `fit` or not).
    pub asked: Option<(u32, u32)>,
    /// Whether the start-up size has been looked for on this driver.
    pub started: bool,
}

impl Resize {
    /// The preferred mode, wanted at once, unless the host already said
    /// what it wants.
    pub fn start(&mut self, preferred: Option<(u32, u32)>) {
        self.started = true;
        let none = self.want == (0, 0);
        if let Some(p) = preferred.filter(|p| none && p.0 != 0 && p.1 != 0) {
            (self.want, self.since) = (p, i64::MIN / 2);
        }
    }

    /// A size the host wants, seen at `now`; a new one restarts the hold
    /// and 0 x 0 (no hint) changes nothing.
    pub fn note(&mut self, want: (u32, u32), now: i64) {
        if want.0 != 0 && want.1 != 0 && want != self.want {
            (self.want, self.since) = (want, now);
        }
    }

    /// Whether a size is waiting to be asked for: held long enough, and
    /// not asked for already.
    pub fn due(&self, now: i64) -> bool {
        let held = now.saturating_sub(self.since) >= HOLD_MS;
        held && self.want.0 != 0 && self.asked != Some(self.want)
    }

    /// The size to ask for now, marked asked; None while held or done.
    pub fn take(&mut self, now: i64) -> Option<(u32, u32)> {
        self.due(now).then(|| {
            self.asked = Some(self.want);
            self.want
        })
    }
}
