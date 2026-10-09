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

use crate::image::jpeg::bits::BitReader;

fn is_rst(m: u8) -> bool {
    (0xD0..=0xD7).contains(&m)
}

/// The next marker at or after `p` that is not byte stuffing, fill or a
/// restart: (marker code, offset just past it).
pub fn next_marker(b: &[u8], mut p: usize) -> Option<(u8, usize)> {
    while p + 1 < b.len() {
        let m = b[p + 1];
        if b[p] == 0xFF && m != 0 && m != 0xFF && !is_rst(m) {
            return Some((m, p + 2));
        }
        p += 1;
    }
    None
}

/// Where the marker parser resumes after a scan: at the marker the reader
/// stopped on, else where it stopped reading.
pub fn resume_at(br: &BitReader) -> usize {
    if br.marker_hit.is_some() {
        br.pos.saturating_sub(2)
    } else {
        br.pos
    }
}

/// Step over a restart marker between intervals, dropping the partial
/// byte before it. A restart of any number is accepted, as decoders that
/// resynchronise do; false when a different marker ends the scan instead.
pub fn restart(br: &mut BitReader) -> bool {
    br.align_to_byte();
    br.flush();
    let m = match br.marker_hit.take() {
        Some(m) => m,
        None => {
            let b = br.data;
            let mut p = br.pos;
            while p + 1 < b.len() && !(b[p] == 0xFF && b[p + 1] != 0 && b[p + 1] != 0xFF) {
                p += 1;
            }
            if p + 1 >= b.len() {
                return false;
            }
            br.pos = p + 2;
            b[p + 1]
        }
    };
    if is_rst(m) {
        return true;
    }
    br.marker_hit = Some(m);
    false
}
