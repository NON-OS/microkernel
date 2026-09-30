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

/*
 * What the catalogue says: tiers of files, each with its length, SHA-256
 * and the URLs it can be fetched from, the NONOS repository first.
 */

use alloc::string::String;
use alloc::vec::Vec;

pub struct File {
    /* The file's name, as on the volume without its slash. */
    pub name: String,
    pub bytes: u64,
    pub sha256: [u8; 32],
    pub mirrors: Vec<String>,
}

pub struct Tier {
    pub word: String,
    /* Bytes of memory the tier needs to run, as qwenchat plans it. */
    pub memory: u64,
    pub files: Vec<File>,
}

pub struct Catalogue {
    pub serial: u64,
    /* The NONOS repository's base URL; empty when the build named none. */
    pub base: String,
    pub tiers: Vec<Tier>,
}

impl Catalogue {
    pub fn tier(&self, word: &[u8]) -> Option<&Tier> {
        self.tiers.iter().find(|t| t.word.as_bytes() == word)
    }
}

impl Tier {
    pub fn bytes(&self) -> u64 {
        self.files.iter().map(|f| f.bytes).sum()
    }
}

impl File {
    /* The name the kernel's data calls take: a slash, then the file. */
    pub fn volume_name(&self) -> Vec<u8> {
        [b"/", self.name.as_bytes()].concat()
    }

    /*
     * Whether the volume can keep it: the kernel marks a stream beside it as
     * `<name>.partial`, and a directory entry holds 56 bytes of name.
     */
    pub fn keepable(&self) -> bool {
        self.name.len() + b".partial".len() <= 56
    }
}
