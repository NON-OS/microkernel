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

//! C10d: the image's own files, as the build lists them from the committed
//! catalogue, and the members that would take their place.

use crate::install::image_list::image_files::{image_files, source, write};
use crate::install::member::image::shadows;

const STORE: &str = include_str!("../../../../../tools/nix/store.json");

#[test]
fn the_catalogue_lists_the_image_s_files_sorted_from_the_root() {
    let files = image_files(STORE).expect("the committed catalogue parses");
    for f in ["/etc/ssl/cert.pem", "/etc/passwd", "/bin/qwenchat", "/usr/bin/python3"] {
        assert!(files.binary_search(&f.to_string()).is_ok(), "{f}");
    }
    assert!(files.windows(2).all(|w| w[0] < w[1]), "sorted, once each");
    assert!(files.iter().all(|f| f.starts_with('/') && !f.starts_with("/linux/")));
    let to = std::env::temp_dir().join(format!("image_files_{}.rs", std::process::id()));
    write(&to).expect("the build's own write");
    let written = std::fs::read_to_string(&to).unwrap();
    let _ = std::fs::remove_file(&to);
    assert_eq!(written, source(&files));
}

#[test]
fn a_member_at_an_image_file_or_a_link_above_one_is_refused() {
    let files = image_files(STORE).expect("the committed catalogue parses");
    let image: Vec<&[u8]> = files.iter().map(|f| f.as_bytes()).collect();
    let at = |p: &str, link: bool| shadows(p.as_bytes(), link, &image);
    for f in ["/etc/ssl/cert.pem", "/etc/passwd", "/bin/qwenchat"] {
        assert!(at(f, false) && at(f, true), "{f}");
    }
    for dir in ["/etc/ssl", "/etc", "/bin", "/usr/lib/perl5", "/"] {
        assert!(at(dir, true), "a link at {dir}");
        assert!(!at(dir, false), "a file at {dir} replaces nothing");
    }
    for f in ["/etc/ssl/certs/x.pem", "/etc/passwd-", "/etc/ss", "/usr/bin/jq2"] {
        assert!(!at(f, false) && !at(f, true), "{f}");
    }
}

#[test]
fn a_catalogue_the_build_cannot_trust_fails_it() {
    let one = |p: &str| format!("{{\"linux\": [{{\"path\": \"{p}\", \"file\": \"f\"}}]}}");
    assert_eq!(image_files(&one("/linux/etc/passwd")).unwrap(), ["/etc/passwd"]);
    for bad in [
        "{}".to_string(),
        format!("{} {}", one("/linux/a"), one("/linux/b")),
        "{\"linux\": []}".to_string(),
        one("/etc/passwd"),
        one("/linux/a b"),
        one("/linux/a\\\\b"),
        "{\"linux\": [{\"path\": 1}]}".to_string(),
    ] {
        assert!(image_files(&bad).is_err(), "{bad}");
    }
}
