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

//! The wire sweep against the server's own model displays: two outputs
//! listing their modes, and one range adapter with a cursor.

use crate::model::{Heads, Pool, Vram};
use crate::server::Server;
use crate::sweep::server_sweep::sweep;

#[test]
fn two_listing_outputs_answer_every_frame_as_the_wire_defines() {
    let mut s = Server::new(Heads::new(&[(1920, 1080), (1280, 1024)]), Pool::new(8192));
    let t = sweep(&mut s, 0x5E7E_0001, 60_000, 7);
    assert!(
        t.unanswered >= 3_000 && t.refused >= 10_000 && t.taken >= 10_000 && t.shown >= 1_000,
        "{t:?}"
    );
}

#[test]
fn a_range_adapter_with_a_cursor_answers_every_frame_as_the_wire_defines() {
    let mut s = Server::new(Vram::new(true, true), Pool::new(8192));
    let t = sweep(&mut s, 0x5E7E_0002, 60_000, 7);
    assert!(
        t.unanswered >= 3_000 && t.refused >= 10_000 && t.taken >= 10_000 && t.shown >= 1_000,
        "{t:?}"
    );
}

#[test]
fn the_shared_heap_holds_a_server_through_the_sweep() {
    let mut s = Server::new(Heads::new(&[(800, 600)]), crate::sweep::heap::Heap::new(4096));
    let t = sweep(&mut s, 0x5E7E_0003, 20_000, 7);
    assert!(t.taken >= 3_000 && t.shown >= 300, "{t:?}");
}
