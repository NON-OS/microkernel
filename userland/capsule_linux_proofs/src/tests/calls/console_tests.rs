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

//! The console's input: a message of exactly 0x04 is an end of file, read
//! once and in its place; bytes are read a line at a time, never past an
//! end of file; nothing waiting is Empty, which parks the reader.

use crate::console::queue::Queue;
use crate::console::queue_piece::{Taken, EOF, MOST};
use crate::console::wipe::wipe;

fn bytes(b: &[u8]) -> Taken {
    Taken::Bytes(b.to_vec())
}

#[test]
fn only_the_single_byte_is_an_end_of_file() {
    let mut q = Queue::new();
    assert_eq!(q.take(64), Taken::Empty);
    q.push(&[EOF]);
    q.push(&[EOF, b'\n']);
    q.push(b"a\x04");
    q.push(b"");
    assert_eq!(q.take(64), Taken::Eof);
    assert_eq!(q.take(64), bytes(&[EOF, b'\n']));
    assert_eq!(q.take(64), bytes(b"a\x04"));
    assert!(!q.ready() && q.take(64) == Taken::Empty);
}

#[test]
fn bytes_come_a_line_at_a_time_and_stop_at_an_end_of_file() {
    let mut q = Queue::new();
    q.push(b"hello\nworld\n");
    q.push(b"par");
    q.push(b"tial");
    q.push(&[EOF]);
    q.push(b"after\n");
    assert_eq!(q.queued(), 12 + 7 + 6);
    assert_eq!(q.take(64), bytes(b"hello\n"));
    assert_eq!(q.take(3), bytes(b"wor"));
    assert_eq!(q.take(64), bytes(b"ld\n"));
    assert_eq!(q.take(64), bytes(b"partial"));
    assert_eq!(q.take(64), Taken::Eof);
    assert_eq!(q.take(64), bytes(b"after\n"));
    assert_eq!(q.queued(), 0);
    assert_eq!(q.take(64), Taken::Empty);
}

#[test]
fn a_full_queue_takes_no_more_and_a_flush_empties_it() {
    let mut q = Queue::new();
    let line = vec![b'x'; 4096];
    while !q.full() {
        q.push(&line);
    }
    assert_eq!(q.queued(), MOST);
    q.clear();
    assert!(!q.ready() && q.queued() == 0 && !q.full());
    let mut secret = *b"typed";
    wipe(&mut secret);
    assert_eq!(secret, [0; 5]);
}
