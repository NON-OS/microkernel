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

use nonos_libc::{mk_app_install, mk_app_launch, mk_app_uninstall};

use super::next_step::{can_open, on_remove, Uninstall};
use super::progress::Progress;
use super::state::State;

/// What the user is told after asking.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Asked {
    Queued,
    Opening,
    Busy,
    /// Refused outright, with the kernel's errno (`refused`).
    Refused(i64),
    NotInstallable,
    /// A NONOS capsule or community listing: part of this system's image,
    /// so there is nothing to install and the dock starts it.
    InImage,
    /// A Linux listing the market says this machine cannot install yet.
    NotReady,
    /// The last install stopped for a reason no retry changes.
    Cannot,
    /// An uninstall was queued.
    Removing,
    /// Only something installed can be uninstalled.
    NotInstalled,
    /// An install or uninstall of it is still moving.
    Moving,
    /// A package not installed has nothing to start.
    InstallFirst,
    /// A tier's install was queued, its model to come over a direct
    /// connection, as the person asked with `d`.
    QueuedDirect,
    /// `d` on a listing with no direct download to offer.
    NoDirect,
}

impl Asked {
    pub fn label(self) -> &'static [u8] {
        match self {
            Asked::Queued => b"install requested",
            // The exec gate has the last word, after this window has asked.
            Asked::Opening => b"asked to start it; the system checks it first",
            // EBUSY: the queue is full, or holds this very request already.
            Asked::Busy => b"the system is busy with it or with others; ask again soon",
            Asked::Refused(errno) => super::refused::refused(errno),
            Asked::NotInstallable => b"nothing is selected",
            Asked::InImage => b"part of this system: start it from the dock",
            Asked::NotReady => b"this machine cannot install it yet: see its gates",
            Asked::Cannot => b"asking again cannot change this",
            Asked::Removing => b"uninstall requested",
            Asked::NotInstalled => b"it is not installed, so there is nothing to uninstall",
            Asked::Moving => b"it is still moving; uninstall it once it stops",
            Asked::InstallFirst => b"it is not installed: install it, then open it",
            Asked::QueuedDirect => {
                b"install requested, downloading direct: the mirror sees this machine's address"
            }
            Asked::NoDirect => b"no direct download to offer for this one",
        }
    }
}

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

/// `d`: ask for the selected Qwen tier to be installed with its model
/// downloaded over a direct connection, for this install only. Offered
/// only where a download through an anonymity network would be large
/// (`route_offer::direct_offered`); init reads the release word `@direct`.
pub fn direct(state: &mut State) -> Asked {
    let Some(listing) = state.current() else {
        return Asked::NotInstallable;
    };
    if !direct_offered_for(state, listing) {
        return Asked::NoDirect;
    }
    if !listing.ready {
        return Asked::NotReady;
    }
    let asked = match mk_app_install(&listing.id, DIRECT_RELEASE) {
        0 => Asked::QueuedDirect,
        -16 => Asked::Busy,
        errno => Asked::Refused(errno),
    };
    if asked == Asked::QueuedDirect {
        if let Some(l) = state.current_mut() {
            l.progress = Progress::Queued;
            l.removing = false;
        }
    }
    asked
}

/// The release word init reads as a direct download (linux_jobs/direct.rs).
const DIRECT_RELEASE: &[u8] = b"@direct";

/// Whether `d` would ask for a direct download of `l`'s model.
pub fn direct_offered_for(state: &State, l: &super::listing::Listing) -> bool {
    let left = super::tier_fit::tier_of(&l.id)
        .and_then(|t| super::tier_fit::weight(t, super::model_weights::MODEL_WEIGHTS));
    left.is_some_and(|left| {
        let why = match l.progress {
            super::progress::Progress::Failed(w) => Some(w),
            _ => None,
        };
        super::route_offer::direct_offered(state.route, left, l.progress.uninstalled(), why)
    })
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
