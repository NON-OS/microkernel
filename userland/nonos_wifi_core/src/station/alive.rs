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

//! Whether the access point is still there, by elapsed time: unheard for
//! `QUIET_MS` (or at a firmware's missed-beacons notice) the station probes it,
//! and with `PROBES` unanswered the link is lost. Any frame from it is heard.

/// How long the access point may go unheard: about twenty beacon intervals.
pub const QUIET_MS: u64 = 2_000;
/// How long each probe is given, and how many before the access point is gone.
pub const PROBE_EVERY_MS: u64 = 500;
pub const PROBES: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Due {
    Nothing,
    Probe,
    Lost,
}

#[derive(Clone, Copy)]
pub(super) struct Alive {
    /// When the access point was last heard, from the first tick on.
    heard_ms: Option<u64>,
    /// The radio said the beacons stopped: ask now, unless already asking.
    missed: bool,
    probes: u8,
    next_ms: u64,
}

impl Alive {
    pub(super) const fn new() -> Self {
        Self { heard_ms: None, missed: false, probes: 0, next_ms: 0 }
    }

    pub(super) fn heard(&mut self, now: u64) {
        self.heard_ms = Some(now);
        self.missed = false;
        self.probes = 0;
    }

    pub(super) fn missed(&mut self, now: u64) {
        if self.probes == 0 {
            self.missed = true;
            self.next_ms = now;
        }
    }

    pub(super) fn due(&mut self, now: u64) -> Due {
        let heard = *self.heard_ms.get_or_insert(now);
        let quiet = self.missed || now.saturating_sub(heard) >= QUIET_MS;
        if !quiet || now < self.next_ms {
            return Due::Nothing;
        }
        if self.probes >= PROBES {
            return Due::Lost;
        }
        self.probes += 1;
        self.next_ms = now.saturating_add(PROBE_EVERY_MS);
        Due::Probe
    }
}
