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

//! The small session bodies, at and just past every bound.

use nsh_wire::{data, empty, exit_status, signal, window, Signal, WireError};
use nsh_wire::{Window, MAX_DATA};

#[test]
fn terminal_bytes_are_one_to_sixteen_kib() {
    assert_eq!(data(&[]), Err(WireError::Value));
    assert_eq!(data(&[0; MAX_DATA]).map(<[u8]>::len), Ok(MAX_DATA));
    assert_eq!(data(&[0; MAX_DATA + 1]), Err(WireError::Oversize));
}

#[test]
fn a_window_is_one_to_a_thousand_cells_each_way() {
    let w = |c: u16, r: u16| [c.to_le_bytes(), r.to_le_bytes()].concat();
    assert_eq!(window(&w(1, 1000)), Ok(Window { cols: 1, rows: 1000 }));
    assert_eq!(window(&w(0, 24)), Err(WireError::Value));
    assert_eq!(window(&w(80, 1001)), Err(WireError::Value));
    assert_eq!(window(&w(80, 24)[..3]), Err(WireError::Short));
    assert_eq!(window(&[w(80, 24), vec![0]].concat()), Err(WireError::Trailing));
    let mut out = [0u8; 4];
    assert_eq!(Window { cols: 0, rows: 1 }.encode(&mut out), Err(WireError::Value));
    assert_eq!(Window { cols: 80, rows: 24 }.encode(&mut out), Ok(4));
    assert_eq!(out.to_vec(), w(80, 24));
}

#[test]
fn a_signal_is_one_or_two() {
    assert_eq!(signal(&[1]), Ok(Signal::Interrupt));
    assert_eq!(signal(&[2]), Ok(Signal::End));
    for b in (0..=255u8).filter(|b| !matches!(b, 1 | 2)) {
        assert_eq!(signal(&[b]), Err(WireError::Value), "{b}");
    }
    assert_eq!((signal(&[]), signal(&[1, 1])), (Err(WireError::Short), Err(WireError::Trailing)));
}

#[test]
fn exit_and_close_are_exact() {
    assert_eq!(exit_status(&(-9i32).to_le_bytes()), Ok(-9));
    assert_eq!(exit_status(&[0; 5]), Err(WireError::Trailing));
    assert_eq!((empty(&[]), empty(&[0])), (Ok(()), Err(WireError::Trailing)));
}
