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

//! Which PCI functions find counts as displays (L79): those whose base
//! class code is the display class, 0x03, whatever their subclass and
//! programming interface, and no others. An arm placed before the display
//! arm that took some of its codes, or a display arm narrowed to some
//! subclasses, would leave a machine's one display uncounted and turn the
//! fallback off; a wider one would count a second display that is none.

use super::class::{classify_pci, ids};

#[test]
fn every_display_class_code_and_no_other_maps_to_a_display() {
    let mut displays = 0u32;
    for class in 0..=u8::MAX {
        for sub in 0..=u8::MAX {
            for progif in 0..=u8::MAX {
                let display = classify_pci(class, sub, progif).id() == ids::DISPLAY;
                assert_eq!(display, class == 0x03, "{class:#04x}/{sub:#04x}/{progif:#04x}");
                displays += u32::from(display);
            }
        }
    }
    assert_eq!(displays, 256 * 256, "every display code met once");
}
