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

//! GET_DISPLAY_INFO's reply swept: taken exactly when whole and typed, each
//! scanout as the device said, the count its enabled entries.

use super::device::case;
use super::mutate;
use super::oracle::{display, DISPLAY_ROOM};
use super::reply;
use crate::device::cmd::get_display_info;
use crate::tests::wire_sweep::rng::Rng;

#[test]
fn every_display_info_reply_is_taken_exactly_when_well_formed() {
    let (mut r, mut taken) = (Rng(0x6475_6301), 0);
    for i in 0..3_000 {
        let good = reply::display(&mut r);
        let reply = mutate::mutate(&mut r, good, DISPLAY_ROOM);
        let want = display(&reply);
        let (got, room, live) = case(reply.clone(), |q| get_display_info(q, 1));
        assert_eq!(room, DISPLAY_ROOM, "the oracle's room is the driver's");
        assert!(live, "case {i}: a bad body reset the queue: {reply:?}");
        let Some(want) = want else {
            assert!(got.is_err(), "case {i}: taken though malformed: {reply:?}");
            continue;
        };
        let info = got.unwrap_or_else(|e| panic!("case {i}: refused ({e}) though well formed"));
        let seen: Vec<_> = info.scanouts.iter().map(|s| (s.width, s.height, s.enabled)).collect();
        assert_eq!(seen, want, "case {i}");
        assert_eq!(info.count as usize, want.iter().filter(|s| s.2 != 0).count(), "case {i}");
        taken += 1;
    }
    assert!((500..2_500).contains(&taken), "{taken} of 3000 taken: the sweep is lopsided");
}
