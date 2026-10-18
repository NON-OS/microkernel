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

//! Compositor to driver: a request is taken exactly when it carries the
//! wire's magic and version and the length it says fits both the frame and
//! the wire's 256 bytes; its payload is then exactly those bytes, and
//! anything the frame carries past them is not read.

use super::frames::{bytes, frame, header, random_header};
use super::rng::Rng;
use crate::header::HDR_LEN;
use crate::request::Request;

#[test]
fn a_request_is_taken_exactly_when_its_length_fits_the_frame_and_the_wire() {
    let mut r = Rng::new(0x2E9_0E57);
    let mut seen = [0usize; 4];
    for _ in 0..200_000 {
        let n = r.below(260) as usize;
        let body = bytes(&mut r, n);
        let said = if r.below(4) == 0 { r.word() } else { body.len() as u32 };
        let h = random_header(&mut r);
        let f = frame(&mut r, h, said, &body);
        let hh = header(&f);
        let end = hh.map(|x| HDR_LEN as u64 + x.payload_len as u64);
        let fits = end.is_some_and(|e| e <= 256 && e <= f.len() as u64);
        let got = Request::decode(&f);
        assert_eq!(got.is_ok(), fits, "{f:02X?}");
        if let (Ok(g), Some(e)) = (got, end) {
            assert_eq!((g.payload, g.header), (&f[HDR_LEN..e as usize], hh.unwrap()));
        }
        let k = match (hh.is_some(), end.is_some_and(|e| e <= 256)) {
            _ if fits => 0,
            (false, _) => 1,
            (true, false) => 2,
            _ => 3,
        };
        seen[k] += 1;
    }
    // [taken, no magic or version, past the wire's 256, past the frame]
    assert!(seen.iter().all(|&n| n >= 2_000), "{seen:?}");
}
