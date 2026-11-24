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

//! Which programs have GTK's caches made before they start: GTK's own, and
//! a library's that brings GTK, but no Qt program whose packages bring GTK
//! along.

use crate::gtk_needs::wanted;

const GTK3: &[u8] = b"\0libgtk-3.so.0\0libc.musl-x86_64.so.1\0";
const QT5: &[u8] = b"\0libQt5Widgets.so.5\0libQt5Core.so.5\0";
const QT6: &[u8] = b"\0libQt6Core.so.6\0";
const OWN: &[u8] = b"\0libmousepad.so.0\0";

#[test]
fn a_program_that_links_gtk_wants_them() {
    assert!(wanted(GTK3, false));
    assert!(wanted(GTK3, true));
}

#[test]
fn gtk_through_a_library_of_its_own_wants_them() {
    assert!(wanted(OWN, true));
    assert!(!wanted(OWN, false));
}

#[test]
fn a_qt_program_with_gtk_in_its_store_does_not() {
    assert!(!wanted(QT5, true));
    assert!(!wanted(QT6, true));
}

#[test]
fn a_qt_program_that_links_gtk_itself_still_does() {
    let both = [QT5, GTK3].concat();
    assert!(wanted(&both, true));
}
