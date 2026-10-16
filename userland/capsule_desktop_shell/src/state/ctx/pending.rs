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

//! What waits on another process or on a person: open-with paths, a
//! Terminal command, the consent and install prompts, the live-session
//! offer and a desktop Delete's confirmation.

pub struct Pending {
    /// Open-with broker: path handed to a target app's service name by
    /// `OP_OPEN_WITH`, pulled once via `OP_TAKE_OPEN_ARG` after the app wakes.
    pub pending_open: alloc::collections::BTreeMap<alloc::string::String, alloc::string::String>,
    /// A command line a Launchpad tool tile left for the Terminal, answered
    /// only to a Terminal window and only while still wanted
    /// (state::open_arg). Kept apart from `pending_open`, which any process
    /// can fill through `OP_OPEN_WITH`.
    pub pending_command: Option<crate::state::open_arg::PendingCommand>,
    /// Name of the runtime-installed app whose launch is awaiting the consent
    /// modal, or None when no dialog is up.
    pub pending_consent: Option<alloc::vec::Vec<u8>>,
    /// The /pkgs package whose install awaits the consent modal, carrying the
    /// summary pkg_query returned, or None when no dialog is up.
    pub pending_pkg_install: Option<crate::state::PkgInstallPrompt>,
    /// The offer to install NONOS, shown once at the start of a live session.
    pub live_prompt: crate::state::live_prompt::LivePrompt,
    /// A desktop Delete waiting on its confirmation.
    pub pending_delete: crate::state::delete_prompt::DeletePrompt,
}

impl Pending {
    pub fn new() -> Self {
        Pending {
            pending_open: alloc::collections::BTreeMap::new(),
            pending_command: None,
            pending_consent: None,
            pending_pkg_install: None,
            live_prompt: crate::state::live_prompt::LivePrompt::new(),
            pending_delete: crate::state::delete_prompt::DeletePrompt::new(),
        }
    }
}
