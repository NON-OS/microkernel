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
//! One class-specific or endpoint descriptor of an alternate setting read
//! into what is known of it.

use super::alt::{CS_ENDPOINT, CS_INTERFACE, ENDPOINT, ISOCH};
use super::format::{format_i, general};
use super::pick::{Alt, Ep};

/// bmAttributes bits 5:4 of a 2.0 endpoint: 1 is explicit feedback.
const FEEDBACK: u8 = 1;

pub(super) fn read(cur: &mut Alt, d: &[u8]) {
    match (d[1], d.get(2).copied()) {
        (CS_INTERFACE, Some(1)) if d.len() >= 4 => {
            let (pcm, channels) = general(cur.s.version, d);
            (cur.pcm, cur.s.terminal) = (pcm, d[3]);
            cur.s.channels = channels.unwrap_or(cur.s.channels);
        }
        (CS_INTERFACE, Some(2)) => {
            let f = format_i(cur.s.version, d);
            cur.format = f.is_some();
            cur.s.channels = f.flatten().unwrap_or(cur.s.channels);
        }
        (ENDPOINT, Some(a)) if d.len() >= 7 && d[3] & 0x3 == ISOCH => {
            let sync = (d[3] >> 2) & 0x3;
            let ep = Ep {
                address: a,
                sync,
                max_packet: u16::from_le_bytes([d[4], d[5]]),
                b_interval: d[6],
            };
            if a & 0x80 == 0 {
                cur.out = Some(ep);
            } else if (d[3] >> 4) & 0x3 != FEEDBACK {
                cur.inp = Some(ep);
            }
        }
        (CS_ENDPOINT, Some(1)) if d.len() >= 4 => {
            cur.s.rate_control = cur.s.version == 1 && d[3] & 1 != 0;
        }
        _ => {}
    }
}
