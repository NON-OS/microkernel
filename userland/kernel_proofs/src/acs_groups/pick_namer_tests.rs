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

//! A false namer on the chain (L77d): a bus named by a bridge that ends a
//! link and by one that does not is borne out by neither, so a fully swapped
//! switch nested below it still counts its ports. Each namer of a bus is
//! asked, all of them, never the first that agrees.

use super::pick_gen::ROOT;
use super::port_rule::{picked, Bridge, DOWNSTREAM, UPSTREAM};

#[test]
fn a_false_namer_on_the_chain_hides_no_nested_switch_port() {
    // A root port, a switch, and below its second port a switch whose
    // upstream port says downstream and whose first port says upstream.
    let tree = |u: u8, d: u8, namer: Bridge| {
        let mut t = vec![(0, Some(1), Some(ROOT)), (1, Some(2), Some(u))];
        t.extend([(2, Some(3), Some(DOWNSTREAM)), (2, Some(4), Some(d))]);
        t.extend([(4, Some(5), Some(DOWNSTREAM)), (5, Some(6), Some(UPSTREAM))]);
        t.extend([(5, Some(7), Some(DOWNSTREAM)), namer]);
        t
    };
    let ended = tree(ROOT, UPSTREAM, (0, Some(2), Some(ROOT)));
    let up = tree(UPSTREAM, DOWNSTREAM, (1, Some(4), Some(UPSTREAM)));
    for (what, t) in [("a false root port", ended), ("a false upstream port", up)] {
        let pick = picked(&t);
        assert!(pick(5) && pick(6), "{what} hides a nested switch port: {t:?}");
    }
}

#[test]
fn every_namer_of_a_bus_is_asked_not_the_first_that_agrees() {
    // A root port, a link to a switch with one port, a link below that; then
    // a false namer of the root port's bus, or of the switch's own bus.
    let chain = vec![(0, Some(1), Some(ROOT)), (1, Some(2), Some(UPSTREAM))];
    let chain =
        [chain, vec![(2, Some(3), Some(DOWNSTREAM)), (3, Some(4), Some(UPSTREAM))]].concat();
    assert!(!picked(&chain)(3), "honest: the nested upstream port is borne out");
    for namer in [(0, Some(1), Some(7)), (0, Some(2), Some(UPSTREAM))] {
        let t = [chain.clone(), vec![namer]].concat();
        assert!(picked(&t)(3), "a false namer {namer:?} bore out what it names");
    }
}

#[test]
fn a_bridge_saying_root_ends_a_link_only_where_nothing_names_its_bus() {
    // Below a root port, a lone bridge saying root and a lone one below it.
    let t = [(0, Some(1), Some(ROOT)), (1, Some(2), Some(ROOT)), (2, Some(3), Some(UPSTREAM))];
    let pick = picked(&t);
    assert!(pick(1) && pick(2), "a root port below a bridge bears nothing out");
}
