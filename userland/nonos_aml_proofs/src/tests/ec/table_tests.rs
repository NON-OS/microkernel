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

//! Finding the EC: its ports from _CRS or the ECDT, and its _Qxx methods.

use crate::consts::at;
use crate::ec::{ec_ports, parse_ecdt, query_methods, EcPorts};
use crate::enc::*;
use crate::run::boot;

#[test]
fn ports_from_io_and_fixed_io_descriptors() {
    let io = |p: u16| [0x47, 0x01, p as u8, (p >> 8) as u8, p as u8, (p >> 8) as u8, 0, 1];
    let crs = [&io(0x62)[..], &io(0x66), &[0x79, 0]].concat();
    assert_eq!(ec_ports(&crs), Some(EcPorts { data: 0x62, command: 0x66 }));
    let fixed = [0x4B, 0x62, 0x00, 1, 0x4B, 0x66, 0x00, 1, 0x79, 0];
    assert_eq!(ec_ports(&fixed), Some(EcPorts { data: 0x62, command: 0x66 }));
    assert_eq!(ec_ports(&[&io(0x62)[..], &[0x79, 0]].concat()), None, "one port only");
}

#[test]
fn the_ecdt_gives_ports_gpe_and_path() {
    let mut t = vec![0u8; 65];
    t[..4].copy_from_slice(b"ECDT");
    let gas = |port: u64| [&[1u8, 8, 0, 1][..], &port.to_le_bytes()].concat();
    t[36..48].copy_from_slice(&gas(0x66));
    t[48..60].copy_from_slice(&gas(0x62));
    t[64] = 0x17;
    t.extend_from_slice(b"\\_SB.PCI0.LPCB.EC0\0");
    let e = parse_ecdt(&t).unwrap();
    assert_eq!((e.ports, e.gpe), (EcPorts { data: 0x62, command: 0x66 }, 0x17));
    assert_eq!(e.path, b"\\_SB.PCI0.LPCB.EC0");
    t[36] = 0;
    assert_eq!(parse_ecdt(&t), None, "a control port in memory");
}

#[test]
fn query_methods_by_their_hex_numbers() {
    let nop = [0xA3];
    let ec = cat(&[
        &method("_Q0A", 0, &nop),
        &method("_Q1F", 0, &nop),
        &method("_QXY", 0, &nop),
        &name_obj("_Q20", &int(1)),
    ]);
    let (aml, _) = boot(&device("\\_SB.EC0", &ec), 2);
    let q = query_methods(&aml.ns, at(&aml, "\\_SB.EC0").unwrap());
    assert_eq!(q.keys().copied().collect::<Vec<_>>(), [0x0A, 0x1F]);
    assert_eq!(q[&0x0A], at(&aml, "\\_SB.EC0._Q0A").unwrap());
}
