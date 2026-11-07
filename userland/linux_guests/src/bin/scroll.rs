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

//! A Linux program that scrolls: a window whose content moves up eight
//! rows a frame, drawn into whichever of its two buffers its compositor
//! has given back and committed with the rows that moved as damage, a frame
//! at each frame callback. It says how long its frames take, so a live run
//! measures a Linux window kept moving as an app's scroll keeps it.

fn main() {
    if !nonos_linux_guests::wl::scroll::run() {
        std::process::exit(1);
    }
}
