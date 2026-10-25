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

use super::compat::Ports;
use super::dispi::{dispi, DISPI, VBOX_VENDOR, VBOX_VGA};

#[test]
fn only_vboxvga_without_an_io_bar_gets_the_dispi_ports() {
    assert_eq!(dispi(VBOX_VENDOR, VBOX_VGA, false), Some(DISPI));
    assert_eq!(DISPI, Ports { base: 0x1CE, count: 3 });
    // VBoxSVGA: the same id with its registers in an I/O BAR.
    assert_eq!(dispi(VBOX_VENDOR, VBOX_VGA, true), None);
    assert_eq!(dispi(VBOX_VENDOR, 0xCAFE, false), None);
    assert_eq!(dispi(0x1234, 0x1111, false), None);
    assert_eq!(dispi(0x15AD, 0x0405, false), None);
}
