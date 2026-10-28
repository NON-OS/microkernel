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

//! gdk-pixbuf knows an image by its mime type, so with no mime database GTK
//! cannot read its own icons and aborts. The whole database is about 930
//! files, past the 128 names a family keeps in its scratch (declared/sizes.rs),
//! so it is built from the types GTK reads: the images gdk-pixbuf loads and
//! plain text, kept from the package's own source with their globs and magic
//! as they are. The result goes where env.rs's XDG_DATA_DIRS looks first.
//! Each step is a tool of its own, as a box without EXEC lets no shell run one.

use super::helper::helper;

pub(super) const TOOL: &[u8] = b"/usr/bin/update-mime-database";
pub(super) const SOURCE: &[u8] = b"/usr/share/mime/packages/freedesktop.org.xml";
const BUSYBOX: &[u8] = b"/bin/busybox";
const DIR: &[u8] = b"/tmp/share/mime";
const PACKAGES: &[u8] = b"/tmp/share/mime/packages";
const WANTED: &[u8] = b"W= image/png image/jpeg image/gif image/bmp image/tiff image/webp \
image/svg+xml image/vnd.microsoft.icon image/x-icns image/x-tga image/x-xbitmap \
image/x-xpixmap image/x-portable-anymap text/plain text/xml application/xml ";
const OUT: &[u8] = b"O=/tmp/share/mime/packages/nonos.xml";
/// Every line outside a type, and the lines of each wanted type, as they are.
const KEEP: &[u8] = br#"/<mime-type type="/ { t = $0; sub(/.*<mime-type type="/, "", t);
sub(/".*/, "", t); keep = index(W, " " t " ") > 0; inside = 1 }
!inside || keep { print > O }
/<\/mime-type>/ { inside = 0 }"#;

pub(super) fn build() {
    helper(BUSYBOX, &[b"mkdir", b"-p", PACKAGES]);
    helper(BUSYBOX, &[b"awk", b"-v", WANTED, b"-v", OUT, KEEP, SOURCE]);
    helper(TOOL, &[DIR]);
}
