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

use nonos_pinctrl::{Layout, LAYOUTS};

// Pin counts of each Linux soc_data (the last community's end plus one).
const NPINS: &[(&str, u32)] = &[
    ("Sunrise Point-LP", 152),
    ("Sunrise Point-H", 192),
    ("Cannon Point-LP", 244),
    ("Cannon Lake-H", 299),
    ("Ice Lake-LP", 241),
    ("Ice Lake-N", 213),
    ("Jasper Lake", 233),
    ("Tiger Lake-LP", 277),
    ("Tiger Lake-H", 291),
    ("Alder Lake-N", 257),
    ("Alder Lake-S", 304),
    ("Meteor Lake-P", 289),
];

/// Groups tile each community from its first pin, and communities tile the
/// controller from pin zero, as the start and end pins of Linux's tables do.
fn tiled_pins(l: &Layout) -> u32 {
    let mut next = 0u32;
    for c in l.communities {
        assert_eq!(u32::from(c.first), next, "{}: community starts off the tiling", l.name);
        for g in c.groups {
            assert_eq!(u32::from(g.first), next, "{}: group at {} leaves a gap", l.name, g.first);
            assert!(g.size >= 1 && g.size <= 32, "{}: Linux caps a group at 32 pads", l.name);
            next += u32::from(g.size);
        }
    }
    next
}

#[test]
fn every_layout_tiles_its_pins_with_the_linux_count() {
    assert_eq!(LAYOUTS.len(), NPINS.len());
    for (_, l) in LAYOUTS {
        let want = NPINS.iter().find(|(n, _)| *n == l.name).map(|(_, n)| *n);
        assert_eq!(Some(tiled_pins(l)), want, "{}", l.name);
    }
}

#[test]
fn no_two_groups_share_a_firmware_pin_number() {
    for (_, l) in LAYOUTS {
        let mut seen = vec![false; 1024];
        for g in l.communities.iter().flat_map(|c| c.groups) {
            let Some(base) = g.gpio else { continue };
            for n in base..base + g.size {
                assert!(!seen[n as usize], "{}: firmware pin {} mapped twice", l.name, n);
                seen[n as usize] = true;
            }
        }
    }
}
