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

use super::acs_span::{group_of, groups, Port};

fn port(bus: u8, first: u8, last: u8, closed: bool) -> Port {
    Port { bus, below: Some((first, last)), closed }
}

#[test]
fn the_outermost_open_switch_names_the_group() {
    // A switch on bus 2 (ports to 3 and 4..=6); one below it on bus 5.
    let inner_open = [port(2, 3, 3, true), port(2, 4, 6, true), port(5, 6, 6, false)];
    assert_eq!(groups(&inner_open), vec![(6, 6)]);
    let outer_open = [port(2, 3, 3, false), port(2, 4, 6, true), port(5, 6, 6, false)];
    assert_eq!(groups(&outer_open), vec![(3, 6)], "the inner one is inside it");
    assert_eq!(group_of(&groups(&outer_open), 6), Some(3));
    assert_eq!(group_of(&groups(&outer_open), 2), None, "the switch's own bus");
    let shut = [port(2, 3, 3, true), port(2, 4, 6, true), port(5, 6, 6, true)];
    assert!(groups(&shut).is_empty(), "every port closed: no group");
}

#[test]
fn switches_apart_stay_apart_and_spans_sharing_a_bus_merge() {
    let apart = [port(2, 3, 4, false), port(6, 7, 8, false)];
    assert_eq!(groups(&apart), vec![(3, 4), (7, 8)]);
    assert_ne!(group_of(&groups(&apart), 4), group_of(&groups(&apart), 7));
    let touching = [port(2, 3, 7, false), port(6, 7, 9, false)];
    assert_eq!(groups(&touching), vec![(3, 9)], "bus 7 in both");
}

#[test]
fn a_range_that_cannot_be_read_reaches_every_later_bus() {
    let unread = Port { bus: 2, below: None, closed: true };
    assert_eq!(groups(&[unread, port(2, 3, 3, false)]), vec![(3, 0xFF)]);
    let backwards = port(4, 4, 9, false);
    assert_eq!(groups(&[backwards]), vec![(5, 0xFF)], "a range not below its own bus");
    let last = Port { bus: 0xFF, below: None, closed: false };
    assert!(groups(&[last]).is_empty(), "no bus after 255");
}
