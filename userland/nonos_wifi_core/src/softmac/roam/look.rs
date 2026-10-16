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

//! A look for other APs of the session's network. It is passive: the
//! station only listens and sends no probe, so it names no network. It
//! visits every channel of the domain's plan but its own, `AWAY_CHANNELS`
//! at a time, a passive dwell on each, and comes home for `HOME_MS`
//! between. Each trip starts by telling the AP the station is away, and
//! ends by telling it the station is back (`null`).

use alloc::vec::Vec;

use super::pick::Heard;
use crate::regdom::{Band, Domain, ScanPlan};

pub const AWAY_CHANNELS: usize = 2;
pub const HOME_MS: u64 = 300;
pub const HEARD_MAX: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Off the session's channel: its frames wait.
    Away,
    /// On it: the session runs as usual.
    Home,
    /// Every channel visited, and the station is home.
    Done,
}

pub(super) enum Phase {
    Away { left: usize, until_ms: u64, at: (Band, u8) },
    Home { until_ms: u64 },
    Done,
}

pub struct Look {
    pub(super) plan: ScanPlan,
    pub(super) home: (Band, u8),
    pub(super) bss: ([u8; 6], [u8; 6]),
    pub(super) phase: Phase,
    pub heard: Vec<Heard>,
}

impl Look {
    /// A look from the session on `home`, of station `sta` with AP `bssid`.
    pub fn new(
        domain: Domain,
        bands: (bool, bool),
        home: (Band, u8),
        sta: [u8; 6],
        bssid: [u8; 6],
    ) -> Self {
        let plan = ScanPlan::new(domain, bands.0, bands.1);
        Self {
            plan,
            home,
            bss: (sta, bssid),
            phase: Phase::Home { until_ms: 0 },
            heard: Vec::new(),
        }
    }
}
