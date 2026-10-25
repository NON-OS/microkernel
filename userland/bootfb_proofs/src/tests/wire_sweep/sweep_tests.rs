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

//! The boot framebuffer's server under the wire sweep.

use super::server_sweep::sweep;
use crate::fb::Layout;
use crate::tests::serve::run;

#[test]
fn the_boot_framebuffer_answers_every_frame_as_the_wire_defines() {
    let mut mem = vec![0u32; 20 * 8];
    let (mut s, _) = run(Layout::BlueFirst, &mut mem);
    let t = sweep(&mut s, 0x00B0_07FB, 40_000, 7);
    assert!(t.unanswered >= 2_000 && t.refused >= 5_000 && t.taken >= 5_000, "{t:?}");
}
