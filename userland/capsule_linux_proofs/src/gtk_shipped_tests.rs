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

//! The environment GTK starts with, for every mix of caches the image
//! carries: the shipped path for each one carried, today's /tmp path for
//! each one built, and nothing carried reads exactly as before.

use crate::gtk_shipped::{env_lines, Shipped, LOADERS_FILE, MIME_FILE, MIME_SHARE, SCHEMAS, SCHEMAS_FILE};

fn s(lines: &[Vec<u8>; 3]) -> [String; 3] {
    lines.clone().map(|l| String::from_utf8(l).expect("ascii"))
}

#[test]
fn nothing_carried_is_the_per_launch_build() {
    assert_eq!(
        s(&env_lines(Shipped::default())),
        [
            "GSETTINGS_SCHEMA_DIR=/tmp".to_string(),
            "GDK_PIXBUF_MODULE_FILE=/tmp/loaders.cache".to_string(),
            "XDG_DATA_DIRS=/tmp/share:/usr/local/share:/usr/share".to_string(),
        ]
    );
}

#[test]
fn each_cache_carried_is_used_where_it_lies() {
    for bits in 0..8u8 {
        let have = Shipped { schemas: bits & 1 != 0, loaders: bits & 2 != 0, mime: bits & 4 != 0 };
        let [schemas, loaders, data] = s(&env_lines(have));
        assert_eq!(schemas.ends_with("/usr/share/nonos-gtk/schemas"), have.schemas, "{have:?}");
        assert_eq!(loaders.ends_with("/usr/share/nonos-gtk/loaders.cache"), have.loaders, "{have:?}");
        assert_eq!(data.starts_with("XDG_DATA_DIRS=/usr/share/nonos-gtk/share:"), have.mime, "{have:?}");
        assert!(data.ends_with(":/usr/local/share:/usr/share"), "{have:?}");
    }
}

#[test]
fn the_files_checked_are_inside_the_paths_pointed_at() {
    assert!(SCHEMAS_FILE.starts_with(SCHEMAS));
    assert!(MIME_FILE.starts_with(MIME_SHARE));
    assert!(LOADERS_FILE.starts_with(b"/usr/share/nonos-gtk/"));
    // The member layout agreed with a work area: everything under usr/share/nonos-gtk.
    for p in [SCHEMAS, SCHEMAS_FILE, LOADERS_FILE, MIME_SHARE, MIME_FILE] {
        assert!(p.starts_with(b"/usr/share/nonos-gtk"));
    }
}
