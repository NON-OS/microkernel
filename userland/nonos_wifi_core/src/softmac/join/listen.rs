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

//! One dwell of the hunt: read frames off the radio until the target's
//! beacon or probe response arrives or the dwell ends.

use alloc::vec::Vec;

use crate::dot11::parse::parse_beacon;
use crate::softmac::radio::{Env, Radio};

pub(super) struct Heard<'a> {
    pub(super) ssid: &'a [u8],
    pub(super) beacons: u32,
}

/// The frame and the channel its DS Parameter Set names, if it names one: on
/// 2.4 GHz a beacon leaks into the neighbouring channels, so the channel the
/// AP says is the one to join on.
pub(super) fn listen<R: Radio, E: Env>(
    r: &mut R,
    env: &E,
    h: &mut Heard,
    dwell_ms: u64,
) -> Option<(Vec<u8>, Option<u8>)> {
    let until = env.now_ms().saturating_add(dwell_ms);
    let mut buf = [0u8; 2400];
    while env.now_ms() < until {
        let Some(rx) = r.recv(&mut buf) else { continue };
        let frame = &buf[..rx.len.min(buf.len())];
        let Some(info) = parse_beacon(frame) else { continue };
        h.beacons += 1;
        if info.ssid == h.ssid {
            return Some((frame.to_vec(), (info.channel != 0).then_some(info.channel)));
        }
    }
    None
}
