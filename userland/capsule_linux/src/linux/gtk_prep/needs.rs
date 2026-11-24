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

//! Whether a program wants GTK's caches made before it starts. One that
//! links GTK does, and so does one that links it through a library of its
//! own (Mousepad through libmousepad), known by gdk-pixbuf's loaders in its
//! run's store. A Qt program does not: Qt loads its GTK theme only when the
//! desktop or QT_QPA_PLATFORMTHEME names one, and env.rs names neither. So
//! a Qt program whose packages bring GTK along (KeePassXC's do) would have
//! the caches made at every launch and never read.

const GTK: [&[u8]; 3] = [b"libgtk-3.so", b"libgtk-4.so", b"libgdk_pixbuf-2.0.so"];
const QT: [&[u8]; 2] = [b"libQt5Core.so", b"libQt6Core.so"];

fn names(program: &[u8], libs: &[&[u8]]) -> bool {
    libs.iter().any(|l| program.windows(l.len()).any(|w| w == *l))
}

/// `pixbuf` when the run's store has gdk-pixbuf's loaders.
pub fn wanted(program: &[u8], pixbuf: bool) -> bool {
    names(program, &GTK) || (pixbuf && !names(program, &QT))
}
