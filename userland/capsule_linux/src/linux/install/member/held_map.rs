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

//! Which installed packages hold each path, kept for one install. The
//! member check asked every other package's record for every package it
//! placed, and searched each record's paths one by one: Telegram's 254
//! packages after 150 installed read some forty thousand records. Built
//! once from the records, kept as this install writes and takes them out,
//! and asked by path. It answers as conflict.rs does. Pure, so the host
//! proofs hold it.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::manifest::Manifest;

#[derive(Default)]
pub struct Held {
    names: Vec<String>,
    /// Each path, and every package whose record holds it.
    owners: BTreeMap<Vec<u8>, Vec<usize>>,
}

impl Held {
    /// `name`'s record as it now is: what it held before is let go first,
    /// and an empty record holds nothing.
    pub fn keep(&mut self, name: &str, m: &Manifest) {
        let at = self.at(name);
        self.owners.retain(|_, holders| {
            holders.retain(|h| *h != at);
            !holders.is_empty()
        });
        for path in m.files.iter().chain(&m.links) {
            let holders = self.owners.entry(path.clone()).or_default();
            if !holders.contains(&at) {
                holders.push(at);
            }
        }
    }

    /// The first of `mine` that a package other than `name` holds.
    pub fn held_by_another<'a>(&self, mine: &'a [Vec<u8>], name: &str) -> Option<&'a [u8]> {
        let own = self.names.iter().position(|n| n == name);
        let other =
            |p: &&Vec<u8>| self.owners.get(*p).is_some_and(|h| h.iter().any(|&i| Some(i) != own));
        mine.iter().find(other).map(Vec::as_slice)
    }

    fn at(&mut self, name: &str) -> usize {
        if let Some(i) = self.names.iter().position(|n| n == name) {
            return i;
        }
        self.names.push(String::from(name));
        self.names.len() - 1
    }
}
