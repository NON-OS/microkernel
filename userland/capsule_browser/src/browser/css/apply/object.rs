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

use alloc::vec::Vec;

use crate::browser::css::computed::{Computed, ObjectFit};

/* object-fit and object-position, each keeping what the other set. */
pub(super) fn apply_object(c: &mut Computed, name: &str, value: &str, fs: u32) {
    match name {
        "object-fit" => {
            let at = c.object_fit.pos();
            c.object_fit = match value.trim() {
                "cover" => ObjectFit::Cover(at),
                "fill" => ObjectFit::Fill(at),
                "contain" | "scale-down" | "none" => ObjectFit::Contain(at),
                _ => c.object_fit,
            }
        }
        "object-position" => {
            let ws: Vec<&str> = value.split_whitespace().collect();
            if let Some(p) = crate::browser::css::bg_url::bg_pos(&ws, fs) {
                c.object_fit = c.object_fit.at(p);
            }
        }
        _ => {}
    }
}
