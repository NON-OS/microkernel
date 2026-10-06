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

//! The host memory buffer offered to a DRAM-less controller: how it is
//! sized and cut. The real parser reads the four identify fields at their
//! spec offsets (NVMe 1.4, figure 247: bytes 272, 276, 332 and 336).

use crate::admin::hmb::{plan, HmbAsk, HmbPlan};
use crate::admin::ControllerIdentity;

pub(crate) fn ask(preferred: u32, minimum: u32, min_piece: u32, max_pieces: u16) -> HmbAsk {
    HmbAsk { preferred, minimum, min_piece, max_pieces }
}

#[test]
fn identify_names_the_buffer_at_its_offsets() {
    let mut page = vec![0u8; 4096];
    page[0x110..0x114].copy_from_slice(&16_384u32.to_le_bytes());
    page[0x114..0x118].copy_from_slice(&2_560u32.to_le_bytes());
    page[0x14c..0x150].copy_from_slice(&16u32.to_le_bytes());
    page[0x150..0x152].copy_from_slice(&32u16.to_le_bytes());
    let id = ControllerIdentity::parse(&page);
    assert_eq!(id.hmb, ask(16_384, 2_560, 16, 32));
}

#[test]
fn a_dram_less_drive_gets_its_preferred_size_in_4_mib_pieces() {
    // Shaped like a Samsung PM991: prefers 64 MiB, needs 10 MiB.
    let p = plan(ask(16_384, 2_560, 0, 0)).unwrap();
    assert_eq!(p, HmbPlan { piece_pages: 1024, pieces: 16 });
    assert_eq!(p.pages(), 16_384);
}

#[test]
fn a_drive_that_wants_none_gets_none() {
    assert_eq!(plan(ask(0, 0, 0, 0)), None);
    assert_eq!(ControllerIdentity::parse(&[0u8; 4096]).hmb, ask(0, 0, 0, 0));
}

#[test]
fn the_budget_caps_the_preferred_size_but_never_below_the_minimum() {
    assert_eq!(plan(ask(1 << 20, 0, 0, 0)).unwrap().pages(), 16_384);
    assert!(plan(ask(1 << 20, 20_000, 0, 0)).unwrap().pages() >= 20_000);
    assert_eq!(plan(ask(1 << 20, 40_000, 0, 0)), None, "more than 128 MiB is not given");
}

#[test]
fn a_small_ask_is_one_piece_of_its_own_size() {
    assert_eq!(plan(ask(512, 256, 0, 0)), Some(HmbPlan { piece_pages: 512, pieces: 1 }));
}

#[test]
fn the_controllers_piece_limits_hold() {
    // At most 4 descriptors: 4 pieces of 4 MiB, which covers the minimum.
    assert_eq!(plan(ask(16_384, 2_560, 0, 4)), Some(HmbPlan { piece_pages: 1024, pieces: 4 }));
    // Too few descriptors to reach the minimum: nothing is offered.
    assert_eq!(plan(ask(16_384, 8_192, 0, 2)), None);
    // A smallest piece above one map: nothing is offered.
    assert_eq!(plan(ask(16_384, 0, 2048, 0)), None);
    // A smallest piece below one map is honoured by the map size.
    assert!(plan(ask(16_384, 0, 512, 0)).unwrap().piece_pages >= 512);
}
