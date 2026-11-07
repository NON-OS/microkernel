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

//! A laptop's tables in the shape HP and most others write them: the EC's
//! _REG sets ECOK when the EmbeddedControl space is connected, and the
//! battery's _STA says "battery present" only once ECOK is set and the
//! EC's present bit (0x41 bit 2) is. Until then the battery reads as a slot
//! with nothing in it (0x0F), which is what firmware means to say before
//! the EC can be asked. _REG also keeps the AC bit in PWRS, as many do, so
//! the EC is read from the moment its space is connected.

use crate::enc::*;

/// Store(field, Index(pkg, i)).
fn put(pkg: &str, i: u64, value: &[u8]) -> Vec<u8> {
    store(value, &e(0x88, &[&name(pkg), &int(i), &[0]]))
}

/// LEqual(a, b).
fn equal(a: &[u8], b: &[u8]) -> Vec<u8> {
    e(0x93, &[a, b])
}

/// With `battery` false, the same machine with no battery device at all: a
/// desktop's EC.
pub fn with_battery(battery: bool) -> Vec<u8> {
    let bits = [unit("ADPS", 1), unit("LIDS", 1), unit("B0PR", 1), skip(5)];
    let units = [&[skip(0x40 * 8), unit("B0ST", 8)][..], &bits[..]].concat();
    let words = ["B0RR", "B0RC", "B0FC", "B0DC", "B0DV"].map(|n| unit(n, 16));
    let mut bix: Vec<Vec<u8>> = (0..16).map(|_| int(0)).collect();
    bix.extend(["BAT-57", "0001", "LION", "NONOS"].map(string));
    let (b, x) = ("PBST", "PBIX");
    let bst = [put(b, 0, &name("B0ST")), put(b, 1, &name("B0RR")), put(b, 2, &name("B0RC"))];
    let bst = cat(&[&bst.concat(), &put(b, 3, &name("B0DV")), &ret(&name(b))]);
    let fill = [put(x, 1, &int(1)), put(x, 2, &name("B0DC")), put(x, 3, &name("B0FC"))];
    let bix_m = cat(&[&fill.concat(), &put(x, 5, &name("B0DV")), &ret(&name(x))]);
    let reg = cat(&[&store(&arg(1), &name("ECOK")), &store(&name("ADPS"), &name("PWRS"))]);
    let reg = if_(&equal(&arg(0), &int(3)), &reg);
    let ec = cat(&[
        &name_obj("_HID", &int(0x090C_D041)),
        &name_obj("ECOK", &int(0)),
        &name_obj("PWRS", &int(0)),
        &region("ERAM", 3, 0, 0xFF),
        &field("ERAM", 0x11, &[&units[..], &words[..]].concat()),
        &method("_REG", 2, &reg),
    ]);
    let sta = cat(&[&if_(&name("ECOK"), &if_(&name("B0PR"), &ret(&int(0x1F)))), &ret(&int(0x0F))]);
    let bat = cat(&[
        &name_obj("_HID", &int(0x0A0C_D041)),
        &method("_STA", 0, &sta),
        &name_obj(b, &package(4, &[int(0), int(0), int(0), int(0)])),
        &name_obj(x, &package(20, &bix)),
        &method("_BST", 0, &bst),
        &method("_BIX", 0, &bix_m),
    ]);
    let ac =
        cat(&[&name_obj("_HID", &string("ACPI0003")), &method("_PSR", 0, &ret(&name("ADPS")))]);
    let bat = if battery { device("BAT0", &bat) } else { vec![] };
    let ec = cat(&[&ec, &bat, &device("ADP1", &ac)]);
    device("\\_SB.LPCB", &device("EC0", &ec))
}
