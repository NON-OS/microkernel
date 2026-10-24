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
//! A UAS interface in a configuration: the mass-storage interface's
//! alternate of protocol 62h, and its four bulk pipes, each endpoint
//! followed by a Pipe Usage descriptor (24h) naming it: 1 command, 2
//! status, 3 data-in, 4 data-out. At SuperSpeed the companion's
//! bmAttributes 4:0 give the streams it takes, as a power of two.

const INTERFACE: u8 = 0x04;
const ENDPOINT: u8 = 0x05;
const SS_COMPANION: u8 = 0x30;
const PIPE_USAGE: u8 = 0x24;
const MASS_STORAGE: (u8, u8, u8) = (0x08, 0x06, 0x62);

use crate::pipe::{Pipe, UasPipes};

/// The first UAS alternate in configuration `raw` with all four pipes.
pub fn uas_pipes(raw: &[u8]) -> Option<UasPipes> {
    let (mut cur, mut inside, mut last) = (UasPipes::default(), false, Pipe::default());
    let mut i = raw.first().map_or(0, |&l| l as usize).max(2);
    while let Some(d) = raw.get(i..i + raw.get(i).map_or(0, |&l| l as usize)) {
        i += d.len().max(1);
        match (d.len() >= 2).then(|| d[1]) {
            Some(INTERFACE) if d.len() >= 9 => {
                if inside && cur.whole() {
                    return Some(cur);
                }
                inside = (d[5], d[6], d[7]) == MASS_STORAGE;
                cur = UasPipes { interface: d[2], alt: d[3], ..UasPipes::default() };
            }
            Some(ENDPOINT) if inside && d.len() >= 7 && d[3] & 0x3 == 0x2 => {
                let max_packet = u16::from_le_bytes([d[4], d[5]]);
                last = Pipe { address: d[2], max_packet, ..Pipe::default() };
            }
            Some(SS_COMPANION) if inside && d.len() >= 4 => {
                (last.burst, last.streams) = (d[2] & 0x0F, d[3] & 0x1F);
            }
            Some(PIPE_USAGE) if inside && d.len() >= 3 && last.address != 0 => {
                let into = match d[2] {
                    1 if last.address & 0x80 == 0 => &mut cur.command,
                    2 if last.address & 0x80 != 0 => &mut cur.status,
                    3 if last.address & 0x80 != 0 => &mut cur.data_in,
                    4 if last.address & 0x80 == 0 => &mut cur.data_out,
                    _ => continue,
                };
                *into = last;
            }
            _ => {}
        }
    }
    (inside && cur.whole()).then_some(cur)
}
