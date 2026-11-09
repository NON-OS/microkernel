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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Controls, NavItem, Rect as Area};

use super::nav::Look;
use crate::browser::ui::group_rows::dot_slot;
use crate::browser::ui::rect::Rect;
use crate::browser::ui::sidebar_geom::{Row, RowKind};

/// The icon and label a row is drawn with. A group row has neither: it
/// shows the group's colour and name.
fn names(kind: RowKind) -> (Option<IconId>, &'static str) {
    let (icon, label) = match kind {
        RowKind::Start => (IconId::Home, "Start Page"),
        RowKind::Tabs => (IconId::Tabs, "Tabs"),
        RowKind::Bookmarks => (IconId::Star, "Bookmarks"),
        RowKind::Reader => (IconId::Reader, "Reading View"),
        RowKind::Downloads => (IconId::Download, "Downloads"),
        RowKind::History => (IconId::History, "History"),
        RowKind::Settings => (IconId::Sliders, "Settings"),
        RowKind::AllTabs => (IconId::Layers, "All tabs"),
        RowKind::NewGroup => (IconId::Plus, "New group"),
        RowKind::Group(_) => return (None, ""),
    };
    (Some(icon), label)
}

/// One row as a `NavItem`, lit while it stands for what the window shows.
pub(super) fn item(fb: &mut PaintBuffer, r: &Row, rail: bool, k: &Look, c: &Controls<RowKind>) {
    let (icon, label) = names(r.kind);
    let group = match r.kind {
        RowKind::Group(i) => k.groups.get(i).copied(),
        _ => None,
    };
    let at = area(group.map_or(r.rect, |_| dot_slot(r.rect).1));
    let it = NavItem::new(group.map_or(label, |g| g.0));
    let mut it = it.active(k.on == Some(r.kind)).state(c.state(r.kind));
    it = match (icon, rail) {
        (Some(id), true) => it.icon(id).compact(),
        (Some(id), false) => it.icon(id),
        (None, _) => it,
    };
    let count = if r.kind == RowKind::Tabs { Some(k.tabs) } else { group.map(|g| g.1) };
    if let Some(n) = count.filter(|_| !rail) {
        it = it.count(n as u32);
    }
    it.draw(fb, at);
}

pub(super) fn area(r: Rect) -> Area {
    Area { x: r.x.max(0) as u32, y: r.y.max(0) as u32, w: r.w.max(0) as u32, h: r.h.max(0) as u32 }
}
