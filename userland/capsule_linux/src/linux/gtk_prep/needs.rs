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

//! Whether a program needs GTK's preparation. One that names GTK or
//! gdk-pixbuf anywhere in its bytes does, linked or loaded by name; so does
//! one whose closure carries gdk-pixbuf's loaders and that links GTK
//! through a library of its own (zathura through girara). A Qt program that
//! names neither draws with Qt alone: GTK's tools never ran for anything
//! it reads, and each of its launches waited on them. Pure.

const GTK: [&[u8]; 3] = [b"libgtk-3.so", b"libgtk-4.so", b"libgdk_pixbuf-2.0.so"];
const QT: [&[u8]; 2] = [b"libQt6Core.so", b"libQt5Core.so"];

/// `pixbuf` asks the run's store for gdk-pixbuf's loaders, only if needed.
/// The program is read once, at each "lib", however long it is.
pub fn needs(program: &[u8], pixbuf: impl FnOnce() -> bool) -> bool {
    let mut qt = false;
    for at in program.windows(3).enumerate().filter(|(_, w)| *w == b"lib").map(|(i, _)| i) {
        let rest = &program[at..];
        if GTK.iter().any(|l| rest.starts_with(l)) {
            return true;
        }
        qt |= QT.iter().any(|l| rest.starts_with(l));
    }
    !qt && pixbuf()
}
