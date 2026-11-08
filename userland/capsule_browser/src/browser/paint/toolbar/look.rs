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

use nonos_toolkit::icons::IconId;

use crate::browser::shell::route::Route;
use crate::browser::ui::toolbar_geom::Part;

/// What the toolbar's controls show: whether Back and Forward have
/// somewhere to go, a load is running, a page is shown to bookmark and is
/// bookmarked, the tab has a page to read and the reader is open, and the
/// route the badge names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shows {
    pub back: bool,
    pub fwd: bool,
    pub loading: bool,
    pub page: bool,
    pub marked: bool,
    pub ready: bool,
    pub reading: bool,
    pub route: Route,
}

/// The icon a toolbar button shows: Stop in place of Reload while a page
/// loads, and a filled star while the page shown is bookmarked. The address
/// field and the route badge are not buttons and show none.
pub fn icon(part: Part, s: &Shows) -> Option<IconId> {
    Some(match part {
        Part::Back => IconId::Back,
        Part::Fwd => IconId::Forward,
        Part::Reload if s.loading => IconId::Close,
        Part::Reload => IconId::Reload,
        Part::Home => IconId::Home,
        Part::Star if s.marked => IconId::StarFilled,
        Part::Star => IconId::Star,
        Part::Reader => IconId::Reader,
        Part::Tabs => IconId::Tabs,
        Part::Sidebar => IconId::Sidebar,
        Part::Pill | Part::Route => return None,
    })
}

/// The address field's leading icon: a lock on an https page, else search.
pub fn lead(secure: bool) -> IconId {
    if secure {
        IconId::Lock
    } else {
        IconId::Search
    }
}
