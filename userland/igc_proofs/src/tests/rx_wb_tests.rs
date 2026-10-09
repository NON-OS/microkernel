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

//! The receive write-back verdict on fixed vectors: DD and EOP with no RXE
//! and a length in 1..=1514 is a frame; anything else written back is
//! dropped; no DD is not yet the driver's.

use crate::constants::rx_bits::{RXDEXT_STATERR_RXE, RXD_STAT_DD, RXD_STAT_EOP};
use crate::queue::rx_wb::{parse, Rx};

#[test]
fn write_back_bits_are_the_igc_defines_h_values() {
    assert_eq!((RXD_STAT_DD, RXD_STAT_EOP, RXDEXT_STATERR_RXE), (0x01, 0x02, 0x8000_0000));
}

#[test]
fn parse_vectors() {
    let whole = RXD_STAT_DD | RXD_STAT_EOP;
    assert_eq!(parse(whole, 60), Rx::Frame(60));
    assert_eq!(parse(whole, 1514), Rx::Frame(1514));
    assert_eq!(parse(whole, 1515), Rx::Drop);
    assert_eq!(parse(whole, 0), Rx::Drop);
    assert_eq!(parse(whole | RXDEXT_STATERR_RXE, 60), Rx::Drop);
    assert_eq!(parse(RXD_STAT_DD, 60), Rx::Drop);
    assert_eq!(parse(RXD_STAT_EOP, 60), Rx::NotDone);
    assert_eq!(parse(0, 60), Rx::NotDone);
}
