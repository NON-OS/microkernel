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

use super::loader::legacy::{stem as old_stem, Legacy};
use super::loader::name::{packed, stem};
use super::loader::tlv::{parse, Version};
use super::rig::{tlv, BZRU, SOLAR, TYP};

/// A BlazarI: CNVi 0x0190, CNVr 0x0291, variant 0x1e.
pub const BZRI: (u32, u32, u8, &str) = (0x0100_0900, 0x0200_0910, 0x1e, "ibt-0190-0291-iml");

fn v(part: (u32, u32, u8, &str), img: u8) -> Version {
    parse(&tlv(part, img, [0; 3], 0)).unwrap()
}

#[test]
fn cnvi_and_cnvr_are_type_and_step_packed_and_byte_swapped() {
    assert_eq!(packed(0x410), 0x0041);
    assert_eq!(packed(0x401), 0x1040);
    assert_eq!(packed(0x0200_0910), 0x0291);
    assert_eq!(packed(0xf100_f410), 0x0141, "bits outside type and step are not the name's");
}

#[test]
fn ax210_ax211_and_be200_ask_for_their_files() {
    assert_eq!(stem(&v(TYP, 0x01), true), "ibt-0041-0041");
    assert_eq!(stem(&v(SOLAR, 0x01), true), "ibt-1040-0041");
    assert_eq!(stem(&v(BZRU, 0x01), true), "ibt-0291-0291");
    assert_eq!(stem(&v(BZRU, 0x03), true), "ibt-0291-0291", "BlazarU has no loader");
    assert_eq!(stem(&v(TYP, 0x03), false), "ibt-0041-0041");
}

#[test]
fn blazar_and_later_take_an_intermediate_loader_first() {
    assert_eq!(stem(&v(BZRI, 0x01), true), "ibt-0190-0291-iml");
    assert_eq!(stem(&v(BZRI, 0x03), true), "ibt-0190-0291-iml", "matched against its loader");
    let mut iml = v(BZRI, 0x02);
    assert_eq!(stem(&iml, true), "ibt-0190-0291");
    iml.fw_id = b"usb".to_vec();
    assert_eq!(stem(&iml, true), "ibt-0190-0291-usb");
    assert_eq!(stem(&iml, false), "ibt-0190-0291-usb");
}

#[test]
fn an_ax201_is_named_in_decimal_and_only_jfp_to_ccp_so() {
    let l = Legacy { variant: 0x13, hw_revision: 32, fw_variant: 6, fw_revision: 1, build: [0; 3] };
    assert_eq!(old_stem(&l).as_deref(), Some("ibt-19-32-1"));
    assert_eq!(old_stem(&Legacy { variant: 0x14, ..l }).as_deref(), Some("ibt-20-32-1"));
    assert!(old_stem(&Legacy { variant: 0x0b, ..l }).is_none(), "SfP is named otherwise");
    assert!(old_stem(&Legacy { variant: 0x15, ..l }).is_none());
}
