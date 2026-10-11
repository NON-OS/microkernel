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

//! Broadcom's parts named as the tg3 capsule takes them: its 57765 class by
//! table, and the other tg3 families, which no capsule drives, as no
//! driver's part.

use super::broadcom_ids::TG3_DEVICE_IDS;
use super::classify_network::classify_network;
use super::family::HardwareFamily::*;

#[test]
fn broadcom_by_table() {
    for id in TG3_DEVICE_IDS {
        assert_eq!(classify_network(0x00, 0x14e4, id), NetworkTg3, "{id:04x}");
    }
    // The 5761, the 5717 and the 5762 are tg3 families no capsule drives.
    for id in [0x1681, 0x1655, 0x1687] {
        assert_eq!(classify_network(0x00, 0x14e4, id), Unknown, "{id:04x}");
    }
    assert_eq!(classify_network(0x00, 0x8086, 0x16B4), Unknown, "Intel's id space");
}
