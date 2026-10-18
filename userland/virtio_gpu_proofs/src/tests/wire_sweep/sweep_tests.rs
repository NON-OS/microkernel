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
//! virtio-gpu's server under the wire sweep, its primaries on the shared
//! heap, the host model answering every command.

use super::server_sweep::sweep;
use crate::tests::gpu::rig::rig;

#[test]
fn virtio_gpu_answers_every_frame_as_the_wire_defines() {
    let mut r = rig();
    let t = sweep(&mut r.d.server, 0x0717_0001, 20_000, 7);
    assert!(
        t.unanswered >= 1_000 && t.refused >= 2_500 && t.taken >= 2_500 && t.shown >= 200,
        "{t:?}"
    );
}
