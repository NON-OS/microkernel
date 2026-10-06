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

//! A host double for the store calls an install makes: the files of one
//! thread's store, and the refusal a read answers with when one is set, so
//! a proof can say what an install does with a store that would not answer.

use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::root::Key;

type Fail = &'static str;

thread_local! {
    static FILES: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = const { RefCell::new(BTreeMap::new()) };
    static REFUSE: RefCell<Option<Fail>> = const { RefCell::new(None) };
}

/// Start the thread's store over with `files` in it.
pub fn holding(files: &[(&[u8], &[u8])]) {
    FILES.with(|f| *f.borrow_mut() = files.iter().map(|(k, v)| (k.to_vec(), v.to_vec())).collect());
    REFUSE.with(|r| *r.borrow_mut() = None);
}

/// Every read from now on answers `why`, as the real calls do when vfs
/// does not answer.
pub fn refusing(why: Fail) {
    REFUSE.with(|r| *r.borrow_mut() = Some(why));
}

/// The file at `at` as the store holds it now.
pub fn held(at: &[u8]) -> Option<Vec<u8>> {
    FILES.with(|f| f.borrow().get(at).cloned())
}

pub fn read(at: &Key, max: u32) -> Result<Vec<u8>, Fail> {
    if let Some(why) = REFUSE.with(|r| *r.borrow()) {
        return Err(why);
    }
    match held(at.as_bytes()) {
        Some(mut bytes) => {
            bytes.truncate(max as usize);
            Ok(bytes)
        }
        /* What the vfs client answers for a file the store does not have. */
        None => Err("vfs stat failed"),
    }
}

pub fn write(at: &Key, data: &[u8]) -> Result<(), Fail> {
    FILES.with(|f| f.borrow_mut().insert(at.as_bytes().to_vec(), data.to_vec()));
    Ok(())
}
