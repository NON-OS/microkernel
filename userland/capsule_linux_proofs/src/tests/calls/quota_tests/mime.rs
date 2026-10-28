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

//! GTK's mime database, built in the private directories before the
//! program starts: the build fits, and what it leaves behind is small.

use crate::calls::loadavg::declared::PRIVATE_NAMES;

const MIME: &str = include_str!("../../../../../capsule_linux/src/linux/gtk_prep/mime.rs");

/// update-mime-database over shared-mime-info 2.5.1's freedesktop.org.xml,
/// counted on the host: 1051 files and 15 directories under mime/, with
/// /tmp/share beside them, and one name more while a file is renamed.
const PEAK: u64 = 1068;
/// The media its 1038 types fall under, each a directory of a file a type.
const MEDIA: [&str; 13] = [
    "application",
    "audio",
    "chemical",
    "font",
    "image",
    "inode",
    "message",
    "model",
    "multipart",
    "text",
    "video",
    "x-content",
    "x-epoc",
];

/// At 128 the build was refused at its hundred-and-somethingth type and
/// left no name free (KeePassXC: "error creating runtime directory").
#[test]
fn the_build_fits_with_room_for_a_release_with_more_types() {
    const { assert!(PEAK + PEAK / 4 <= PRIVATE_NAMES, "a quarter more types still fit") };
}

#[test]
fn the_source_and_the_files_a_type_go_once_the_build_ends() {
    let at = |needle: &str| MIME.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    let (copied, built, cleared) = (
        at("b\"cp\", b\"-r\", b\"/usr/share/mime\""),
        at("helper(MIME, &[MIME_DIR]);"),
        at("helper(BUSYBOX, &AFTER);"),
    );
    assert!(copied < built && built < cleared);
    assert!(MIME.contains("b\"rm\",\n    b\"-rf\",\n    b\"/tmp/share/mime/packages\","));
    for media in MEDIA {
        assert!(MIME.contains(&format!("    b\"/tmp/share/mime/{media}\",\n")), "{media} stays");
    }
}
