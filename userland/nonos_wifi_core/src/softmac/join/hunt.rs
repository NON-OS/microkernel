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

//! Find the network on the air before a join. The hunt listens: a network
//! that broadcasts its name is found by its beacon, so nothing is sent and
//! no saved network's name leaves the machine. Only for a network marked
//! hidden does it send a probe request naming it, and only on channels the
//! regulatory domain opens to probing. The channels the background scan last
//! heard the name on are tried first; then the whole plan, twice.

use super::listen::Heard;
use super::visit::{visit, Found};
use crate::regdom::{Band, Domain, ScanMode, ScanPlan};
use crate::softmac::radio::{Env, Radio};

/// Two and a half beacon intervals: one beacon lost to noise does not hide
/// the network on a channel it is known to be on.
const HINT_DWELL_MS: u64 = 250;
/// Just over one beacon interval per channel of a full sweep.
const SWEEP_DWELL_MS: u64 = 130;
const SWEEPS: usize = 2;

/// Hunt `ssid`; also returns how many beacons of any network were heard,
/// which tells "the radio hears nothing" from "not this network".
pub fn hunt<R: Radio, E: Env>(
    r: &mut R,
    env: &E,
    domain: Domain,
    ssid: &[u8],
    hidden: bool,
    hints: &[(Band, u8)],
    seq_start: u16,
) -> (Option<Found>, u32) {
    let mut heard = Heard { ssid, beacons: 0 };
    // One random sequence-number start for this scan burst's probes, carried
    // across its channels and never run on into the join that follows (E-70).
    let mut seq = seq_start & 0x0FFF;
    for &(band, ch) in hints {
        if let Some(f) = visit(r, env, &mut heard, (band, ch, false), HINT_DWELL_MS, &mut seq) {
            return (Some(f), heard.beacons);
        }
    }
    let (g2, g5) = r.bands();
    for _ in 0..SWEEPS {
        for step in ScanPlan::new(domain, g2, g5) {
            let probe = hidden && step.mode == ScanMode::Active;
            let at = (step.band, step.channel, probe);
            if let Some(f) = visit(r, env, &mut heard, at, SWEEP_DWELL_MS, &mut seq) {
                return (Some(f), heard.beacons);
            }
        }
    }
    (None, heard.beacons)
}
