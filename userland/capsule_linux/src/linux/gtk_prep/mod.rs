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

use core::sync::atomic::{AtomicBool, Ordering};

mod helper;

use super::file::{key, store_found};
use helper::helper;

const COMPILE: &[u8] = b"/usr/bin/glib-compile-schemas";
const SCHEMAS: &[u8] = b"/usr/share/glib-2.0/schemas";
const LOADERS: &[u8] = b"/usr/bin/gdk-pixbuf-query-loaders";
const MIME: &[u8] = b"/usr/bin/update-mime-database";
/// gdk-pixbuf knows an image by its mime type, so with no mime database
/// GTK cannot read its own icons and aborts. The database is built where
/// env.rs's XDG_DATA_DIRS looks first, from the packages' own sources. Each
/// step is a tool of its own, as a box without EXEC lets no shell run one.
const MIME_DIR: &[u8] = b"/tmp/share/mime";
const BUSYBOX: &[u8] = b"/bin/busybox";
const LINKS: [&[u8]; 3] = [b"libgtk-3.so", b"libgtk-4.so", b"libgdk_pixbuf-2.0.so"];
/// The same, where the program links them through its own library (Mousepad
/// through libmousepad): a run's store holds only its closure's packs, and
/// every GTK brings gdk-pixbuf's loaders. A directory, as a library's name
/// there is a link the store does not answer for.
const PIXBUF: &[u8] = b"/usr/lib/gdk-pixbuf-2.0";

static HELPING: AtomicBool = AtomicBool::new(false);

/// Whether the guest starting now is one of these tools (seal_note.rs).
pub fn helping() -> bool {
    HELPING.load(Ordering::Relaxed)
}

/// Both, for a program that links GTK, where the store has the tool.
pub fn prepare(program: &[u8]) {
    let links = LINKS.iter().any(|l| program.windows(l.len()).any(|w| w == *l));
    if !links && !found(PIXBUF) {
        return;
    }
    if found(COMPILE) && found(SCHEMAS) {
        helper(COMPILE, &[b"--targetdir=/tmp", SCHEMAS]);
    }
    if found(LOADERS) {
        helper(LOADERS, &[b"--update-cache"]);
    }
    if found(MIME) {
        helper(BUSYBOX, &[b"mkdir", b"-p", b"/tmp/share"]);
        helper(BUSYBOX, &[b"cp", b"-r", b"/usr/share/mime", b"/tmp/share/"]);
        helper(MIME, &[MIME_DIR]);
    }
}

fn found(path: &[u8]) -> bool {
    store_found(&key(path)).unwrap_or(false)
}
