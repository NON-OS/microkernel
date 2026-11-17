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

use super::look::Shows;
use crate::browser::net::mixnet;
use crate::browser::paint::tokens::mode;
use crate::browser::settings::NETWORKS;
use crate::browser::shell::types::DlState;
use crate::browser::shell::Shell;
use crate::browser::state::State;

/* What the page's buttons show, read from the window and the active tab.
 * A download still being saved is not one the list can clear. */
pub(super) fn shows(state: &State, shell: &Shell) -> Shows {
    let r = &shell.reader;
    Shows {
        appearance: mode() as u8,
        network: NETWORKS.iter().position(|n| *n == mixnet::chosen()).map(|i| i as u8),
        custom: shell.search.is_some(),
        theme: r.theme,
        width: r.width,
        size: r.size,
        proxy: state.proxy.is_some(),
        history: !shell.visits.is_empty(),
        downloads: shell.downloads.iter().any(|d| d.state != DlState::Saving),
        bookmarks: shell.bookmarks.len(),
        armed: shell.settings.armed,
    }
}
