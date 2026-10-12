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

//! A store held in memory, which notes each file it is asked for.

use std::collections::HashMap;

use super::rtl::blobs::Blobs;

#[derive(Default)]
pub struct Mem {
    pub files: HashMap<&'static str, Vec<u8>>,
    pub asked: Vec<&'static str>,
}

impl Blobs for Mem {
    fn read(&mut self, file: &str, max: usize) -> Option<Vec<u8>> {
        let (name, bytes) = self.files.get_key_value(file)?;
        self.asked.push(name);
        Some(bytes[..bytes.len().min(max)].to_vec())
    }
}
