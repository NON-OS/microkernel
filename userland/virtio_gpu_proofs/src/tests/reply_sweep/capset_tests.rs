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

//! GET_CAPSET_INFO's reply swept: taken exactly when whole and typed, its
//! three words as the device said.

use super::device::case;
use super::mutate;
use super::oracle_capset::{capset, CAPSET_ROOM};
use super::reply;
use crate::device::cmd::get_capset_info;
use crate::tests::wire_sweep::rng::Rng;

#[test]
fn every_capset_info_reply_is_taken_exactly_when_well_formed() {
    let (mut r, mut taken) = (Rng(0x6361_7001), 0);
    for i in 0..3_000 {
        let good = reply::capset(&mut r);
        let reply = mutate::mutate(&mut r, good, CAPSET_ROOM);
        let want = capset(&reply);
        let (got, room, live) = case(reply.clone(), |q| get_capset_info(q, 0));
        assert_eq!(room, CAPSET_ROOM, "the oracle's room is the driver's");
        assert!(live, "case {i}: a bad body reset the queue: {reply:?}");
        let got = got.ok().map(|c| (c.capset_id, c.max_version, c.max_size));
        assert_eq!(got, want, "case {i}: {reply:?}");
        taken += got.is_some() as u32;
    }
    assert!((500..2_500).contains(&taken), "{taken} of 3000 taken: the sweep is lopsided");
}
