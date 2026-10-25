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
    // cut to max_bytes, Ok, since the real loop stops there. stat,
    // write_file and persist answer from the same store; persist records.
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    thread_local! {
        static FILES: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = const { RefCell::new(BTreeMap::new()) };
        static PERSISTED: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
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

    pub fn stat(_owner: u32, path: &[u8]) -> Result<(u64, bool), &'static str> {
        FILES.with(|f| f.borrow().get(path).map(|d| (d.len() as u64, false)).ok_or("vfs stat"))
    }

    pub fn write_file(_owner: u32, path: &[u8], data: &[u8]) -> Result<(), &'static str> {
        put(path, data.to_vec());
        Ok(())
    }

    pub fn persist(_owner: u32, path: &[u8]) -> Result<(), &'static str> {
        PERSISTED.with(|p| p.borrow_mut().push(path.to_vec()));
        Ok(())
    }

    /// Puts `data` at `path`, as a file already on the volume.
    pub fn put(path: &[u8], data: Vec<u8>) {
        FILES.with(|f| f.borrow_mut().insert(path.to_vec(), data));
    }

    pub fn get(path: &[u8]) -> Option<Vec<u8>> {
        FILES.with(|f| f.borrow().get(path).cloned())
    }

    /// The paths persisted, in order.
    pub fn persisted() -> Vec<Vec<u8>> {
        PERSISTED.with(|p| p.borrow().clone())
    }
}
