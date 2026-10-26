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

//! What a verified receipt says, for the command to print (cli_says.rs).

use alloc::string::String;
use alloc::vec::Vec;

use super::{Ran, Release};

/// What a verified receipt says. `enforced` is false for a learn run, held
/// to no policy of its own and never one to rely on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub release: String,
    /// Whether it came from a release, apart from whether it verified.
    pub is_release: Release,
    pub pcr9: [u8; 32],
    pub box_word: String,
    pub box_hash: [u8; 32],
    pub enforced: bool,
    /// None for the personality's own BusyBox, whose note names no program.
    pub program: Option<[u8; 32]>,
    pub app_id: [u8; 32],
    pub built_in: bool,
    pub supervisor: [u8; 32],
    pub seal_no: u64,
    /// The capsule that asked, one of `ran`; zero if none is named.
    pub launcher: [u8; 32],
    /// Every capsule the machine ran when the doc was made.
    pub ran: Vec<Ran>,
}
