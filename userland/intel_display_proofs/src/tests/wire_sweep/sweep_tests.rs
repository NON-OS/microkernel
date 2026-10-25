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

//! The Intel display engine's server under the wire sweep, on the model
//! board with its firmware-lit output, its primaries on the shared heap.

use super::heap::Heap;
use super::server_sweep::sweep;
use crate::tests::wire_rig::rig;
use nonos_gfx_serve::Server;

#[test]
fn the_display_engine_answers_every_frame_as_the_wire_defines() {
    let (d, _model) = rig();
    let mut s = Server::new(d, Heap::new(1 << 16));
    let t = sweep(&mut s, 0x1D15_0001, 4_000, 7);
    assert!(t.unanswered >= 150 && t.refused >= 400 && t.taken >= 400 && t.shown >= 40, "{t:?}");
}
