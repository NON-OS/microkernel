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

//! What Alpine's install triggers would have left for a program that links
//! GTK: the installer runs none, so GSettings finds no compiled schemas (and
//! aborts) and gdk-pixbuf no loader cache. GLib's own tools from the store
//! make both into this run's /tmp, where env.rs points the program, each as
//! a proved, boxed guest of its own that takes no seal note (seal_note.rs).

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use nonos_libc::mk_foreign_spawn;

use super::file::{key, store_found, store_read};
use super::guest::{Guest, Links};
use super::launch::Launch;
use super::say::say;
use super::source::MAX_IMAGE;

const COMPILE: &[u8] = b"/usr/bin/glib-compile-schemas";
const SCHEMAS: &[u8] = b"/usr/share/glib-2.0/schemas";
const LOADERS: &[u8] = b"/usr/bin/gdk-pixbuf-query-loaders";
const LINKS: [&[u8]; 3] = [b"libgtk-3.so", b"libgtk-4.so", b"libgdk_pixbuf-2.0.so"];

static HELPING: AtomicBool = AtomicBool::new(false);

/// Whether the guest starting now is one of these tools (seal_note.rs).
pub fn helping() -> bool {
    HELPING.load(Ordering::Relaxed)
}

/// Both, for a program that links GTK, where the store has the tool.
pub fn prepare(program: &[u8]) {
    if !LINKS.iter().any(|l| program.windows(l.len()).any(|w| w == *l)) {
        return;
    }
    if found(COMPILE) && found(SCHEMAS) {
        helper(COMPILE, &[b"--targetdir=/tmp", SCHEMAS]);
    }
    if found(LOADERS) {
        helper(LOADERS, &[b"--update-cache"]);
    }
}

fn found(path: &[u8]) -> bool {
    store_found(&key(path)).unwrap_or(false)
}

/// One tool, run to its end; one that does not run is no worse than none.
fn helper(path: &[u8], args: &[&[u8]]) {
    let Ok(bytes) = store_read(&key(path), MAX_IMAGE) else { return };
    let Ok(pid) = u32::try_from(mk_foreign_spawn(b"linux")) else { return };
    let mut guest = Guest::new(pid);
    guest.links = alloc::rc::Rc::new(Links::load());
    let args: Vec<Vec<u8>> = args.iter().map(|a| a.to_vec()).collect();
    HELPING.store(true, Ordering::Relaxed);
    let started = super::start_guest::start(&mut guest, Launch::store(path.to_vec(), bytes, args));
    HELPING.store(false, Ordering::Relaxed);
    if started.is_err() || super::serve::serve(guest) != 0 {
        say(b"[LINUX] a GTK preparation tool did not run cleanly\n");
    }
}
