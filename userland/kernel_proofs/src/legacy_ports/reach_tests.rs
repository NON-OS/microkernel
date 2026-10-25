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

use super::support::{fixed, meets, p};

#[test]
fn the_fixed_windows_are_disjoint() {
    let all = fixed();
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert!(!meets(*a, *b), "{a:?} {b:?}");
        }
    }
}

#[test]
fn no_fixed_window_reaches_a_port_the_kernel_or_another_device_owns() {
    let kept = [
        (p(0x00, 0x20), "8237 DMA controller 1"),
        (p(0x20, 2), "master PIC"),
        (p(0x40, 4), "PIT"),
        (p(0x60, 5), "PS/2 controller"),
        (p(0x70, 2), "CMOS and RTC"),
        (p(0xA0, 2), "slave PIC"),
        (p(0xC0, 0x20), "8237 DMA controller 2"),
        (p(0x3F0, 6), "floppy controller"),
        (p(0x3F7, 1), "floppy digital input"),
        (p(0x377, 1), "second floppy digital input"),
        (p(0x3F8, 8), "COM1"),
        (p(0x2F8, 8), "COM2"),
        (p(0xCF8, 8), "PCI configuration"),
    ];
    for w in fixed() {
        for (k, name) in kept {
            assert!(!meets(w, k), "{w:?} reaches the {name}");
        }
    }
}
