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

//! Every port under the host found with its facts, nested ports with the
//! port above them, and each told internal, external or unknown.

use super::rig::laptop;
use crate::ports::{usb_ports, Panel, Reach};
use crate::rig::Rig;
use crate::tests::platform::run::evaluator;

#[test]
fn every_port_with_its_reach() {
    let mut r = Rig::new(&laptop()).connected();
    let host = crate::consts::at(&r.aml, "\\_SB.PCI0.XHC").unwrap();
    let Rig { aml, probe } = &mut r;
    let ports = usb_ports(aml, &mut evaluator(probe), host);
    let seen: Vec<(u8, u8, Reach)> = ports.iter().map(|p| (p.parent, p.port, p.reach())).collect();
    use Reach::{External as E, Internal as I, Unknown as U};
    assert_eq!(
        seen,
        [(0, 1, E), (0, 2, I), (0, 3, I), (0, 4, I), (0, 5, E), (5, 1, U), (0, 6, U), (0, 7, U)],
        "socket, Bluetooth, unconnectable, lid camera, Type-C, the hub's port, short buffers"
    );
    assert_eq!(ports[0].pld.map(|p| (p.panel, p.user_visible)), Some((Panel::Left, true)));
    assert_eq!(
        ports[3].pld.map(|p| (p.lid, p.user_visible, p.panel)),
        Some((true, true, Panel::Front))
    );
    assert_eq!(ports[4].upc.map(|u| u.connector), Some(0x09), "Type-C");
}
