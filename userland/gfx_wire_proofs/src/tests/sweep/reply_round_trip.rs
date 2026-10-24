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

//! An honest reply reads back as written; one too long for the wire is
//! not written at all.

use super::frames::{bytes, random_header};
use super::rng::Rng;
use crate::header::HDR_LEN;
use crate::reply::{reply, Reply};

#[test]
fn an_honest_reply_round_trips_and_an_oversize_one_is_not_written() {
    let mut r = Rng::new(7);
    for n in 0..=260usize {
        let (h, body) = (random_header(&mut r), bytes(&mut r, n));
        let mut out = [0u8; HDR_LEN + 4 + 252];
        match reply(&h, -5, &body, &mut out) {
            Ok(len) => {
                let g = Reply::decode(&out[..len], &h).unwrap();
                assert_eq!((g.status, g.body), (-5, &body[..]));
            }
            Err(_) => assert!(n > 252, "{n} bytes refused"),
        }
    }
}
