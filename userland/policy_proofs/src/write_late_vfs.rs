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

//! The model vfs's calls: each acts first, then answers in time or late.

use super::{answer, Fate, Model};
use crate::write_steps::Vfs;

impl Vfs for Model {
    fn mkdir(&mut self, _path: &[u8]) -> Result<(), ()> {
        answer(self.fate())
    }
    fn unlink(&mut self, path: &[u8]) -> Result<(), ()> {
        let fate = self.fate();
        let _ = self.ram.remove(path);
        answer(fate)
    }
    fn write_file(&mut self, path: &[u8], data: &[u8]) -> Result<(), ()> {
        let fate = self.fate();
        // A refused write still opened the file: created and cut short.
        let done = if fate == Fate::Refused { &data[..data.len() / 2] } else { data };
        let _ = self.ram.insert(path.to_vec(), done.to_vec());
        answer(fate)
    }
    fn persist(&mut self, path: &[u8]) -> Result<(), ()> {
        let fate = self.fate();
        let Some(bytes) = self.ram.get(path).filter(|_| fate != Fate::Refused) else {
            return Err(());
        };
        let _ = self.disk.insert(path.to_vec(), bytes.clone());
        answer(fate)
    }
}
