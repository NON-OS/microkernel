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

//! The writer every encoder stands on, and reserved bytes.

use nsh_wire::{Reader, WireError, Writer};

#[test]
fn reserved_bytes_must_be_zero() {
    assert_eq!(Reader::new(&[0, 0]).zero(2), Ok(()));
    assert_eq!(Reader::new(&[0, 1]).zero(2), Err(WireError::Reserved));
}

#[test]
fn the_writer_refuses_what_does_not_fit_and_what_its_prefix_cannot_say() {
    let mut buf = [0u8; 3];
    let mut w = Writer::new(&mut buf);
    assert_eq!(w.u32(1), Err(WireError::NoRoom));
    assert_eq!(w.done(), 0);
    assert_eq!(w.bytes8(&[0; 256]), Err(WireError::Oversize));
    assert_eq!(w.bytes8(&[5, 6]), Ok(()));
    assert_eq!(buf, [2, 5, 6]);
}
