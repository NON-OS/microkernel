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

use crate::browser::shell::{store, Page, Shell};
use crate::browser::state::{Origin, State};

/// A History or Bookmarks row opens entry `i`'s page in the tab in view.
pub(super) fn open(state: &mut State, shell: &mut Shell, i: usize) -> EventOutcome {
    let url = match shell.page {
        Page::History => shell.visits.get(i).map(|v| v.url.clone()),
        Page::Bookmarks => shell.bookmarks.get(i).map(|b| b.url.clone()),
        _ => None,
    };
    let Some(url) = url else {
        return EventOutcome::Idle;
    };
    shell.show(Page::Web);
    super::navigate::navigate(state, url, Origin::User)
}

/// A row's remove button: a bookmark goes, saved merged with what another
/// window saved; a download's record goes, and the saved file stays.
pub(super) fn remove(shell: &mut Shell, i: usize) -> EventOutcome {
    match shell.page {
        Page::Bookmarks => {
            let Some(url) = shell.bookmarks.get(i).map(|b| b.url.clone()) else {
                return EventOutcome::Idle;
            };
            shell.toggle_bookmark(&url, "", 0);
            store::sync(shell);
        }
        Page::Downloads if i < shell.downloads.len() => {
            shell.downloads.remove(i);
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
