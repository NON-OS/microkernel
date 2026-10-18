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

//! An endpoint the configuration gives no polling rate, or a packet size
//! with its reserved bits set, binds nothing (USB 2.0 9.6.6, as recalled,
//! unverified); the transaction bits are read past.

use crate::config_blob::{config, HID, INTERRUPT};
use crate::descriptors::hid_bindings;

#[test]
fn an_endpoint_polled_at_no_interval_is_never_bound() {
    for (sub, proto) in [(1, 1), (1, 2), (0, 0)] {
        let raw = config().iface(0, HID, sub, proto).ep(0x81, INTERRUPT, 64, 0).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "{sub}:{proto}");
    }
}

#[test]
fn reserved_size_bits_bind_nothing_though_transaction_bits_are_read_past() {
    for bit in 13..16 {
        let raw = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 1 << bit | 8, 10).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "bit {bit}");
    }
    let raw = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 0x1808, 10).build();
    assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(1));
}
