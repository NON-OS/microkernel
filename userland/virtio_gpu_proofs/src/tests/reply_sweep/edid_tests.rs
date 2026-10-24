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

//! GET_EDID's reply swept: the blob handed on exactly when the reply is
//! typed, its size word in range and within what was claimed, and the base
//! block's magic first; and then byte for byte what the device sent.

use super::device::case;
use super::mutate;
use super::oracle::{edid, EDID_ROOM};
use super::reply;
use crate::device::cmd::get_edid;
use crate::device::cmd::get_edid::EDID_MAX_LEN;
use crate::tests::wire_sweep::rng::Rng;

#[test]
fn every_edid_reply_is_handed_on_exactly_when_well_formed() {
    let (mut r, mut taken) = (Rng(0x6564_6901), 0);
    for i in 0..3_000 {
        let good = reply::edid(&mut r);
        let reply = mutate::mutate(&mut r, good, EDID_ROOM);
        let want = edid(&reply);
        let mut blob = [0u8; EDID_MAX_LEN];
        let (got, room, live) = case(reply.clone(), |q| get_edid(q, 0, &mut blob));
        assert_eq!(room, EDID_ROOM, "the oracle's room is the driver's");
        assert!(live, "case {i}: a bad body reset the queue");
        let got = got.ok().map(|n| blob[..n].to_vec());
        assert_eq!(got.as_ref().map(Vec::len), want.as_ref().map(Vec::len), "case {i}");
        assert_eq!(got, want, "case {i}: the blob handed on is not the device's");
        taken += got.is_some() as u32;
    }
    assert!((500..2_500).contains(&taken), "{taken} of 3000 taken: the sweep is lopsided");
}
