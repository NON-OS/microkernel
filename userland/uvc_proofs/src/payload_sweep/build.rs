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

//! A sound payload stream (UVC 1.5, 2.4.3.3, as recalled, unverified): two
//! to eight frames, each split into payloads that start with a header of 2
//! to 12 bytes whose bmHeaderInfo carries the frame's FID (flipped at each
//! new frame) and EOF on its last payload, then the frame's bytes.

use super::rng::Rng;

pub const FID: u8 = 1 << 0;
pub const EOF: u8 = 1 << 1;
pub const ERR: u8 = 1 << 6;
pub const EOH: u8 = 1 << 7;

/// One payload as the endpoint delivers it, and the frame it belongs to.
pub struct Payload {
    pub bytes: Vec<u8>,
    pub frame: usize,
}

pub struct Stream {
    pub payloads: Vec<Payload>,
    /// Each frame's bytes, whole.
    pub frames: Vec<Vec<u8>>,
    pub max: usize,
}

pub fn payload(r: &mut Rng, info: u8, data: &[u8]) -> Vec<u8> {
    let hlen = r.one_of(&[2, 2, 6, 12]);
    let mut p = vec![hlen as u8, info | EOH];
    p.extend((2..hlen).map(|i| i as u8));
    p.extend_from_slice(data);
    p
}

pub fn stream(r: &mut Rng) -> Stream {
    let max = 64usize << r.range(0, 8);
    let mut s = Stream { payloads: Vec::new(), frames: Vec::new(), max };
    let fid0 = r.range(0, 1) as u8;
    for f in 0..r.range(2, 8) {
        let len = r.range(1, max);
        let frame: Vec<u8> = (0..len).map(|i| (i * 7 + f) as u8).collect();
        let mut at = 0;
        while at < len {
            let n = r.range(1, (len - at).min(1024));
            let eof = if at + n == len { EOF } else { 0 };
            let info = (fid0 ^ (f as u8 & 1)) & FID | eof;
            let bytes = payload(r, info, &frame[at..at + n]);
            s.payloads.push(Payload { bytes, frame: f });
            at += n;
        }
        s.frames.push(frame);
    }
    s
}
