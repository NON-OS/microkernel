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

//! One GTK preparation tool, run to its end as a guest of its own. A tool
//! the store does not hold but the built-in BusyBox serves runs as that.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use nonos_libc::mk_foreign_spawn;

use super::super::built_in;
use super::super::file::{key, store_read};
use super::super::guest::{Guest, Links};
use super::super::launch::Launch;
use super::super::say::say;
use super::super::source::MAX_IMAGE;
use super::HELPING;

/// One tool, run to its end; one that does not run is no worse than none.
pub(super) fn helper(path: &[u8], args: &[&[u8]]) {
    let launch = match store_read(&key(path), MAX_IMAGE) {
        Ok(bytes) => Launch::store(path.to_vec(), bytes, Vec::new()),
        Err(_) if built_in::serves(path).is_some() => built_in::built_in(),
        Err(_) => return,
    };
    let Ok(pid) = u32::try_from(mk_foreign_spawn(b"linux")) else { return };
    let mut guest = Guest::new(pid);
    guest.links = alloc::rc::Rc::new(Links::load());
    let launch = Launch { args: args.iter().map(|a| a.to_vec()).collect(), ..launch };
    crate::linux::start_say::clock::helper();
    HELPING.store(true, Ordering::Relaxed);
    let started = super::super::start_guest::start(&mut guest, launch);
    HELPING.store(false, Ordering::Relaxed);
    let code = match started {
        Ok(()) => super::super::serve::serve(guest),
        Err(_) => -1,
    };
    if code != 0 {
        let tool = String::from_utf8_lossy(path);
        let first = String::from_utf8_lossy(args.first().copied().unwrap_or_default());
        say(format!("[LINUX] GTK preparation {tool} {first} ended {code}\n").as_bytes());
    }
}
