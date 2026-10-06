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

//! A name in the store's table is a path the vfs could have written: one
//! that `normalize` leaves as it is. Every lookup goes through `normalize`,
//! so any other spelling is a file nothing can open that still shows in a
//! listing, and one under `/capsules/` that climbs out with `..` passed the
//! capsule tree's prefix test while naming something outside it.

use super::fixture::{image, install, refs, set_name, three, Layout};
use super::fixture::{without, NAME_LEN};
use super::run::load;

const NOT_NORMAL: [&[u8]; 12] = [
    b"/capsules/../nonos/setup/answers",
    b"/capsules/..",
    b"/../etc",
    b"relative/path",
    b"x",
    b"/a//b",
    b"/a/./b",
    b"/a/",
    b"/",
    b"/a\nb",
    b"/a\x7fb",
    b"/a\tb",
];

#[test]
fn a_name_that_is_not_a_normalized_absolute_path_is_refused_alone() {
    let files = three();
    for bad in NOT_NORMAL {
        let mut img = image(&refs(&files), Layout::Installer);
        set_name(&mut img, 1, bad);
        install(&img);
        let got = load().expect("the rest loads");
        let names: Vec<&str> = got.iter().map(|(n, _)| n.as_str()).collect();
        let want = without(&files, 1);
        let want: Vec<&str> = want.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, want, "{}", String::from_utf8_lossy(bad));
    }
}

#[test]
fn every_name_normalize_produces_loads() {
    let ok = ["/a", "/nonos/setup/answers", "/capsules/x.elf", "/home/My Files/a b.txt"];
    let long = format!("/{}", "a".repeat(NAME_LEN - 1));
    for name in ok.iter().copied().chain([long.as_str()]) {
        assert_eq!(crate::normalize(name).as_deref(), Some(name), "not a normal path: {name}");
        let files = vec![(String::from(name), vec![7u8; 9])];
        install(&image(&refs(&files), Layout::Packer));
        assert_eq!(load(), Ok(files), "{name}");
    }
}
