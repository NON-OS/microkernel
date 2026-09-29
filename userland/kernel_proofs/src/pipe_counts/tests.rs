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

use super::buffer::PipeBuffer;

#[test]
fn an_extra_remove_leaves_the_pipe_without_readers_or_writers() {
    let pipe = PipeBuffer::new();
    assert!(pipe.has_readers() && pipe.has_writers());
    pipe.remove_reader();
    pipe.remove_reader();
    pipe.remove_writer();
    pipe.remove_writer();
    assert!(!pipe.has_readers());
    assert!(!pipe.has_writers());
    pipe.add_reader();
    assert!(pipe.has_readers());
}

#[test]
fn bytes_come_out_in_order_across_the_ring_end() {
    let mut pipe = PipeBuffer::new();
    let filler = [0u8; 65000];
    assert_eq!(pipe.write(&filler), Ok(65000));
    let mut sink = [0u8; 65000];
    assert_eq!(pipe.read(&mut sink), Ok(65000));
    let data: [u8; 1000] = core::array::from_fn(|i| i as u8);
    assert_eq!(pipe.write(&data), Ok(1000));
    let mut out = [0u8; 1000];
    assert_eq!(pipe.read(&mut out), Ok(1000));
    assert_eq!(out, data);
}
