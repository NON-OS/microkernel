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

use alloc::string::String;

use super::budget::{copy_of, grow};
use super::types::{File, Store, StoreError, StoreResult, MAX_FILES, MAX_FILE_BYTES};

impl Store {
    /*
     * Place one chunk of an artifact at `path`. Offset zero replaces the whole
     * file; a later chunk must start exactly where the previous one ended, so a
     * dropped or reordered chunk fails instead of stitching a corrupt image.
     * Directories are never overwritten and no fd is involved, so this cannot
     * be reached by a caller holding only a descriptor.
     */
    pub fn install_bytes(
        &mut self,
        path: &str,
        offset: usize,
        bytes: &[u8],
        owner: u32,
    ) -> StoreResult<()> {
        if offset.saturating_add(bytes.len()) > MAX_FILE_BYTES {
            return Err(StoreError::Full);
        }
        if let Some(idx) = self.find(path) {
            if self.files[idx].is_dir {
                return Err(StoreError::IsDir);
            }
        }
        if offset == 0 {
            return self.replace(path, bytes, owner);
        }
        let idx = self.find(path).ok_or(StoreError::NotFound)?;
        if self.files[idx].data.len() != offset {
            return Err(StoreError::Inval);
        }
        let others = self.held_except(Some(idx));
        let data = &mut self.files[idx].data;
        grow(data, offset.saturating_add(bytes.len()), others)?;
        data.extend_from_slice(bytes);
        self.files[idx].mtime = super::time::now_ms();
        Ok(())
    }

    fn replace(&mut self, path: &str, bytes: &[u8], owner: u32) -> StoreResult<()> {
        if self.find(path).is_some() {
            self.unlink(path)?;
        }
        if self.files.len() >= MAX_FILES || !self.may_name(owner, 1) {
            return Err(StoreError::Full);
        }
        let data = copy_of(bytes, self.held_except(None))?;
        self.files.push(File::new(String::from(path), data, false, owner));
        Ok(())
    }
}
