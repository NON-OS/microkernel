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
//! Surviving a codec status register that is telling the truth about nothing.

use crate::controller::probe;
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::sim;

#[test]
fn bits_with_no_codec_behind_them_are_reported_present_but_not_working() {
    let s = sim::start(vec![(0, sim_codec(&alc236_hp()))]);
    let mut link = s.link();
    let found = probe(&mut link, 0b0000_0000_0000_0011);
    assert_eq!(found[0].ok, 1);
    assert_eq!((found[1].present, found[1].ok, found[1].vendor_id), (1, 0, 0));
    // The link still works after the silent address.
    assert_eq!(link.send(0x000f_0000), Ok(0x10ec_0236));
}

#[test]
fn the_sixteenth_status_bit_is_not_treated_as_a_codec() {
    let s = sim::start(vec![]);
    let mut link = s.link();
    let found = probe(&mut link, 1 << 15);
    for (i, p) in found.iter().enumerate() {
        assert_eq!(p.present, 0, "slot {i} answered to a bit with no codec behind it");
    }
}
