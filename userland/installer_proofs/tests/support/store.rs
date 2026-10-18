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

//! Host stand-ins for nonos_app_skeleton's vfs stat, store_install and store_uninstall, over one store.
//
// Replace app_skeleton/src/clients/vfs/stat.rs, store_install.rs and
// store_uninstall.rs, which ask the vfs over IPC; each one's path check is
// kept. store_install records the call and stores the file, also with the
// read stand-in so read_file sees it; store_uninstall records and drops it;
// stat answers (length, not a directory) for a stored path, Err otherwise.

use std::cell::RefCell;
use std::collections::BTreeMap;

const ERR_INVALID: i32 = -22;

thread_local! {
    static STORE: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = const { RefCell::new(BTreeMap::new()) };
    static INSTALLS: RefCell<Vec<(Vec<u8>, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
    static UNINSTALLS: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

fn path_ok(path: &[u8]) -> bool {
    !path.is_empty() && path.len() <= 255
}

pub fn stat(_owner_pid: u32, path: &[u8]) -> Result<(u64, bool), &'static str> {
    path_ok(path).then_some(()).ok_or("vfs path invalid")?;
    STORE.with(|s| s.borrow().get(path).map(|b| (b.len() as u64, false)).ok_or("vfs stat failed"))
}

pub fn store_install(path: &[u8], bytes: &[u8]) -> Result<(), i32> {
    path_ok(path).then_some(()).ok_or(ERR_INVALID)?;
    INSTALLS.with(|i| i.borrow_mut().push((path.to_vec(), bytes.to_vec())));
    STORE.with(|s| s.borrow_mut().insert(path.to_vec(), bytes.to_vec()));
    crate::clients::vfs::put(path, bytes.to_vec());
    Ok(())
}

pub fn store_uninstall(path: &[u8]) -> Result<(), i32> {
    path_ok(path).then_some(()).ok_or(ERR_INVALID)?;
    UNINSTALLS.with(|u| u.borrow_mut().push(path.to_vec()));
    STORE.with(|s| s.borrow_mut().remove(path));
    Ok(())
}

/// Empties the store and the records of installs and uninstalls.
pub fn store_reset() {
    STORE.with(|s| s.borrow_mut().clear());
    INSTALLS.with(|i| i.borrow_mut().clear());
    UNINSTALLS.with(|u| u.borrow_mut().clear());
}

/// (path, bytes) of each store_install since the last call, in order.
pub fn installs() -> Vec<(Vec<u8>, Vec<u8>)> {
    INSTALLS.with(|i| std::mem::take(&mut *i.borrow_mut()))
}

pub fn uninstalls() -> Vec<Vec<u8>> {
    UNINSTALLS.with(|u| std::mem::take(&mut *u.borrow_mut()))
}
