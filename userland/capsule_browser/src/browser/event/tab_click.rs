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

use nonos_app_skeleton::EventOutcome;

use crate::browser::paint::pages::group_head;
use crate::browser::shell::{Page, Shell};
use crate::browser::tab_groups;
use crate::browser::tabs::Tabs;
use crate::browser::ui::page_part::PagePart;

/// A control on the Tabs page. Filtered to a group, its name starts a
/// rename and Delete group drops it. On a card, the close button closes
/// the tab, the chip moves it to the next group, and the rest switches to
/// it.
pub(super) fn act(tabs: &mut Tabs, shell: &mut Shell, part: PagePart) -> EventOutcome {
    match part {
        PagePart::Name => {
            let Some(name) = group_head::filtered(shell).map(|g| g.name.clone()) else {
                return EventOutcome::Idle;
            };
            shell.rename.get_or_insert(name);
        }
        PagePart::Delete => {
            let Some(id) = group_head::filtered(shell).map(|g| g.id) else {
                return EventOutcome::Idle;
            };
            tab_groups::delete(tabs, shell, id);
        }
        PagePart::Close(i, _) => {
            tabs.close(i);
        }
        PagePart::Chip(i, _) => tab_groups::cycle(tabs, shell, i),
        PagePart::Tab(i, _) => {
            tabs.switch(i);
            shell.show(Page::Web);
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
