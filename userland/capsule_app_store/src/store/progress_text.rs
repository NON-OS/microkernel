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

//! How an install's progress reads to a person, and in what colour.

use super::progress::Progress;
use crate::store::theme::{ACCENT, DANGER, MUTED, OK};

impl Progress {
    /// A sentence for the detail pane, with its colour, when there is news.
    pub fn sentence(self) -> Option<(&'static [u8], u32)> {
        let line: (&'static [u8], u32) = match self {
            Progress::Idle => return None,
            Progress::Queued => (b"Waiting for the installer to start", MUTED),
            Progress::Installing => (b"Downloading and checking every file", ACCENT),
            Progress::Installed => (b"Installed. Press Enter to open it", OK),
            Progress::Refused => (b"The system would not start the installer", DANGER),
            Progress::Failed(2) => (b"The package index did not verify", DANGER),
            Progress::Failed(3) => (b"Something it needs is in no index", DANGER),
            Progress::Failed(4) => (b"It needs more packages than this machine allows", DANGER),
            Progress::Failed(5) => (b"A package did not download, or did not verify", DANGER),
            Progress::Failed(8) => (b"This system has no mirror for it", DANGER),
            Progress::Failed(9) => (b"This system holds no key to check it with", DANGER),
            Progress::Failed(_) => (b"The install stopped", DANGER),
        };
        Some(line)
    }
}
