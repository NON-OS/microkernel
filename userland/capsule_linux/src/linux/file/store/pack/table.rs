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

//! The packs this family reads, each loaded once: its index read and held
//! to the format, the trailer over it verified as this machine's, and its
//! store handle kept open for the run. The open handle is the run's hold on
//! the pack: the store refuses to replace or remove a pack another process
//! holds open, so an update waits until the run is done.

use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::mk_getpid;
use nonos_linux_pack::Index;

use super::load::load;
use super::paths::{PACK, PACKS};

pub struct Loaded {
    pub index: Index<'static>,
    pub stream: VfsStream,
}

struct Table(RefCell<Option<Vec<Loaded>>>);

// SAFETY: the serve loop is the only thread that reads a guest's files.
unsafe impl Sync for Table {}

static TABLE: Table = Table(RefCell::new(None));

/// `f` over every pack that loaded, loading them on first use.
pub fn with<R>(f: impl FnOnce(&mut [Loaded]) -> R) -> R {
    let mut table = TABLE.0.borrow_mut();
    f(table.get_or_insert_with(load_all))
}

/// Drop every pack this process holds; the next read loads them again.
/// Called before a pack is replaced or removed, so this process's own hold
/// is never the one in its way, and after, so it reads what is there now.
/// Each load keeps its index for the process's life: a few kilobytes a pack.
pub fn forget() {
    *TABLE.0.borrow_mut() = None;
}

fn load_all() -> Vec<Loaded> {
    let pid = mk_getpid();
    let names = vfs::list_paths(pid, PACKS).unwrap_or_default();
    let packs = names.iter().map(|n| n.as_bytes()).filter(|n| n.ends_with(PACK));
    packs.filter_map(|path| load(pid, path).map_err(|why| say(path, why)).ok()).collect()
}

fn say(path: &[u8], why: &str) {
    let line = alloc::format!(
        "[LINUX] pack {} not served: {why}\n",
        alloc::string::String::from_utf8_lossy(path)
    );
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
