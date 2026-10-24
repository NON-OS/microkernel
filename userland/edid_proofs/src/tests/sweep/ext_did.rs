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

//! DisplayID extensions: one section whose version, length, data block
//! tags, revisions and lengths may each be honest or a lie.

use super::gen::{record, Kind};
use super::rng::Rng;
use crate::build::seal;

/// The block, and the records a reader must find in it: those of each
/// honest timing block in an honest section.
pub struct Did {
    pub bytes: [u8; 128],
    pub records: Vec<Kind>,
}

pub fn did(r: &mut Rng) -> Did {
    let v2 = r.one_in(2);
    let version = match r.range(0, 3) {
        0 => r.byte(),
        n => [[0x10, 0x12, 0x13], [0x20, 0x21, 0x2F]][v2 as usize][n as usize - 1],
    };
    let (tag, unit) = if v2 { (0x22, 1) } else { (0x03, 10) };
    let mut payload = Vec::new();
    let mut records = Vec::new();
    // A block whose length lies moves every later block off its start.
    let mut aligned = true;
    while payload.len() < 116 && !r.one_in(3) {
        let honest_tag = !r.one_in(4);
        let rev = if r.one_in(5) { r.byte() } else { 0 };
        let mut body = Vec::new();
        let mut found = Vec::new();
        for _ in 0..r.range(0, 5) {
            let (rec, kind) = record(r, unit);
            body.extend_from_slice(&rec);
            found.push(kind);
        }
        let honest_len = !r.one_in(5);
        let len = if honest_len { body.len() } else { r.range(0, 255) as usize };
        payload.extend_from_slice(&[if honest_tag { tag } else { r.byte() }, rev, len as u8]);
        payload.extend_from_slice(&body);
        aligned &= honest_len;
        let whole = aligned && honest_tag && rev & 0x70 == 0 && version >> 4 == 1 + v2 as u8;
        if whole && payload.len() <= 121 {
            records.extend(found);
        }
    }
    let honest = payload.len() <= 121 && !r.one_in(5);
    let mut b = [0u8; 128];
    b[0] = 0x70;
    b[1] = version;
    b[2] = if honest { payload.len() as u8 } else { r.byte() };
    let n = payload.len().min(121);
    b[5..5 + n].copy_from_slice(&payload[..n]);
    if honest {
        seal(&mut b[1..6 + n]);
    }
    seal(&mut b);
    Did { bytes: b, records: if honest { records } else { vec![] } }
}
