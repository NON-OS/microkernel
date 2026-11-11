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

use alloc::{string::String, string::ToString, vec::Vec};

use super::layout::{metrics, px};
use super::sidebar_model::{Rows, SideHit, SideKind, SideRow, CARD_FOOT, CARD_H};
use super::sidebar_model::{DOWNLOADS, NAV, PLACES, SEC_GAP};

/// The one layout pass: `paint_sidebar` draws these rows and `side_at` tests
/// against them, so a click can never land where nothing was drawn. Favorites is
/// variable-length, so every row below it is placed by the running y, not by index.
pub fn side_rows(favorites: &[&str], win_h: u32) -> Vec<SideRow> {
    let mut b = Rows { rows: Vec::new(), y: metrics().side_first_y };
    for (text, screen) in NAV {
        b.row(text.to_string(), SideHit::Screen(screen));
    }
    b.rule();
    if !favorites.is_empty() {
        b.label("FAVORITES");
        for path in favorites {
            b.row(base_label(path), SideHit::Path(path.to_string()));
        }
        b.y += px(SEC_GAP);
    }
    b.label("PLACES");
    b.row("Downloads".to_string(), SideHit::Path(DOWNLOADS.to_string()));
    for (text, path) in PLACES {
        b.row(text.to_string(), SideHit::Path(path.to_string()));
    }
    storage_slot(win_h, &mut b);
    b.rows
}

// The drive card is pinned to the foot of the rail rather than stacked after the
// groups, so it is placed off the window height and withheld entirely when the
// groups above it would overlap it. Its click browses the store root, which is
// the destination PLACES already names, so no new one is invented for it.
fn storage_slot(win_h: u32, b: &mut Rows) {
    let (card, foot) = (px(CARD_H), px(CARD_FOOT));
    let y = win_h.saturating_sub(card + foot);
    if y < b.y + foot {
        return;
    }
    let hit = SideHit::Path(PLACES[0].1.to_string());
    b.pinned(y, card, SideKind::Storage, Some(hit));
}

/// Which row `y` lands on, by its index in `rows`; `None` on a section label,
/// a rule, or a gap.
pub fn side_at(rows: &[SideRow], y: u32) -> Option<usize> {
    rows.iter().position(|r| r.hit.is_some() && y >= r.y && y < r.y + r.h)
}

/// A favorite shows its last path segment; a path with no segment keeps itself.
pub fn base_label(path: &str) -> String {
    match path.trim_end_matches('/').rsplit('/').next() {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => path.to_string(),
    }
}
