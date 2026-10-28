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

//! gdk-pixbuf knows an image by its mime type, so with no mime database
//! GTK cannot read its own icons and aborts. The database is built where
//! env.rs's XDG_DATA_DIRS looks first, from the packages' own sources. Each
//! step is a tool of its own, as a box without EXEC lets no shell run one.

use super::helper::helper;

const MIME: &[u8] = b"/usr/bin/update-mime-database";
const MIME_DIR: &[u8] = b"/tmp/share/mime";
const BUSYBOX: &[u8] = b"/bin/busybox";
/// The tool writes one XML file a type before its caches, and a write it
/// is refused ends it: 1038 types, 1068 names with the copied source and
/// the caches (shared-mime-info 2.5.1), which a run's 128 names could not
/// hold, so no cache was made and nothing else fitted (KeePassXC). GLib
/// reads mime.cache alone where there is one, and a type's own file only
/// for its description, which falls back to the type's name. So once the
/// build ends, made or not, the source and the files a type go, and a run
/// keeps the caches: fifteen names, not a thousand.
const AFTER: [&[u8]; 16] = [
    b"rm",
    b"-rf",
    b"/tmp/share/mime/packages",
    b"/tmp/share/mime/application",
    b"/tmp/share/mime/audio",
    b"/tmp/share/mime/chemical",
    b"/tmp/share/mime/font",
    b"/tmp/share/mime/image",
    b"/tmp/share/mime/inode",
    b"/tmp/share/mime/message",
    b"/tmp/share/mime/model",
    b"/tmp/share/mime/multipart",
    b"/tmp/share/mime/text",
    b"/tmp/share/mime/video",
    b"/tmp/share/mime/x-content",
    b"/tmp/share/mime/x-epoc",
];

/// The database, where the store has the tool.
pub(super) fn build() {
    if !super::found(MIME) {
        return;
    }
    helper(BUSYBOX, &[b"mkdir", b"-p", b"/tmp/share"]);
    helper(BUSYBOX, &[b"cp", b"-r", b"/usr/share/mime", b"/tmp/share/"]);
    helper(MIME, &[MIME_DIR]);
    helper(BUSYBOX, &AFTER);
}
