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

//! What a sound assembler hands up, stated apart from it (UVC 1.5, 2.4.3.3,
//! as recalled, unverified): a payload whose FID differs from the last
//! header's drops the frame in progress, cut short; ERR spoils its frame,
//! as does a frame past `max`; EOF ends a frame, handed up when nothing
//! spoiled it and it holds a byte. A payload whose bHeaderLength is under
//! 2 or past the payload has no FID to read, so it may belong to the frame
//! in progress or the next: it spoils both, and nothing is handed up until
//! an EOF seen after it. Empty payloads are nothing.

pub fn frames(payloads: &[Vec<u8>], max: usize) -> Vec<(usize, Vec<u8>)> {
    let (mut out, mut buf, mut fid, mut bad) = (Vec::new(), Vec::<u8>::new(), None, false);
    let mut lost = false;
    for (i, p) in payloads.iter().enumerate() {
        if p.is_empty() {
            continue;
        }
        let hlen = p[0] as usize;
        if hlen < 2 || hlen > p.len() {
            (bad, lost) = (true, true);
            continue;
        }
        let info = p[1];
        if fid.is_some_and(|f| f != info & 1) {
            (buf, bad, lost) = (Vec::new(), lost, false);
        }
        fid = Some(info & 1);
        let data = &p[hlen..];
        if info & 0x40 != 0 || buf.len() + data.len() > max {
            bad = true;
        }
        if !bad {
            buf.extend_from_slice(data);
        }
        if info & 2 != 0 {
            if !bad && !buf.is_empty() {
                out.push((i, core::mem::take(&mut buf)));
            }
            (buf, bad, lost) = (Vec::new(), false, false);
        }
    }
    out
}
