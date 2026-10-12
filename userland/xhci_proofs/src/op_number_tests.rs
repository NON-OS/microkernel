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

//! One number, one op: every op the controller driver serves and every op a
//! class driver's copy of the wire names agree where they share a number.
//! The isoch ops were first given 0x14, which the USB network drivers
//! already call `OP_BULK_IN_POLL`.

fn ops(src: &str) -> Vec<(String, u16)> {
    src.lines()
        .filter_map(|l| l.trim().strip_prefix("pub const OP_"))
        .filter_map(|r| {
            let (name, rest) = r.split_once(": u16 = ")?;
            let v = rest.trim_end_matches(';').trim_start_matches("0x");
            Some((name.to_string(), u16::from_str_radix(v, 16).ok()?))
        })
        .collect()
}

#[test]
fn no_number_means_two_ops() {
    let served = ops(include_str!("../../capsule_driver_xhci/src/protocol/ops.rs"));
    assert!(served.len() >= 16, "{served:?}");
    let clients = [
        include_str!("../../nonos_usbnet/src/xhci/wire.rs"),
        include_str!("../../capsule_driver_usb_msc/src/xhci/wire.rs"),
        include_str!("../../capsule_driver_usb_hid/src/xhci/wire/constants.rs"),
    ];
    for (name, n) in &served {
        assert_eq!(served.iter().filter(|(_, m)| m == n).count(), 1, "{name} {n:#x}");
        for c in clients {
            for (other, m) in ops(c) {
                assert!(m != *n || other == *name, "{n:#x} is {name} here and {other} there");
            }
        }
    }
}
