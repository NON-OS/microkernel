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

use super::compat::Ports;
use super::support::{fixed, meets, p};

#[test]
fn overlap_is_half_open_and_never_wraps() {
    assert!(meets(p(0x1F0, 8), p(0x1F7, 1)));
    assert!(!meets(p(0x1F0, 8), p(0x1F8, 1)));
    assert!(!meets(p(0x1EF, 1), p(0x1F0, 8)));
    assert!(meets(p(0x1EF, 2), p(0x1F0, 8)));
    assert!(meets(p(0xFFFF, 1), p(0xFFFE, 2)));
    assert!(!meets(p(0xFFFF, 1), p(0x0000, 1)));
    assert!(!meets(p(0x1F0, 0), p(0x1F0, 8)));
    assert!(!meets(p(0x1F0, 8), p(0x1F0, 0)));
}

#[test]
fn overlap_is_symmetric_and_matches_a_port_by_port_walk() {
    let windows = [p(0, 1), p(0x1EE, 3), p(0x1F0, 8), p(0x3F4, 4), p(0xFFF0, 16), p(0x100, 0)];
    for a in windows.iter().chain(fixed().iter()) {
        for b in windows.iter().chain(fixed().iter()) {
            let ports = |w: &Ports| w.base as u32..w.base as u32 + w.count as u32;
            let walk = ports(a).any(|x| ports(b).contains(&x));
            assert_eq!(meets(*a, *b), walk, "{a:?} {b:?}");
            assert_eq!(meets(*a, *b), meets(*b, *a));
        }
    }
}
