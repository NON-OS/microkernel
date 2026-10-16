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

//! Root ports across several controllers, numbered one after another.

#[test]
fn ports_of_several_controllers_are_numbered_one_after_another() {
    use crate::server::mux_ports::{local_port, port_ranges};
    // An Intel PCH with 16 ports beside a Thunderbolt controller with 4.
    let r = port_ranges(&[16, 4]);
    assert_eq!(r, vec![(0, 16), (16, 4)]);
    assert_eq!(local_port(&r, 1), Some((0, 1)));
    assert_eq!(local_port(&r, 16), Some((0, 16)));
    assert_eq!(local_port(&r, 17), Some((1, 1)));
    assert_eq!(local_port(&r, 20), Some((1, 4)));
    assert_eq!(local_port(&r, 21), None);
    assert_eq!(local_port(&r, 0), None, "port 0 is no port");
    // A single controller keeps its own numbers.
    let one = port_ranges(&[26]);
    assert!((1..=26).all(|p| local_port(&one, p) == Some((0, p))));
}

#[test]
fn global_port_numbers_stop_at_255() {
    use crate::server::mux_ports::{local_port, port_ranges};
    let r = port_ranges(&[200, 100, 10]);
    assert_eq!(r, vec![(0, 200), (200, 55), (255, 0)]);
    assert_eq!(local_port(&r, 255), Some((1, 55)));
    for p in 0..=255u8 {
        if let Some((c, l)) = local_port(&r, p) {
            assert!(l >= 1 && l <= r[c].1, "port {p} maps outside controller {c}");
        }
    }
}
