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

//! Asking for the selected listing to be installed, or started.

use nonos_libc::{mk_app_install, mk_app_launch};

use super::next_step::can_open;
use super::state::State;

pub use super::asked::Asked;
pub use super::install_direct::{direct, direct_offered_for};
pub use super::install_remove::remove;

pub fn ask(state: &State) -> Asked {
    let Some(listing) = state.current() else {
        return Asked::NotInstallable;
    };
    if !listing.id.starts_with(b"linux.") {
        return Asked::InImage;
    }
    // `ready` only saves a request: the kernel asks the market again.
    if !listing.ready {
        return Asked::NotReady;
    }
    match mk_app_install(&listing.id, b"") {
        0 => Asked::Queued,
        -16 => Asked::Busy,
        errno => Asked::Refused(errno),
    }
}

/// Ask for the selected listing's program to start.
pub fn open(state: &State) -> Asked {
    let Some(listing) = state.current() else {
        return Asked::NotInstallable;
    };
    if !listing.id.starts_with(b"linux.") {
        return Asked::InImage;
    }
    if !can_open(&listing.id, listing.progress) {
        return Asked::InstallFirst;
    }
    match mk_app_launch(&listing.id) {
        0 => Asked::Opening,
        -16 => Asked::Busy,
        errno => Asked::Refused(errno),
    }
}
