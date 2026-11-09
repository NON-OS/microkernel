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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Controls, IconButton, Rect as Area, Tabs};

use super::marks::marks;
use crate::browser::ui::fit;
use crate::browser::ui::tabs_geom::{plus_rect, tab_rect, visible, Strip, TabPart};

/// Each tab's label, the icon it leads with and whether its page loads.
pub struct Shown<'a> {
    pub labels: &'a [String],
    pub icons: &'a [IconId],
    pub busy: &'a [bool],
}

/// Lays the tabs in view out in `c`, each one's close mark over it and the
/// plus after the last, then draws them: the strip slides to keep the
/// active tab in view, and a label is cut to its slot so no tab has to
/// shrink below the width it asks for.
pub fn paint(fb: &mut PaintBuffer, s: &Strip, t: &Shown, c: &mut Controls<TabPart>) {
    let (first, count) = visible(s);
    let range = first..first + count;
    let slot = tab_rect(s, first).map_or(0, |r| r.w);
    let names: Vec<String> = range.clone().map(|i| cut(&t.labels[i], t.icons[i], slot)).collect();
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    let area = Area { x: 0, y: 0, w: plus_rect(s).x.max(0) as u32, h: s.h.max(0) as u32 };
    let icons = &t.icons[range.clone()];
    let tabs = Tabs::new(&refs, s.active - first).icons(icons).busy(&t.busy[range]);
    let laid = marks(&tabs, area, first, s.w.max(0) as u32);
    c.set(laid.iter().copied());
    tabs.hovered(hovered(c, first)).draw(fb, area);
    for (at, part) in laid {
        let icon = match part {
            TabPart::Tab(_) => continue,
            TabPart::Close(_) => IconId::Close,
            TabPart::Plus => IconId::Plus,
        };
        IconButton::new(icon).state(c.state(part)).draw(fb, at);
    }
}

fn hovered(c: &Controls<TabPart>, first: usize) -> Option<usize> {
    match c.lit() {
        Some(TabPart::Tab(i) | TabPart::Close(i)) => i.checked_sub(first),
        _ => None,
    }
}

fn cut(label: &str, icon: IconId, slot: i32) -> String {
    let icons = [icon];
    let natural = |t: &str| Tabs::new(&[t], 0).icons(&icons).size().0 as i32;
    fit::truncate_to(label, slot, natural)
}
