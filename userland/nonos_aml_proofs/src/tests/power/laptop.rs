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

//! A laptop's tables as firmware writes them (ACPI 6.5, 10.2, 10.3, 9.4,
//! 12.11): an EC (PNP0C09) whose EmbeddedControl region holds the battery
//! and adapter registers; a battery (PNP0C0A) whose _BST and _BIX fill
//! named packages from those fields by Index; an adapter (ACPI0003) whose
//! _PSR and a lid (PNP0C0D) whose _LID each read one bit.

use crate::enc::*;

/// Store(field, Index(pkg, i)).
fn put(pkg: &str, i: u64, value: &[u8]) -> Vec<u8> {
    store(value, &e(0x88, &[&name(pkg), &int(i), &[0]]))
}

pub fn tables() -> Vec<u8> {
    let units = [skip(0x40 * 8), unit("B0ST", 8), unit("ADPS", 1), unit("LIDS", 1), skip(6)];
    let words = ["B0RR", "B0RC", "B0FC", "B0DC", "B0DV"].map(|n| unit(n, 16));
    let mut bix: Vec<Vec<u8>> = (0..16).map(|_| int(0)).collect();
    bix.extend(["BAT-57", "0001", "LION", "NONOS"].map(string));
    let (b, x) = ("PBST", "PBIX");
    let bst = [put(b, 0, &name("B0ST")), put(b, 1, &name("B0RR")), put(b, 2, &name("B0RC"))];
    let bst = cat(&[&bst.concat(), &put(b, 3, &name("B0DV")), &ret(&name(b))]);
    let fill = [put(x, 1, &int(1)), put(x, 2, &name("B0DC")), put(x, 3, &name("B0FC"))];
    let bix_m = cat(&[&fill.concat(), &put(x, 5, &name("B0DV")), &ret(&name(x))]);
    let ec = cat(&[
        &name_obj("_HID", &int(0x090C_D041)),
        &region("ERAM", 3, 0, 0xFF),
        &field("ERAM", 0x11, &[&units[..], &words[..]].concat()),
    ]);
    let bat = cat(&[
        &name_obj("_HID", &int(0x0A0C_D041)),
        &method("_STA", 0, &ret(&int(0x1F))),
        &name_obj(b, &package(4, &[int(0), int(0), int(0), int(0)])),
        &name_obj(x, &package(20, &bix)),
        &method("_BST", 0, &bst),
        &method("_BIX", 0, &bix_m),
    ]);
    let ac =
        cat(&[&name_obj("_HID", &string("ACPI0003")), &method("_PSR", 0, &ret(&name("ADPS")))]);
    let lid = cat(&[
        &name_obj("_HID", &int(0x0D0C_D041)),
        &method("_LID", 0, &ret(&name("\\_SB.LPCB.EC0.LIDS"))),
    ]);
    let ec = cat(&[&ec, &device("BAT0", &bat), &device("ADP1", &ac)]);
    cat(&[&device("\\_SB.LPCB", &device("EC0", &ec)), &device("\\_SB.LID0", &lid)])
}
