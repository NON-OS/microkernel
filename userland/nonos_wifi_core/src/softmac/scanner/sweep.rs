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

//! The background scan: while no session holds the radio, it hops the
//! channels the regulatory domain leaves open and listens, building the
//! picture the scan op answers from instantly. It never transmits, so no
//! saved network's name leaves the machine. The hop runs on the clock, so a
//! serve loop busy with requests moves on as surely as an idle one.

use crate::dot11::parse::parse_beacon;
use crate::regdom::{Band, Domain, ScanPlan};
use crate::scan_list::{beacon_flags, signal_percent, ScanResults};
use crate::softmac::radio::{Env, Radio};

use super::hints::Hints;

/// Frames drained per step, so a busy channel never starves the serve loop.
const DRAIN_PER_STEP: usize = 16;

pub struct Scanner {
    pub(super) results: ScanResults,
    pub(super) hints: Hints,
    pub(super) plan: Option<ScanPlan>,
    pub(super) at: Option<(Band, u8)>,
    pub(super) hop_at_ms: u64,
    /// Steps taken, frames the radio delivered, and those that were beacons:
    /// the panel reads them to tell where a scan that hears nothing stops.
    pub steps: u32,
    pub raw: u32,
    pub beacons: u32,
}

impl Scanner {
    pub fn step<R: Radio, E: Env>(&mut self, r: &mut R, env: &E, domain: Domain) {
        self.steps = self.steps.wrapping_add(1);
        let mut buf = [0u8; 2400];
        for _ in 0..DRAIN_PER_STEP {
            let Some(rx) = r.recv(&mut buf) else { break };
            self.raw = self.raw.wrapping_add(1);
            let f = &buf[..rx.len.min(buf.len())];
            let (Some(info), Some((band, tuned))) = (parse_beacon(f), self.at) else { continue };
            self.beacons = self.beacons.wrapping_add(1);
            self.results.heard(
                info.bssid,
                info.ssid,
                beacon_flags(f, info.capability),
                signal_percent(rx.rssi_dbm),
            );
            let channel = if info.channel != 0 { info.channel } else { tuned };
            self.hints.note(info.ssid, band, channel);
        }
        if env.now_ms() >= self.hop_at_ms {
            self.hop(r, env, domain);
        }
    }
}
