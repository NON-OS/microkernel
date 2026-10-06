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

//! What Enter and `u` mean for a listing, from where it stands alone, apart
//! from the asking so market_proofs can hold it.
//!
//! A stopped uninstall is followed by another uninstall, never an install:
//! the button says Retry, and a Retry that installed what the person had
//! just asked to take away would do the opposite of what it says. Whether
//! the last request was an uninstall is known from this window's own asking
//! (`removing`), and, for a window opened after it, from the reason only an
//! uninstall stops with.

use nonos_market_proto::reason::WHY_NOT_INSTALLED;
use nonos_market_proto::removal_only;

use super::progress::Progress;

/// What Enter does next.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Enter {
    /// Installed: start it.
    Open,
    /// Still moving, or no NONOS disk to hold it: nothing to ask.
    Wait,
    /// An uninstall that stopped and may finish if asked again.
    Remove,
    /// It stopped for a reason asking again cannot change.
    Cannot,
    /// Ask for it to be installed.
    Install,
}

/// What `u` does next.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Uninstall {
    /// Ask the system to take it away.
    Ask,
    /// An install or uninstall of it is still moving.
    Wait,
    /// Nothing of it was installed for an uninstall to take away.
    NotInstalled,
}

/// Whether the last request for a listing at `p` was an uninstall.
pub fn uninstalling(p: Progress, removing: bool) -> bool {
    removing || p == Progress::Removing || matches!(p, Progress::Failed(c) if removal_only(c))
}

/// Whether the system refused to start the uninstall asked last, so the
/// pane says so rather than that it would not start an installer.
pub fn refused_uninstall(p: Progress, removing: bool) -> bool {
    p == Progress::Refused && uninstalling(p, removing)
}

pub fn on_enter(p: Progress, removing: bool) -> Enter {
    match p {
        Progress::Installed => Enter::Open,
        /* An uninstall found nothing installed: as good as never installed. */
        Progress::Failed(WHY_NOT_INSTALLED) => Enter::Install,
        p if p.pending() || p.needs_installed_system() => Enter::Wait,
        p @ (Progress::Refused | Progress::Failed(_)) if uninstalling(p, removing) => {
            match p.retryable() {
                true => Enter::Remove,
                false => Enter::Cannot,
            }
        }
        p @ (Progress::Refused | Progress::Failed(_)) if !p.retryable() => Enter::Cannot,
        _ => Enter::Install,
    }
}

/*
 * The status table is this boot's, so a listing installed before the last
 * restart reads as never asked (Idle): the system is asked all the same, as
 * the Terminal's `market uninstall` asks it, and the personality says
 * whether any install of it is recorded. A Qwen tier's model kept on an
 * installed disk is taken off it this way, and so is the part of a model
 * a download that stopped left behind, which on a live session is memory.
 */
pub fn on_remove(p: Progress) -> Uninstall {
    match p {
        p if p.pending() => Uninstall::Wait,
        Progress::Removed | Progress::Failed(WHY_NOT_INSTALLED) => Uninstall::NotInstalled,
        _ => Uninstall::Ask,
    }
}

/// Whether `o` asks for a listing to start: one installed, or a Qwen tier,
/// whose program is part of the image and whose window says itself when
/// its model is not here yet. A package not installed has nothing to
/// start, and asking would end in nothing on screen at all.
pub fn can_open(listing: &[u8], p: Progress) -> bool {
    p == Progress::Installed || listing.starts_with(b"linux.qwen-")
}
