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

//! The EDID read over I2C over AUX.

use crate::dp::aux::AuxError;
use crate::dp::i2c::read_edid;
use crate::model::sink::sink;
use crate::model::sink_aux::AuxEnd;

fn edid() -> Vec<u8> {
    (0..256u32).map(|i| (i * 7) as u8).collect()
}

#[test]
fn a_block_comes_back_in_sixteen_byte_reads_then_a_stop() {
    let s = sink(edid());
    let mut b = [0u8; 128];
    read_edid(&mut AuxEnd(s.clone()), 0, &mut b).unwrap();
    assert_eq!(&b[..], &edid()[..128]);
    assert_eq!(s.borrow().transfers, 1 + 8 + 1, "offset, eight reads, the stop");
    read_edid(&mut AuxEnd(s.clone()), 128, &mut b).unwrap();
    assert_eq!(&b[..], &edid()[128..]);
}

#[test]
fn defers_are_waited_out_and_a_nack_ends_the_read() {
    let s = sink(edid());
    s.borrow_mut().defers = 3;
    let mut b = [0u8; 16];
    read_edid(&mut AuxEnd(s.clone()), 0, &mut b).unwrap();
    assert_eq!(&b[..], &edid()[..16]);
    s.borrow_mut().nack = true;
    assert_eq!(read_edid(&mut AuxEnd(s), 0, &mut b), Err(AuxError::Refused));
}
