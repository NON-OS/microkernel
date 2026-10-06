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

/* One line under each screen's title: what the screen is for. */

use super::proofs_subtitle::proofs_subtitle;
use crate::install::source::Boot;
use crate::install::state::Screen;

pub(super) fn subtitle(screen: Screen, boot: &Boot) -> &'static str {
    match screen {
        Screen::Welcome => "What this computer booted, and what it has, read from the machine.",
        Screen::Proofs => proofs_subtitle(boot.available, boot.proof, boot.path_only),
        Screen::Disks => "The disks this boot's drivers serve. The one you choose is erased.",
        Screen::Confirm => "The plan for this disk. Nothing is written until the word is typed.",
        Screen::Writing => "Writing the new disk. Keep the computer on.",
        Screen::Verifying => "Reading every sector back. Keep the computer on.",
        Screen::Done => "Every sector read back as written.",
        Screen::Failed => "The install stopped.",
    }
}
