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

//! Who may attach a surface (L18c). A handle is a slot and a small counter,
//! easy to guess, so holding one proves nothing: the owner names receivers,
//! as thread groups. Pure, so kernel_proofs checks it by path.

/// Named receivers per surface: a decode buffer goes to one codec, so few.
pub(crate) const NAMED_MAX: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Named {
    Added,
    Already,
    Full,
    Refused,
}

#[derive(Clone, Copy)]
pub(crate) struct Shares {
    owner: u32,
    named: [u32; NAMED_MAX],
}

impl Shares {
    /// Shared with no one: only the owner's own group may attach.
    pub(crate) const fn new(owner_group: u32) -> Self {
        Self { owner: owner_group, named: [0; NAMED_MAX] }
    }

    /// Name `group` as a receiver. Group 0 is no process.
    pub(crate) fn name(&mut self, group: u32) -> Named {
        if group == 0 {
            return Named::Refused;
        }
        if group == self.owner || self.named.contains(&group) {
            return Named::Already;
        }
        match self.named.iter_mut().find(|g| **g == 0) {
            Some(free) => {
                *free = group;
                Named::Added
            }
            None => Named::Full,
        }
    }

    /// May `receiver` attach? Only the owner's group or a group it named:
    /// the compositor too is a receiver only when named (K20).
    pub(crate) fn may_attach(&self, receiver: u32) -> bool {
        receiver != 0 && (receiver == self.owner || self.named.contains(&receiver))
    }
}
