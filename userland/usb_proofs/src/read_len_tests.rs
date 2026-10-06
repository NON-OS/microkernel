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

//! The interrupt read for a bound endpoint asks for one packet, never more
//! than the controller driver will take. That driver answers a longer read
//! with E_INVAL, and an endpoint read that way never delivers a report.

use crate::poll::read_len::read_len;
use crate::xhci_limits::HID_REPORT_MAX as XHCI_READ_MAX;

#[test]
fn a_read_asks_for_one_packet_and_never_past_what_the_controller_driver_takes() {
    let most = XHCI_READ_MAX as u16;
    for max_packet in 0..=u16::MAX {
        let len = read_len(max_packet);
        assert!(len <= most, "{max_packet} byte packets read as {len}");
        assert_eq!(len, max_packet.min(most), "{max_packet} byte packets");
    }
}

#[test]
fn every_packet_size_a_binding_allows_is_read_in_full_up_to_a_boot_report() {
    // Bindings take 3 to 1024 byte packets; QEMU's devices use 4 and 8.
    assert_eq!(read_len(3), 3);
    assert_eq!(read_len(4), 4);
    assert_eq!(read_len(8), 8);
    assert_eq!(read_len(64), 8);
    assert_eq!(read_len(1024), 8);
}
