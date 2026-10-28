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

//! One GTK preparation tool, run to its end as a guest of its own.

use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use nonos_libc::mk_foreign_spawn;

use super::super::file::{key, store_read};
use super::super::guest::{Guest, Links};
use super::super::launch::Launch;
use super::super::say::say;
use super::super::source::MAX_IMAGE;
use super::HELPING;

/// One tool, run to its end; one that does not run is no worse than none.
pub(super) fn helper(path: &[u8], args: &[&[u8]]) {
    let Ok(bytes) = store_read(&key(path), MAX_IMAGE) else { return };
    let Ok(pid) = u32::try_from(mk_foreign_spawn(b"linux")) else { return };
    let mut guest = Guest::new(pid);
    guest.links = alloc::rc::Rc::new(Links::load());
    let args: Vec<Vec<u8>> = args.iter().map(|a| a.to_vec()).collect();
    HELPING.store(true, Ordering::Relaxed);
    let started =
        super::super::start_guest::start(&mut guest, Launch::store(path.to_vec(), bytes, args));
    HELPING.store(false, Ordering::Relaxed);
    if started.is_err() || super::super::serve::serve(guest) != 0 {
        say(b"[LINUX] a GTK preparation tool did not run cleanly\n");
    }
}
