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

//! What Alpine's install triggers would have left for a GTK program, made
//! before it starts. The installer runs no package's trigger, so GSettings
//! finds no compiled schemas (and aborts) and gdk-pixbuf no loader cache (and
//! draws no icon). GLib's own tools, from the store, make both into this run's
//! private /tmp, where env.rs points the program. Each runs as a guest of its
//! own, proved and boxed like any store program, and gone before the program
//! the person asked for starts.

use alloc::vec::Vec;

use nonos_libc::mk_foreign_spawn;

use super::file::{key, store_found};
use super::guest::{Guest, Links};
use super::launch::Launch;
use super::say::say;
use super::source::MAX_IMAGE;

const COMPILE: &[u8] = b"/usr/bin/glib-compile-schemas";
const SCHEMAS: &[u8] = b"/usr/share/glib-2.0/schemas";
const LOADERS: &[u8] = b"/usr/bin/gdk-pixbuf-query-loaders";

/// Both, where the store has the tool and what it reads.
pub fn prepare() {
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

/// One tool, run to its end. A tool that does not run leaves the program to
/// start without what it makes, which is no worse than before.
fn helper(path: &[u8], args: &[&[u8]]) {
    let Ok(bytes) = super::file::store_read(&key(path), MAX_IMAGE) else { return };
    let pid = mk_foreign_spawn(b"linux");
    if pid < 0 {
        return;
    }
    let mut guest = Guest::new(pid as u32);
    guest.links = alloc::rc::Rc::new(Links::load());
    let args: Vec<Vec<u8>> = args.iter().map(|a| a.to_vec()).collect();
    if super::start_guest::start(&mut guest, Launch::store(path.to_vec(), bytes, args)).is_err() {
        say(b"[LINUX] a GTK preparation tool did not start\n");
        return;
    }
    if super::serve::serve(guest) != 0 {
        say(b"[LINUX] a GTK preparation tool did not finish cleanly\n");
    }
}
