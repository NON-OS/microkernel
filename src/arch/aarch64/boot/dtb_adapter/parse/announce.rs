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

use crate::arch::fdt::Fdt;
use crate::sys::serial;

/// One line naming the board, so a log shows which tree the facts came from.
pub(super) fn model(fdt: &Fdt) {
    let model = fdt.find_path(b"/").ok().flatten().and_then(|root| {
        fdt.property(&root, b"model").ok().flatten().and_then(|p| p.string().ok())
    });
    // The model is the tree's to choose, so it is cut to a line of plain
    // ASCII before it reaches the console.
    let mut line = [b'?'; 64];
    let model = model.unwrap_or(b"(none)");
    let len = model.len().min(line.len());
    for (out, &byte) in line.iter_mut().zip(&model[..len]) {
        if byte.is_ascii_graphic() || byte == b' ' {
            *out = byte;
        }
    }
    serial::print(b"[DT] model: ");
    serial::println(&line[..len]);
}
