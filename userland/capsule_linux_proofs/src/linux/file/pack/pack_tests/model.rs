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

//! A store that keeps files by name, holds one open if asked, is full if
//! asked, and stops dead after `steps` steps, as a crash does.

use std::collections::BTreeMap;

use super::super::shelf::{Shelf, Unlinked};

#[derive(Clone, Default, PartialEq, Debug)]
pub(super) struct Model {
    pub files: BTreeMap<Vec<u8>, Vec<u8>>,
    pub held: Option<Vec<u8>>,
    pub steps: Option<usize>,
    pub full: bool,
}

impl Model {
    fn step(&mut self) -> bool {
        match &mut self.steps {
            Some(0) => false,
            Some(n) => {
                *n -= 1;
                true
            }
            None => true,
        }
    }
}

impl Shelf for Model {
    fn write(&mut self, at: &[u8], bytes: &[u8]) -> bool {
        let ok = self.step() && !self.full;
        if ok {
            self.files.insert(at.to_vec(), bytes.to_vec());
        }
        ok
    }

    fn unlink(&mut self, at: &[u8]) -> Unlinked {
        match self.step() {
            false => Unlinked::Failed,
            true if self.held.as_deref() == Some(at) => Unlinked::Busy,
            true => {
                self.files.remove(at);
                Unlinked::Gone
            }
        }
    }

    fn rename(&mut self, from: &[u8], to: &[u8]) -> bool {
        if !self.step() || self.files.contains_key(to) {
            return false;
        }
        let Some(bytes) = self.files.remove(from) else { return false };
        self.files.insert(to.to_vec(), bytes);
        true
    }
}
