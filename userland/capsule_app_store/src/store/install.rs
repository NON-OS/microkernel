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

//! Asking for the selected listing to be installed.

use nonos_libc::{mk_app_install, mk_app_launch};

use super::state::State;

/// What the user is told after asking.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Asked {
    Queued,
    Opening,
    Busy,
    Refused,
    NotInstallable,
}

impl Asked {
    pub fn label(self) -> &'static [u8] {
        match self {
            Asked::Queued => b"install requested",
            Asked::Opening => b"starting",
            Asked::Busy => b"too many installs already queued",
            Asked::Refused => b"the system refused the request",
            Asked::NotInstallable => b"nothing to fetch for this listing",
        }
    }
}

pub fn ask(state: &State) -> Asked {
    let Some(listing) = state.current() else {
        return Asked::NotInstallable;
    };
    // `ready` only saves a request: the kernel asks the market again.
    if !listing.id.starts_with(b"linux.") || !listing.ready {
        return Asked::NotInstallable;
    }
    match mk_app_install(&listing.id, b"") {
        0 => Asked::Queued,
        -16 => Asked::Busy,
        _ => Asked::Refused,
    }
}

/// Ask for the selected listing's program to start.
pub fn open(state: &State) -> Asked {
    let Some(listing) = state.current() else {
        return Asked::NotInstallable;
    };
    if !listing.id.starts_with(b"linux.") {
        return Asked::NotInstallable;
    }
    match mk_app_launch(&listing.id) {
        0 => Asked::Opening,
        -16 => Asked::Busy,
        _ => Asked::Refused,
    }
}
