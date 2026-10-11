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

//! DisplayPort M/N.

use crate::engine::error::Error;
use crate::engine::mn::{compute, write, Mn, N, TU_64};
use crate::engine::pipe::Transcoder;
use crate::model::Gen9;

#[test]
fn ten_eighty_p_on_four_lanes_of_hbr() {
    // Data: 148.5 MHz x 24 / (270 MHz x 4 x 8) = 0.4125; link: 0.55.
    let mn = compute(148_500, 24, 4, 2700).unwrap();
    assert_eq!(mn, Mn { data_m: 216_269, link_m: 288_358 });
    let mut m = Gen9::new(1);
    write(&mut m, Transcoder::Edp, mn);
    assert_eq!(m.get(0x6_F030), TU_64 | 216_269);
    assert_eq!((m.get(0x6_F034), m.get(0x6_F040), m.get(0x6_F044)), (N, 288_358, N));
}

#[test]
fn a_mode_the_link_cannot_carry_is_refused() {
    assert_eq!(compute(533_250, 24, 2, 2700), Err(Error::Argument));
    assert!(compute(533_250, 24, 4, 5400).is_ok());
    assert_eq!(compute(0, 24, 4, 5400), Err(Error::Argument));
    assert_eq!(compute(148_500, 24, 0, 5400), Err(Error::Argument));
}

#[test]
fn the_ratios_round_to_nearest_and_stay_in_24_bits() {
    let mn = compute(162_000, 24, 1, 8100).unwrap();
    let (num, den) = (162_000u64 * 24 * N as u64, 810_000u64 * 8);
    assert_eq!(mn.data_m, ((num + den / 2) / den) as u32);
    assert!(mn.link_m < 1 << 24 && mn.data_m <= N);
}
