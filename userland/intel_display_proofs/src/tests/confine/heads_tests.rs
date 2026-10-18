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
//! Two heads, each in its own window of the table, flushing: confined.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::Rect;

use super::kit::confined;
use crate::tests::two_rig::two_rig;

#[test]
fn two_heads_flushing_stay_confined() {
    let (mut d, model, _) = two_rig();
    let px = vec![0x33u8; 1920 * 1080 * 4];
    for (o, (w, h)) in [(0u16, (1920, 1080)), (1, (1024, 768))] {
        let s = Shadow { bytes: &px, width: w, height: h, stride: w * 4, cursor_offset: 0 };
        d.flush(o, &[Rect { x: 0, y: 0, width: w, height: h }], &s).unwrap();
    }
    confined(&model.borrow(), "two heads");
}
