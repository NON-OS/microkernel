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

//! Asking for the selected listing's install to be taken away.

use nonos_libc::mk_app_uninstall;

use super::asked::Asked;
use super::next_step::{on_remove, Uninstall};
use super::progress::Progress;
use super::state::State;

/// Ask for the selected listing's install to be taken away: a package's
/// files, or a Qwen tier's model files. Its program, if part of the image,
/// stays there. One that reads as never asked this boot is asked about all
/// the same (`next_step::on_remove`): a tier's model may be on the disk
/// from an earlier boot.
pub fn remove(state: &mut State) -> Asked {
    let Some(listing) = state.current() else {
        return Asked::NotInstallable;
    };
    if !listing.id.starts_with(b"linux.") {
        return Asked::InImage;
    }
    match on_remove(listing.progress) {
        Uninstall::Ask => {}
        Uninstall::Wait => return Asked::Moving,
        Uninstall::NotInstalled => return Asked::NotInstalled,
    }
    let asked = match mk_app_uninstall(&listing.id) {
        0 => Asked::Removing,
        -16 => Asked::Busy,
        errno => Asked::Refused(errno),
    };
    if asked == Asked::Removing {
        if let Some(l) = state.current_mut() {
            l.progress = Progress::Removing;
            l.removing = true;
        }
    }
    asked
}
