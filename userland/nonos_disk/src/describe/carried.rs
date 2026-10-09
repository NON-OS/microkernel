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

//! The row that says what the store carries over.

use alloc::format;
use alloc::string::String;

use nonos_policy_proto::keyboard_layout_labels::KEYBOARD_LAYOUT_LABELS;

use crate::carry::Carried;

pub fn carried_text(c: &Carried) -> String {
    let answers = match &c.answers {
        Some(a) => {
            let label = KEYBOARD_LAYOUT_LABELS.get(a.keyboard_layout as usize);
            let keyboard = label.and_then(|l| core::str::from_utf8(l).ok()).unwrap_or("keyboard");
            format!("setup answers ({keyboard}, UTC{:+})", a.timezone)
        }
        None => String::from("no setup answers on this boot"),
    };
    let plural = if c.programs == 1 { "" } else { "s" };
    let data = match c.data {
        0 => String::new(),
        1 => String::from(", 1 file they read"),
        n => format!(", {n} files they read"),
    };
    let walls = match c.wallpapers.carried {
        0 => String::new(),
        1 => String::from(", 1 wallpaper"),
        n => format!(", {n} wallpapers"),
    };
    format!("{answers}, {} signed program{plural}{data}{walls}", c.programs)
}
