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

//! A kept wallpaper goes to the new disk only whole and as its pin says: one
//! the device changed is left out, and one an install already wrote alone is
//! carried again from its own file.

#[path = "common/fake_vfs.rs"]
mod fake_vfs;
#[path = "common/kept_wallpapers.rs"]
mod kept_wallpapers;

use fake_vfs::FakeVfs;
use kept_wallpapers::{answers, collection, slice, wallpapers};
use nonos_disk::gather;
use nonos_wallpapers::{COLLECTION, PINS};

#[test]
fn a_kept_wallpaper_the_device_changed_is_left_out() {
    let mut all = collection();
    all[PINS[1].offset as usize + 100] ^= 1;
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &answers(2, (1 << 1) | (1 << 2)));
    v.put(COLLECTION, &all);
    let c = gather(&mut v);
    assert_eq!((c.wallpapers.carried, c.wallpapers.left_out), (1, 1));
    let names: Vec<String> = wallpapers(c.store.bytes()).into_iter().map(|(n, _)| n).collect();
    assert_eq!(names, [format!("/Wallpapers/{}.jpg", std::str::from_utf8(PINS[2].slug).unwrap())]);
}

#[test]
fn a_disk_an_install_wrote_carries_its_own_kept_files_again() {
    let all = collection();
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &answers(2, 1 << 2));
    let path = format!("/Wallpapers/{}.jpg", std::str::from_utf8(PINS[2].slug).unwrap());
    v.put(&path, &slice(&all, 2));
    let c = gather(&mut v);
    assert_eq!((c.wallpapers.carried, c.wallpapers.left_out), (1, 0));
    assert_eq!(wallpapers(c.store.bytes()), [(path, slice(&all, 2))]);
}
