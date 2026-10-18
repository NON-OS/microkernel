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

//! One channel of the hunt: tune, probe for a hidden network where allowed,
//! and listen for the target.

use alloc::vec::Vec;

use super::listen::{listen, Heard};
use crate::dot11::mgmt::probe_request;
use crate::regdom::Band;
use crate::softmac::radio::{Env, Radio};

/// The 802.11b/g rates a probe offers, the CCK ones basic.
pub const PROBE_RATES: [u8; 8] = [0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24];

/// The target's beacon and where it was heard.
pub struct Found {
    pub beacon: Vec<u8>,
    pub band: Band,
    pub channel: u8,
}

pub(super) fn visit<R: Radio, E: Env>(
    r: &mut R,
    env: &E,
    h: &mut Heard,
    at: (Band, u8, bool),
    dwell: u64,
    seq: &mut u16,
) -> Option<Found> {
    let (band, channel, probe) = at;
    if !r.tune(band, channel) {
        return None;
    }
    let mut out = [0u8; 80];
    if let Some(n) =
        probe.then(|| probe_request(&mut out, r.mac(), h.ssid, &PROBE_RATES, *seq)).flatten()
    {
        r.send(&out[..n]);
        *seq = seq.wrapping_add(1) & 0x0FFF;
    }
    listen(r, env, h, dwell).map(|(beacon, named)| Found {
        beacon,
        band,
        channel: named.unwrap_or(channel),
    })
}
