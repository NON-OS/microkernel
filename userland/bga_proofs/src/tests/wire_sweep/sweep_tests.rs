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

//! The DISPI adapter's server under the wire sweep, its primaries on the
//! shared heap.

use super::heap::Heap;
use super::server_sweep::sweep;
use crate::dispi::DispiMode;
use crate::edid::Panel;
use crate::model::Adapter;
use nonos_gfx_serve::Server;

#[test]
fn the_dispi_adapter_answers_every_frame_as_the_wire_defines() {
    let mut a = Adapter::new();
    let b = a.bga(Panel::default(), DispiMode { width: 1024, height: 768, stride: 4096 });
    let mut s = Server::new(b, Heap::new(8192));
    let t = sweep(&mut s, 0xB6A_0001, 40_000, 7);
    assert!(
        t.unanswered >= 2_000 && t.refused >= 5_000 && t.taken >= 5_000 && t.shown >= 500,
        "{t:?}"
    );
}
