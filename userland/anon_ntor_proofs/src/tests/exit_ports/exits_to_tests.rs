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

//! `exits_to` at the edges, and the web ports kept to the rule they had.

use crate::directory::microdesc::policy::{exits_to, exits_web};
use crate::directory::microdesc::summary::summary;
use crate::path::Ports;

use crate::tests::path_pool::relay;

#[test]
fn the_ends_of_the_port_space_and_of_each_range() {
    let accept = summary(b"accept 1,18080-18089,65535").unwrap();
    for (port, out) in [(1, true), (2, false), (18079, false), (18080, true), (18081, true)] {
        assert_eq!(exits_to(&accept, port), out, "accept {port}");
    }
    for (port, out) in [(18089, true), (18090, false), (65534, false), (65535, true)] {
        assert_eq!(exits_to(&accept, port), out, "accept {port}");
    }
    let reject = summary(b"reject 1,18080-18089,65535").unwrap();
    for port in [1u16, 2, 18079, 18080, 18089, 18090, 65534, 65535] {
        assert_eq!(exits_to(&reject, port), !exits_to(&accept, port), "reject {port}");
    }
}

#[test]
fn port_zero_is_never_let_out() {
    assert!(!exits_to(&summary(b"reject 1").unwrap(), 0));
    assert!(!exits_to(&summary(b"accept 1-65535").unwrap(), 0));
    assert!(!exits_to(&Default::default(), 0));
}

#[test]
fn the_web_ports_are_read_by_exits_web_and_nothing_else() {
    let lines: [&[u8]; 6] =
        [b"accept 80", b"accept 80,443", b"reject 443", b"accept 79-444", b"reject 1-65535", b"x"];
    for line in lines {
        let mut exit = relay([10, 1, 0, 1], 1);
        exit.exits_web = exits_web(line);
        let ports = Ports::new(alloc::vec![([1; 20], summary(line).unwrap_or_default())]);
        for port in [80u16, 443] {
            assert_eq!(ports.lets_out(&exit, port), exits_web(line), "{line:?} {port}");
        }
    }
}

#[test]
fn any_other_port_is_read_from_the_exits_own_summary() {
    let mut exit = relay([10, 1, 0, 1], 1);
    exit.exits_web = true;
    let ports = Ports::new(alloc::vec![
        ([9; 20], summary(b"accept 18081").unwrap()),
        ([1; 20], summary(b"accept 80,443,18089").unwrap()),
    ]);
    assert!(ports.lets_out(&exit, 18089));
    assert!(!ports.lets_out(&exit, 18081), "another relay's summary says nothing of this one");
    let unknown = relay([10, 2, 0, 1], 2);
    assert!(!ports.lets_out(&unknown, 18089), "no summary held lets the port out nowhere");
    assert!(ports.lets_out(&unknown, 443), "the web rule still reads its own flag");
}
