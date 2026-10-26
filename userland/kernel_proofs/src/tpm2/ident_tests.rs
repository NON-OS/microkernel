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

//! An empty window is no TPM, whichever way its bus reads; a FIFO part is
//! one only when its access register answers; a CRB part names itself.

use super::ident_rule::{classify, Interface};

const VALID: u8 = 0x80;

#[test]
fn an_empty_window_is_no_tpm_whether_it_reads_zeros_or_ones() {
    assert_eq!(classify(0, 0), None, "q35 with nothing at the TIS base reads zeros");
    assert_eq!(classify(u32::MAX, u8::MAX), None, "an empty bus reads all ones");
    assert_eq!(classify(u32::MAX, 0), None);
    assert_eq!(classify(0, u8::MAX), None);
}

#[test]
fn a_fifo_part_is_one_only_when_its_access_register_answers() {
    assert_eq!(classify(0x0000_0000, VALID), Some(Interface::Fifo));
    assert_eq!(classify(0x1D00_4000, VALID | 0x01), Some(Interface::Fifo));
    assert_eq!(classify(0x1D00_4000, 0x01), None, "tpmRegValidSts clear");
    assert_eq!(classify(0xFFFF_FFFF, VALID), Some(Interface::Fifo), "TIS 1.3: no id register");
    assert_eq!(classify(0x0000_000F, VALID), Some(Interface::Fifo), "TIS 1.3");
    assert_eq!(classify(0x0000_000F, 0x00), None);
}

#[test]
fn a_crb_part_names_itself_and_nothing_else_does() {
    assert_eq!(classify(0x1D00_4011, 0), Some(Interface::Crb));
    assert_eq!(classify(0x1D00_4011, u8::MAX), Some(Interface::Crb));
    for kind in [0x2, 0x3, 0x7, 0xE] {
        assert_eq!(classify(kind, VALID), None, "{kind:#x}");
    }
}
