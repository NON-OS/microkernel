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

//! The playback alternate setting a device offers, found in one of its
//! configuration descriptors (USB Audio 1.0 4.5, as recalled, unverified).

use nonos_usbnet::desc::walk;

use super::alt::{Alt, Playback, CLASS_AUDIO, CS_ENDPOINT, CS_INTERFACE, ENDPOINT, INTERFACE};
use super::alt::{ISOCH, SUBCLASS_STREAMING, UAC1};
use super::format::{is_pcm, plays_48k_stereo};

/// The first alternate setting in `config` that plays 48 kHz stereo 16-bit
/// PCM through an isochronous OUT endpoint.
pub fn playback(config: &[u8]) -> Option<Playback> {
    let mut cur = Alt::default();
    for d in walk(config) {
        match (d[1], d.get(2).copied()) {
            (INTERFACE, _) if d.len() >= 9 => {
                if cur.usable() {
                    return Some(cur.p);
                }
                let streaming = d[5] == CLASS_AUDIO && d[6] == SUBCLASS_STREAMING && d[7] == UAC1;
                let p = Playback { interface: d[2], alt: d[3], ..Playback::default() };
                cur = Alt { p, streaming, ..Alt::default() };
            }
            (CS_INTERFACE, Some(1)) => cur.pcm = is_pcm(d),
            (CS_INTERFACE, Some(2)) => cur.format = plays_48k_stereo(d),
            (ENDPOINT, Some(a)) if d.len() >= 7 && a & 0x80 == 0 && d[3] & 0x3 == ISOCH => {
                cur.p.ep_address = a;
                cur.p.sync = (d[3] >> 2) & 0x3;
                cur.p.max_packet = u16::from_le_bytes([d[4], d[5]]);
                cur.p.b_interval = d[6];
            }
            (CS_ENDPOINT, Some(1)) if d.len() >= 4 => cur.p.rate_control = d[3] & 1 != 0,
            _ => {}
        }
    }
    cur.usable().then_some(cur.p)
}
