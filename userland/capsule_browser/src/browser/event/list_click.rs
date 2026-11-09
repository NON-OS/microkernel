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

use nonos_app_skeleton::{feed, EventOutcome, InputEvent};
use nonos_toolkit::widgets::Step;

use crate::browser::shell::{Page, Shell};
use crate::browser::tab_groups;
use crate::browser::tabs::Tabs;
use crate::browser::ui::page_key::live;
use crate::browser::ui::page_part::{PagePart, Pick};

use super::{list_rows, tab_click};

/// A press at content-local `at` on a list page or the Tabs page, fed to
/// the controls its last paint laid out, so it acts on what was drawn
/// there and a disabled pill takes nothing. A row or tab acts only while
/// its index still holds what was drawn there; when the list or the tabs
/// moved under it, the press only repaints. Anything but the group's name
/// ends a rename in progress, and a press on nothing then repaints.
pub(super) fn click(tabs: &mut Tabs, shell: &mut Shell, at: &InputEvent) -> EventOutcome {
    let hit = match feed(&mut tabs.active_mut().ui.pages, at) {
        Step::Act(p) => Some(p),
        _ => None,
    };
    let renaming = shell.rename.is_some();
    if hit != Some(PagePart::Name) {
        tab_groups::commit_rename(shell);
    }
    match hit {
        Some(PagePart::Pill(p)) => pick(shell, p),
        Some(p) if !live(shell, p) => EventOutcome::Repaint,
        Some(PagePart::Open(i, _)) => list_rows::open(tabs.active_mut(), shell, i),
        Some(PagePart::Remove(i, _)) => list_rows::remove(shell, i),
        Some(part) => tab_click::act(tabs, shell, part),
        None if renaming => EventOutcome::Repaint,
        None => EventOutcome::Idle,
    }
}

/* A pill: History's or Downloads' filter, the Tabs page's group filter, or
 * a new group. The list starts again from its top. */
fn pick(shell: &mut Shell, pick: Pick) -> EventOutcome {
    match pick {
        Pick::Filter(i) if shell.page == Page::Downloads => shell.dl_filter = i,
        Pick::Filter(i) => shell.hist_filter = i,
        Pick::Group(id) => tab_groups::show(shell, id),
        Pick::NewGroup => {
            tab_groups::create(shell);
        }
    }
    shell.page_scroll = 0;
    EventOutcome::Repaint
}
