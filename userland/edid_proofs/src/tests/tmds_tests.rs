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

//! The largest TMDS rate from the HDMI and HDMI Forum blocks.

use crate::build::base;
use crate::build_ext::{cta, data, with};
use crate::edid::Edid;

fn hdmi(units: u8) -> Vec<u8> {
    // OUI 00-0C-03 least significant first, physical address, flags, rate.
    data(3, &[0x03, 0x0C, 0x00, 0x10, 0x00, 0x00, units])
}

fn forum(units: u8) -> Vec<u8> {
    data(3, &[0xD8, 0x5D, 0xC4, 0x01, units, 0x00, 0x00])
}

fn scdb(units: u8) -> Vec<u8> {
    data(7, &[0x79, 0x00, 0x00, 0x01, units, 0x00])
}

fn tmds(blocks: &[u8]) -> u32 {
    let raw = with(base(), &[cta(blocks, &[])]);
    Edid::parse(&raw).unwrap().max_tmds_khz()
}

#[test]
fn the_hdmi_block_gives_the_rate_in_5_mhz_steps() {
    assert_eq!(tmds(&hdmi(68)), 340_000);
}

#[test]
fn the_forum_blocks_raise_it_past_340_mhz() {
    let mut b = hdmi(68);
    b.extend(forum(120));
    assert_eq!(tmds(&b), 600_000);
    let mut b = hdmi(68);
    b.extend(scdb(118));
    assert_eq!(tmds(&b), 590_000);
}

#[test]
fn a_forum_zero_leaves_the_hdmi_rate_and_nothing_is_unknown() {
    let mut b = hdmi(30);
    b.extend(forum(0));
    assert_eq!(tmds(&b), 150_000);
    assert_eq!(tmds(&[]), 0);
    assert_eq!(Edid::parse(&base()).unwrap().max_tmds_khz(), 0);
}

#[test]
fn a_short_hdmi_block_or_another_vendor_names_no_rate() {
    assert_eq!(tmds(&data(3, &[0x03, 0x0C, 0x00, 0x10, 0x00])), 0);
    assert_eq!(tmds(&data(3, &[0x01, 0x02, 0x03, 0, 0, 0, 0xFF])), 0);
}
