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

//! The model's exits, with and without the register rule.

use super::model::World;

#[test]
fn without_the_rule_a_frame_reaches_the_allocator_twice() {
    for device in [false, true] {
        let mut w = World::default();
        w.register(false, 1, 0, 2, device);
        w.register(false, 1, 0, 2, false);
        w.slots.iter_mut().for_each(|s| s.2 = true);
        w.exit(1);
        w.compositor_lets_go();
        assert!(w.freed.values().any(|&n| n == 2), "device {device}");
    }
}

#[test]
fn with_the_rule_every_frame_is_freed_exactly_once() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..2000 {
        let mut w = World::default();
        for _ in 0..12 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let (p, at, n) = (1 + (seed >> 60) % 3, (seed >> 40) % 6, 1 + (seed >> 30) % 3);
            w.register(true, p, at, n, (seed >> 20) & 3 == 0);
            if (seed >> 10) & 1 == 1 {
                w.slots.iter_mut().for_each(|s| s.2 = true);
            }
        }
        (1..4).for_each(|p| w.exit(p));
        w.compositor_lets_go();
        assert!(w.freed.values().all(|&n| n == 1));
        assert_eq!(w.freed.len(), 3 * 16, "every page and DMA buffer freed");
    }
}
