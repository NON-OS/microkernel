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

//! GTK's three caches as the image ships them: a member a work area builds per
//! image (`nonos-gtk-<16 hex of its tar's BLAKE3>`), under the measurement
//! like every member, with its files under /usr/share/nonos-gtk. Each one
//! the image carries is used where it lies and its per-launch build is
//! skipped; each one it does not carry is built into this run's /tmp, as
//! before.

use alloc::vec::Vec;

pub const SCHEMAS: &[u8] = b"/usr/share/nonos-gtk/schemas";
pub const SCHEMAS_FILE: &[u8] = b"/usr/share/nonos-gtk/schemas/gschemas.compiled";
pub const LOADERS_FILE: &[u8] = b"/usr/share/nonos-gtk/loaders.cache";
pub const MIME_SHARE: &[u8] = b"/usr/share/nonos-gtk/share";
pub const MIME_FILE: &[u8] = b"/usr/share/nonos-gtk/share/mime/mime.cache";

/// Which of the three the image carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Shipped {
    pub schemas: bool,
    pub loaders: bool,
    pub mime: bool,
}

/// The three environment lines that point GTK at its caches: the shipped
/// one where the image has it, this run's /tmp where it is built.
pub fn env_lines(s: Shipped) -> [Vec<u8>; 3] {
    let schemas: &[u8] = if s.schemas { SCHEMAS } else { b"/tmp" };
    let loaders: &[u8] = if s.loaders { LOADERS_FILE } else { b"/tmp/loaders.cache" };
    let share: &[u8] = if s.mime { MIME_SHARE } else { b"/tmp/share" };
    [
        [b"GSETTINGS_SCHEMA_DIR=", schemas].concat(),
        [b"GDK_PIXBUF_MODULE_FILE=", loaders].concat(),
        [b"XDG_DATA_DIRS=", share, b":/usr/local/share:/usr/share"].concat(),
    ]
}
