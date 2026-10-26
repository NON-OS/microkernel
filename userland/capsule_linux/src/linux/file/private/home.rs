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

//! The family's home: /home and /root when its policy grants FILES_HOME,
//! kept under a root its listing names (app/home_key.rs) instead of the
//! run's random id, so the next run of the same app finds it. It lives in
//! the store's memory, outside /linux and /linux-private, and lasts the
//! boot; nothing of it reaches a disk.

use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

pub const HOMES: &[u8] = b"/linux-home";

struct Key(RefCell<Option<[u8; 32]>>);

// SAFETY: only the serve loop resolves a path; a guest only traps into it.
unsafe impl Sync for Key {}

static KEY: Key = Key(RefCell::new(None));

/// The store root of the home `key` names: its first 128 bits, so a guest
/// path under it keeps most of the store's 255 bytes.
fn root_of(key: &[u8; 32]) -> Vec<u8> {
    let mut out = [HOMES, b"/"].concat();
    out.extend(key[..16].iter().flat_map(|b| alloc::format!("{b:02x}").into_bytes()));
    out
}

/// This family's home root, once one is open.
pub fn root() -> Option<Vec<u8>> {
    KEY.0.borrow().as_ref().map(root_of)
}

/// The home `key` names, made if it is not there, with /home and /root in
/// it. False when it cannot be, and the guest must not start.
pub fn open(key: [u8; 32]) -> bool {
    let (pid, at) = (mk_getpid(), root_of(&key));
    let (home, root) = ([&at[..], b"/home"].concat(), [&at[..], b"/root"].concat());
    for dir in [HOMES.to_vec(), at, home, root] {
        // Made by an earlier run, or now: either way it must be there after.
        let _ = vfs::mkdir(pid, &dir);
        if vfs::dirstat(pid, &dir).is_err() {
            return false;
        }
    }
    *KEY.0.borrow_mut() = Some(key);
    true
}

/// The home `key` names, and everything in it, gone; true when none is left.
pub fn erase(key: &[u8; 32]) -> bool {
    let (pid, at) = (mk_getpid(), root_of(key));
    let _ = vfs::rmdir(pid, &at, true);
    vfs::dirstat(pid, &at).is_err()
}
