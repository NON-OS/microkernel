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

//! One row per proof, in the order the boot reached them.

use alloc::string::String;

use super::mark::Mark;
use super::{capsules, kernel, loader, policy};
use crate::install::source::Boot;

pub const ROWS: usize = 5;

pub struct Row {
    pub name: &'static str,
    pub mark: Mark,
    /// What the word rests on, in words.
    pub says: String,
    /// The short hex of what was proven, or counts; empty when there is none.
    pub detail: String,
}

impl Row {
    pub(super) fn new(name: &'static str, mark: Mark, says: &str, detail: String) -> Row {
        Row { name, mark, says: String::from(says), detail }
    }
}

/// The kernel the bootloader admitted and its signature, the bootloader the
/// kernel checked, then the capsule root and the capsules checked against it.
pub fn rows(b: &Boot) -> [Row; ROWS] {
    let k = kernel::kernel(b);
    let root = policy::capsule_root(b, k.mark);
    [k, kernel::signature(b), loader::bootloader(b), root, capsules::capsules(b)]
}
