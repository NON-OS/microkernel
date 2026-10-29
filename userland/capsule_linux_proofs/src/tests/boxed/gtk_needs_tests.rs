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

//! Which programs wait on GTK's preparation at launch: one that names GTK
//! or gdk-pixbuf, linked or loaded by name; one that links GTK through a
//! library of its own where its closure has gdk-pixbuf's loaders; never a
//! Qt program that names neither (gtk_prep/needs.rs).

#[path = "../../../../capsule_linux/src/linux/gtk_prep/needs.rs"]
mod needs;

use needs::needs;

fn elf(names: &[&str]) -> Vec<u8> {
    let mut out = b"\x7fELF\0\0padding libc.so.6\0".to_vec();
    names.iter().for_each(|n| out.extend_from_slice(&[n.as_bytes(), b"\0x"].concat()));
    out
}

#[test]
fn a_program_naming_gtk_needs_it_whatever_its_closure() {
    let never = || panic!("the store is not asked when the program names GTK");
    for gtk in ["libgtk-3.so.0", "libgtk-4.so.1", "libgdk_pixbuf-2.0.so.0"] {
        assert!(needs(&elf(&[gtk]), never), "{gtk}");
        assert!(needs(&elf(&["libQt6Core.so.6", gtk]), never), "Qt that loads {gtk} by name");
    }
}

#[test]
fn a_qt_program_naming_no_gtk_needs_none_of_it() {
    for qt in ["libQt6Core.so.6", "libQt5Core.so.5"] {
        assert!(!needs(&elf(&[qt, "libQt6Widgets.so.6"]), || true), "{qt}");
    }
}

#[test]
fn a_program_linking_gtk_through_its_own_library_needs_it_where_gdk_pixbuf_is() {
    let zathura = elf(&["libgirara-gtk3.so.4", "libglib-2.0.so.0"]);
    assert!(needs(&zathura, || true));
    assert!(!needs(&zathura, || false), "no loaders in its closure: no GTK");
    assert!(!needs(b"lib", || false) && needs(b"", || true), "short programs");
}
