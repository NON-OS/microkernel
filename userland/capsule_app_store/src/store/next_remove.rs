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

//! What `u` does next, and whether the last request was an uninstall.

use nonos_market_proto::reason::WHY_NOT_INSTALLED;
use nonos_market_proto::removal_only;

use super::super::progress::Progress;

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
