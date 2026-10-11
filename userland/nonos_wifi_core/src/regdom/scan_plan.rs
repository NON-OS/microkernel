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

//! The sweep a scan runs: every channel the domain does not close, in order,
//! with what the station may do there. A probe request is initiating
//! radiation, so it goes out only where the domain permits it; elsewhere the
//! station dwells long enough to hear a beacon.

use super::band::{channel_mhz, Band, CHANNELS_2GHZ, CHANNELS_5GHZ};
use super::domain::Domain;
use super::permit::Permit;
use super::scan_step::{ScanMode, ScanStep, ACTIVE_DWELL_MS, PASSIVE_DWELL_MS};

/// An iterator over the steps of one sweep of the bands the radio has.
pub struct ScanPlan {
    domain: Domain,
    ghz2: bool,
    ghz5: bool,
    at: usize,
}

impl ScanPlan {
    pub const fn new(domain: Domain, ghz2: bool, ghz5: bool) -> Self {
        Self { domain, ghz2, ghz5, at: 0 }
    }

    fn slot(&self, i: usize) -> Option<(Band, u8, bool)> {
        match CHANNELS_2GHZ.get(i) {
            Some(&ch) => Some((Band::Ghz2, ch, self.ghz2)),
            None => {
                CHANNELS_5GHZ.get(i - CHANNELS_2GHZ.len()).map(|&ch| (Band::Ghz5, ch, self.ghz5))
            }
        }
    }
}

impl Iterator for ScanPlan {
    type Item = ScanStep;

    fn next(&mut self) -> Option<ScanStep> {
        while let Some((band, channel, wanted)) = self.slot(self.at) {
            self.at += 1;
            let mode = match self.domain.permit(band, channel) {
                _ if !wanted => continue,
                Permit::Disabled => continue,
                Permit::Passive { .. } => ScanMode::Passive,
                Permit::Active => ScanMode::Active,
            };
            let dwell_ms =
                if mode == ScanMode::Active { ACTIVE_DWELL_MS } else { PASSIVE_DWELL_MS };
            let mhz = channel_mhz(band, channel)?;
            return Some(ScanStep { band, channel, mhz, mode, dwell_ms });
        }
        None
    }
}
