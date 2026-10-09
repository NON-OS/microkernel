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

//! The advanced transmit data descriptor against igc_base.h: the literal
//! IGC_ADVTXD_* values, then whole-word vectors and the field positions.

use crate::constants::tx_bits::*;
use crate::queue::tx_encode::{cmd_type_len, olinfo_status};

#[test]
fn bits_are_the_igc_base_h_values() {
    assert_eq!(ADVTXD_DTYP_DATA, 0x0030_0000);
    assert_eq!(ADVTXD_DCMD_EOP, 0x0100_0000);
    assert_eq!(ADVTXD_DCMD_IFCS, 0x0200_0000);
    assert_eq!(ADVTXD_DCMD_RS, 0x0800_0000);
    assert_eq!(ADVTXD_DCMD_DEXT, 0x2000_0000);
    assert_eq!(ADVTXD_PAYLEN_SHIFT, 14);
    assert_eq!(TXD_STAT_DD, 0x0000_0001);
}

#[test]
fn whole_word_vectors() {
    assert_eq!(cmd_type_len(60), 0x2B30_003C);
    assert_eq!(cmd_type_len(1514), 0x2B30_05EA);
    assert_eq!(olinfo_status(60), 0x000F_0000);
    assert_eq!(olinfo_status(1514), 0x017A_8000);
}

#[test]
fn every_length_lands_in_its_field_and_nowhere_else() {
    for len in 0..=u16::MAX {
        let c = cmd_type_len(len);
        assert_eq!(c & 0xFFFF, len as u32, "length in bits 15:0");
        assert_eq!((c >> 20) & 0xF, 0x3, "DTYP data");
        assert_ne!(c & (1 << 29), 0, "DEXT");
        assert_ne!(c & (1 << 27), 0, "RS");
        assert_ne!(c & (1 << 25), 0, "IFCS");
        assert_ne!(c & (1 << 24), 0, "EOP");
        assert_eq!(c & 0xC000_0000, 0, "no VLE, no TSE");
        assert_eq!(c & (0x14 << 24), 0, "bits 26 and 28, unused by igc here, clear");
        assert_eq!((c >> 16) & 0xF, 0, "no IEEE 1588 timestamp request");
        let o = olinfo_status(len);
        assert_eq!(o >> 14, len as u32, "PAYLEN from bit 14");
        assert_eq!(o & 0x3FFF, 0, "no checksum options, DD starts clear");
    }
}
