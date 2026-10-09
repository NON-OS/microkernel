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

//! Where an install stands, as a phrase.

use alloc::format;
use alloc::string::String;

use nonos_market_proto::{installed_line, reason, removal_only, Stage};

/// The Marketplace window's words for it, from the same table, with what
/// to do next: install or uninstall again when that could work, and nothing
/// when it cannot. A reason only an uninstall stops with says uninstall.
pub(super) fn stage_text(id: &[u8], stage: Stage) -> String {
    match stage {
        Stage::Idle => "not installed".into(),
        Stage::Queued => "queued, waiting for the installer to start".into(),
        Stage::Installing => "installing: downloading and checking every file".into(),
        Stage::Installed => String::from(installed_line(id)),
        /* The kernel's table does not say which was refused, so neither is named. */
        Stage::Refused => {
            "the system refused the last install or uninstall of it before it started; ask again"
                .into()
        }
        Stage::Removing => "uninstalling: taking away what its install put here".into(),
        Stage::Removed => "uninstalled: what its install put here is gone".into(),
        Stage::Failed(code) => {
            let said = reason(code);
            let verb = if removal_only(code) { "uninstall" } else { "install" };
            match said.retry {
                true => format!("{} (reason {code}); `market {verb}` again", said.line),
                false => format!("{} (reason {code})", said.line),
            }
        }
    }
}
