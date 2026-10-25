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

//! What the driver says of itself: CAPS and MODES.

use super::ask::ask;
use super::util::up;
use crate::bench::Bench;
use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::mode::{CURRENT, PREFERRED};
use nonos_gfx_wire::{kind, ops, Caps, Modes};

#[test]
fn caps_names_svga_ii_its_range_and_what_vram_holds() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |s| s.max = (2560, 1600));
    let c = Caps::decode(&ask(&mut d.server, ops::CAPS, &[]).1).unwrap();
    assert_eq!((c.kind, c.outputs, c.flags), (kind::SVGA_II, 1, MODE_SET | MODE_RANGE));
    assert_eq!((c.max_width, c.max_height, c.surface_budget), (2560, 1600, 16 << 20));
}

#[test]
fn modes_lists_the_current_mode_first_then_only_what_fits() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let m = Modes::decode(&ask(&mut d.server, ops::MODES, &[]).1).unwrap();
    let sizes: Vec<_> = m.list[..m.len].iter().map(|m| (m.width, m.height)).collect();
    assert_eq!(sizes, [(1024, 768), (800, 600), (640, 480)]);
    assert_eq!(m.list[0].flags & (CURRENT | PREFERRED), CURRENT | PREFERRED);
    let mut small = Bench::new(64 << 10, 2 << 20);
    let mut d = up(&mut small, |_| {});
    let m = Modes::decode(&ask(&mut d.server, ops::MODES, &[]).1).unwrap();
    let sizes: Vec<_> = m.list[..m.len].iter().map(|m| (m.width, m.height)).collect();
    assert_eq!(sizes, [(800, 600), (640, 480)], "1024x768 is not offered in 2 MiB");
}
