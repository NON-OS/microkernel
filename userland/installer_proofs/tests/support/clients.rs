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

//! Host stand-in for nonos_app_skeleton's clients::vfs::read_file, over fixtures.

pub mod vfs {
    // Replaces app_skeleton/src/clients/vfs/read_file.rs, which opens the path
    // over IPC and reads chunks until a short read or until max_bytes are
    // held. What it promises is kept: the same path check, an error when the
    // open fails (here, no fixture), and a file longer than max_bytes handed
    // back cut to max_bytes, Ok, since the real loop stops there.
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    pub type ReadCall = (u32, Vec<u8>, u32);

    thread_local! {
        static FILES: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = const { RefCell::new(BTreeMap::new()) };
        static READS: RefCell<Vec<ReadCall>> = const { RefCell::new(Vec::new()) };
    }

    pub fn read_file(owner_pid: u32, path: &[u8], max_bytes: u32) -> Result<Vec<u8>, &'static str> {
        READS.with(|r| r.borrow_mut().push((owner_pid, path.to_vec(), max_bytes)));
        if path.is_empty() || path.len() > 255 {
            return Err("vfs path invalid");
        }
        FILES.with(|f| match f.borrow().get(path) {
            Some(data) => Ok(data[..data.len().min(max_bytes as usize)].to_vec()),
            None => Err("vfs open failed"),
        })
    }

    /// Empties the store and the record of reads.
    pub fn reset() {
        FILES.with(|f| f.borrow_mut().clear());
        READS.with(|r| r.borrow_mut().clear());
    }

    pub fn put(path: &[u8], data: Vec<u8>) {
        FILES.with(|f| f.borrow_mut().insert(path.to_vec(), data));
    }

    pub fn reads() -> Vec<ReadCall> {
        READS.with(|r| r.borrow().clone())
    }
}
