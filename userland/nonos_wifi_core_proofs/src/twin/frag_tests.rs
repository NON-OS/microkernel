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

//! Fragments. The station joins no fragment: every frame with More
//! Fragments set or a fragment number other than zero is refused, sealed or
//! in the clear, so there is no fragment cache to mix keys, packet numbers
//! or plaintext in, and none to empty on association or rekey. A whole
//! frame after them is taken as any other.

use super::air::{seal, IP};
use super::watch::{delivered, put};
use crate::ap_sim::AA;
use crate::soft::data_frames::from_ds_of;
use crate::soft::join::{joined, sta_of};
use crate::soft::rig::Station;

fn plain(st: &Station, more: bool, number: u8, body: &[u8]) -> Vec<u8> {
    let mut f = from_ds_of(AA, sta_of(st), body);
    f[1] |= if more { 0x04 } else { 0 };
    f[22] |= number;
    f
}

#[test]
fn no_fragment_is_joined_or_delivered_sealed_or_clear() {
    let mut st = joined();
    let start = delivered(&st);
    let (head, tail) = (&IP[..6], &IP[6..]);
    // The first fragment and the last, each sealed at its own packet number.
    put(&mut st, |s| seal(s, &plain(s, true, 0, head), 1));
    put(&mut st, |s| seal(s, &plain(s, false, 1, tail), 2));
    // A clear fragment after a sealed one, and the other way about.
    put(&mut st, |s| seal(s, &plain(s, true, 0, head), 3));
    put(&mut st, |s| plain(s, false, 1, tail));
    put(&mut st, |s| plain(s, true, 0, &IP));
    put(&mut st, |s| seal(s, &plain(s, false, 1, tail), 4));
    // Fragments whose body alone would read as a whole packet.
    put(&mut st, |s| seal(s, &plain(s, true, 0, &IP), 5));
    put(&mut st, |s| seal(s, &plain(s, false, 1, &IP), 6));
    assert_eq!(delivered(&st), start, "no fragment reached the stack");
    // A whole sealed frame is no fragment and is taken.
    put(&mut st, |s| seal(s, &plain(s, false, 0, &IP), 7));
    assert_eq!(delivered(&st), start + 1);
    assert!(st.session().is_some());
}
