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

//! The FADT's event blocks: legacy fields, X_ fields preferred, memory
//! blocks and blocks past the ports refused, and the first FADT's length.

use crate::events::{parse_fadt, Block, BlockError, FadtError};
use crate::fadt_enc::{gpe1, q35, x, GPE0, PM1_CNT, PM1_STS};

#[test]
fn q35_layout_from_the_legacy_fields() {
    let f = parse_fadt(&q35(276)).unwrap();
    assert_eq!((f.sci, f.smi_cmd, f.acpi_enable, f.acpi_disable), (9, 0xB2, 2, 3));
    assert_eq!(f.pm1a_evt, Some(Block { port: PM1_STS, len: 4 }));
    assert_eq!(f.pm1a_cnt, Some(Block { port: PM1_CNT, len: 2 }));
    assert_eq!(f.gpe0, Some(Block { port: GPE0, len: 16 }));
    assert_eq!((f.pm1b_evt, f.pm1b_cnt, f.gpe1), (None, None, None));
    assert!(!f.hw_reduced);
}

#[test]
fn the_first_fadt_parses_and_a_shorter_table_does_not() {
    assert!(parse_fadt(&q35(116)).is_ok());
    assert_eq!(parse_fadt(&q35(115)), Err(FadtError::Table));
    let mut t = q35(276);
    t[..4].copy_from_slice(b"FACS");
    assert_eq!(parse_fadt(&t), Err(FadtError::Table));
}

#[test]
fn an_x_field_with_an_address_wins_and_a_zero_one_does_not() {
    let mut t = q35(276);
    x(&mut t, 220, 1, 128, 0x1800);
    x(&mut t, 148, 1, 32, 0);
    let f = parse_fadt(&t).unwrap();
    assert_eq!(f.gpe0, Some(Block { port: 0x1800, len: 16 }));
    assert_eq!(f.pm1a_evt, Some(Block { port: PM1_STS, len: 4 }), "zero X_ address");
}

#[test]
fn memory_blocks_and_blocks_past_the_ports_are_refused() {
    let mut t = q35(276);
    x(&mut t, 220, 0, 128, 0xFED8_0000);
    assert_eq!(parse_fadt(&t), Err(FadtError::Block(BlockError::Memory)));
    let mut t = q35(276);
    t[80..84].copy_from_slice(&0xFFF8u32.to_le_bytes());
    assert_eq!(parse_fadt(&t), Err(FadtError::Block(BlockError::Range)));
}

#[test]
fn gpe1_and_hardware_reduced() {
    let mut t = q35(276);
    gpe1(&mut t, 0x640, 8, 0x40);
    t[112..116].copy_from_slice(&(1u32 << 20).to_le_bytes());
    let f = parse_fadt(&t).unwrap();
    assert_eq!((f.gpe1, f.gpe1_base), (Some(Block { port: 0x640, len: 8 }), 0x40));
    assert!(f.hw_reduced);
}
