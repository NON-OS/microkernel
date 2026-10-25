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

//! Host stand-in for nonos_app_skeleton's vfs client, over an in-memory store.

pub mod vfs {
    // Replaces app_skeleton/src/clients/vfs. read_file keeps what the real
    // chunked loop promises: the same path check, an error when the open
    // fails (here, no file), and a file longer than max_bytes handed back
    // cut to max_bytes, Ok, since the real loop stops there. write_file,
    // mkdir, stat_full and list_paths answer from the same store.
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    thread_local! {
        static FILES: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = const { RefCell::new(BTreeMap::new()) };
        static WRITES: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    }

    pub fn read_file(_owner: u32, path: &[u8], max_bytes: u32) -> Result<Vec<u8>, &'static str> {
        if path.is_empty() || path.len() > 255 {
            return Err("vfs path invalid");
        }
        FILES.with(|f| match f.borrow().get(path) {
            Some(data) => Ok(data[..data.len().min(max_bytes as usize)].to_vec()),
            None => Err("vfs open failed"),
        })
    }

    pub fn write_file(_owner: u32, path: &[u8], data: &[u8]) -> Result<(), &'static str> {
        WRITES.with(|w| w.borrow_mut().push(path.to_vec()));
        put(path, data.to_vec());
        Ok(())
    }

    pub fn mkdir(_owner: u32, _path: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn stat_full(_owner: u32, path: &[u8]) -> Result<(u64, bool, u64, bool), &'static str> {
        FILES.with(|f| match f.borrow().get(path) {
            Some(data) => Ok((data.len() as u64, false, 0, false)),
            None => Err("vfs stat failed"),
        })
    }

    pub fn list_paths(_owner: u32, _prefix: &[u8]) -> Result<Vec<String>, &'static str> {
        Ok(Vec::new())
    }

    /// Puts `data` at `path`, as a file already on the volume.
    pub fn put(path: &[u8], data: Vec<u8>) {
        FILES.with(|f| f.borrow_mut().insert(path.to_vec(), data));
    }

    /// The paths written, in order.
    pub fn writes() -> Vec<Vec<u8>> {
        WRITES.with(|w| w.borrow().clone())
    }
}
