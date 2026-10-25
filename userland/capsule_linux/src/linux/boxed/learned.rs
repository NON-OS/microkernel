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
//! What a learn run did: each path it wrote and each destination it tried to
//! connect to. A learn run is held to `offline` and to no network, so nothing
//! recorded here was let through that `offline` would refuse. learned_text.rs
//! makes it the tightest `nonos-box 1` file that would let a run do it again.

use alloc::string::String;
use alloc::vec::Vec;

/// Paths kept: past this many, the rest are not recorded.
pub const PATHS_MAX: usize = 256;

#[derive(Default)]
pub struct Learned {
    pub wrote: Vec<Vec<u8>>,
    pub tried: super::connected::Connected,
}

impl Learned {
    pub fn wrote(&mut self, path: &[u8]) {
        if self.wrote.len() < PATHS_MAX && !self.wrote.iter().any(|p| p == path) {
            self.wrote.push(path.to_vec());
        }
    }

    /// The file, with `refused` (what no file can allow) as its comment.
    pub fn file(&self, refused: &str) -> String {
        let tried = self.tried.all();
        let nets: usize = tried.iter().map(|d| d.len() + 5).sum();
        let room = super::learned_text::WORD_MAX.saturating_sub(b"allow:".len() + nets);
        let writes = super::learn_folders::folders(&self.wrote, room);
        super::learned_text::text(&writes, tried, refused)
    }
}
