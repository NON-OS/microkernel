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

//! The pick over whole bridge lists (L77): a bridge firmware left unset, a
//! switch whose ports lie, and a false bridge naming a switch's bus.

use super::pick_gen::ROOT;
use super::port_rule::{picked, Bridge, DOWNSTREAM, UPSTREAM};

fn counted(b: &[Bridge]) -> Vec<usize> {
    let pick = picked(b);
    (0..b.len()).filter(|&at| pick(at)).collect()
}

#[test]
fn a_bridge_left_unset_puts_no_root_port_in_a_switch() {
    let roots = [(0, Some(1), Some(ROOT)), (0, Some(2), Some(ROOT)), (0, Some(3), Some(ROOT))];
    let unset = (1, Some(0), Some(UPSTREAM));
    assert!(counted(&[roots[0], roots[1], roots[2], unset]).is_empty());
    let untyped = (0, Some(4), None);
    assert!(counted(&[roots[0], (2, Some(0), Some(ROOT)), untyped]).is_empty(), "no type");
    assert!(counted(&[roots[0], (1, Some(1), Some(UPSTREAM))]).is_empty(), "names its own bus");
    assert!(!picked(&[])(0) && !picked(&[(0, Some(1), Some(DOWNSTREAM))])(1), "past the list");
}

#[test]
fn a_switch_counts_its_ports_whatever_they_say_and_a_false_namer_hides_none() {
    let tree = [
        (0, Some(1), Some(ROOT)),
        (1, Some(2), Some(UPSTREAM)),
        (2, Some(3), Some(DOWNSTREAM)),
        (2, Some(4), Some(UPSTREAM)),
        (2, Some(5), None),
        (2, Some(6), Some(ROOT)),
    ];
    assert_eq!(counted(&tree), vec![2, 3, 4, 5]);
    let mut lied = tree.to_vec();
    lied.insert(0, (1, Some(2), Some(ROOT)));
    lied.push((9, Some(2), Some(ROOT)));
    assert_eq!(counted(&lied), vec![0, 2, 3, 4, 5, 6], "a second bridge naming bus 2");
}

#[test]
fn a_swapped_switch_on_a_root_bus_counts_its_ports() {
    // No root port above: the upstream port says root, a port upstream.
    for d in [Some(UPSTREAM), Some(DOWNSTREAM), None, Some(ROOT)] {
        let lied = [(0, Some(1), Some(ROOT)), (1, Some(2), Some(UPSTREAM)), (1, Some(3), d)];
        assert_eq!(counted(&lied), vec![1, 2], "a link carries one component: {d:?}");
    }
}
