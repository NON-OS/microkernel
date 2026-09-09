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

extern crate alloc;

use alloc::{string::String, vec::Vec};

use nonos_app_skeleton::clients::vfs::dirstat;
use nonos_app_skeleton::PaintBuffer;

use super::card::{card, Card};
use super::human_size::human_size;
use super::layout::CARD_H;
use super::paint_sidebar::PLACES;
use super::screen_row::section_label;
use super::state::State;
use super::theme::CY_DIM;

/// One card per quick-access place, each populated by a `dirstat` aggregate.
/// The store reports no device total, so the capacity bar is scaled against the
/// largest place rather than against a capacity nobody can name.
pub fn storage_cards(state: &State, fb: &mut PaintBuffer, x: u32, top: u32, w: u32, bottom: u32) {
    let mut y = top + section_label(fb, x, top, "STORAGE & DEVICES");
    let stats: Vec<Option<(u32, u32, u64, bool)>> =
        PLACES.iter().map(|(_, path)| dirstat(state.owner_pid, path.as_bytes()).ok()).collect();
    let peak = stats.iter().flatten().map(|(_, _, bytes, _)| *bytes).max().unwrap_or(0).max(1);
    for ((label, _), stat) in PLACES.iter().zip(stats.iter()) {
        if y + CARD_H > bottom {
            break;
        }
        let sub = match stat {
            Some((files, dirs, bytes, truncated)) => summary(*files, *dirs, *bytes, *truncated),
            None => String::from("-"),
        };
        let spec = Card {
            title: label,
            sub: sub.as_str(),
            meta: "",
            tint: CY_DIM,
            dir: true,
            bar: stat.map(|(_, _, bytes, _)| (bytes * 100 / peak) as u32),
        };
        y += card(fb, x, y, w, &spec);
    }
}

// A truncated aggregate stopped short of the whole subtree, so its size is a
// floor and is marked as one rather than presented as exact.
fn summary(files: u32, dirs: u32, bytes: u64, truncated: bool) -> String {
    let size = human_size(bytes);
    let mark = if truncated { "~" } else { "" };
    alloc::format!("{files} files · {dirs} folders · {mark}{size}")
}
