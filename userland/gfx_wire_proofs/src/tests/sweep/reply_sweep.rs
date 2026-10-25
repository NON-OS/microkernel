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

//! Driver to compositor: a reply is taken exactly when it carries the
//! wire's magic and version, answers the request asked (same op, output
//! and id), says a length from 4 to 256 and holds that many bytes; then
//! its status and body are the frame's. An honest reply round-trips.

use super::frames::{bytes, frame, header, le32, random_header};
use super::rng::Rng;
use crate::header::HDR_LEN;
use crate::reply::Reply;

#[test]
fn a_reply_is_taken_exactly_when_it_answers_what_was_asked() {
    let mut r = Rng::new(0x9E9_7A1E);
    let mut seen = [0usize; 5];
    for _ in 0..200_000 {
        let asked = random_header(&mut r);
        let mut h = asked;
        match r.below(8) {
            0 => h.op ^= 1,
            1 => h.output ^= 1,
            2 => h.request_id ^= 1,
            _ => {}
        }
        let n = r.below(260) as usize;
        let body = bytes(&mut r, n);
        let said = if r.below(4) == 0 { r.word() } else { 4 + body.len() as u32 };
        let mut payload = (-(r.below(3) as i32) * 11).to_le_bytes().to_vec();
        payload.extend_from_slice(&body);
        let f = frame(&mut r, h, said, &payload);
        let hh = header(&f);
        let same = hh.is_some_and(|x| {
            (x.op, x.output, x.request_id) == (asked.op, asked.output, asked.request_id)
        });
        let len = hh.map_or(0, |x| x.payload_len as usize);
        let whole = same && (4..=256).contains(&len) && f.len() >= HDR_LEN + len;
        let got = Reply::decode(&f, &asked);
        assert_eq!(got.is_ok(), whole, "{f:02X?}");
        if let Ok(g) = got {
            assert_eq!(g.status, le32(&f, HDR_LEN).unwrap() as i32);
            assert_eq!(g.body, &f[HDR_LEN + 4..HDR_LEN + len]);
        }
        let k = match (hh.is_some(), same, (4..=256).contains(&len)) {
            _ if whole => 0,
            (false, _, _) => 1,
            (true, false, _) => 2,
            (true, true, false) => 3,
            _ => 4,
        };
        seen[k] += 1;
    }
    // [taken, no magic or version, another request's, a length out of range, cut short]
    assert!(seen.iter().all(|&n| n >= 2_000), "{seen:?}");
}
