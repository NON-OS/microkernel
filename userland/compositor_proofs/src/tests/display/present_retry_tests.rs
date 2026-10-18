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

//! A rectangle the display refused is shown by a later frame. The display
//! here keeps what reached it, as the scanout does; a refused present
//! leaves its rectangle as it was.

use crate::damage::{DamageAccumulator, Rect};
use crate::drain_damage::drain_damage;

struct Display {
    damage: DamageAccumulator,
    /// Presents to refuse before the driver answers again.
    refuse: usize,
    shown: Vec<(u32, u32, u32, u32)>,
}

fn show(d: &mut Display, r: Rect) -> Result<(), &'static str> {
    if d.refuse > 0 {
        d.refuse -= 1;
        return Err("gfx transfer: driver rejected");
    }
    d.shown.push((r.x, r.y, r.width, r.height));
    Ok(())
}

#[test]
fn a_refused_rectangle_is_shown_by_the_next_frame() {
    let mut d = Display { damage: DamageAccumulator::new(), refuse: 1, shown: Vec::new() };
    // A menu opens: one damaged rectangle, and the driver times out on it.
    d.damage.accumulate(Rect { x: 10, y: 20, width: 30, height: 40 });
    assert!(drain_damage(&mut d, |d| &mut d.damage, show).is_err());
    assert!(d.shown.is_empty());
    // Nothing else changes on screen. The next frame must still show it.
    assert!(drain_damage(&mut d, |d| &mut d.damage, show).is_ok());
    assert_eq!(d.shown, vec![(10, 20, 30, 40)], "the menu reached the screen");
}

#[test]
fn rectangles_after_the_refused_one_wait_too() {
    let mut d = Display { damage: DamageAccumulator::new(), refuse: 1, shown: Vec::new() };
    d.damage.accumulate(Rect { x: 0, y: 0, width: 4, height: 4 });
    d.damage.accumulate(Rect { x: 40, y: 40, width: 4, height: 4 });
    assert!(drain_damage(&mut d, |d| &mut d.damage, show).is_err());
    assert!(drain_damage(&mut d, |d| &mut d.damage, show).is_ok());
    d.shown.sort_unstable();
    assert_eq!(d.shown, vec![(0, 0, 4, 4), (40, 40, 4, 4)]);
}
