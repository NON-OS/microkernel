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

//! The wallpapers kept at setup go to the new disk, and only those: the
//! whole collection when every one was kept, each kept one alone otherwise,
//! cut out of the running system's collection and held to its pin first.
//! The store is read back from its own table.

#[path = "common/fake_vfs.rs"]
mod fake_vfs;
#[path = "common/kept_wallpapers.rs"]
mod kept_wallpapers;

use fake_vfs::FakeVfs;
use kept_wallpapers::{answers, collection, slice, wallpapers};
use nonos_disk::gather;
use nonos_policy_proto::wallpapers_kept::ALL;
use nonos_wallpapers::{COLLECTION, PINS};

#[test]
fn only_the_kept_wallpapers_go_each_alone() {
    let all = collection();
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &answers(2, (1 << 1) | (1 << 2) | (1 << 3)));
    v.put(COLLECTION, &all);
    let c = gather(&mut v);
    assert_eq!((c.wallpapers.carried, c.wallpapers.left_out), (3, 0));
    let want: Vec<(String, Vec<u8>)> = [1, 2, 3]
        .into_iter()
        .map(|i| {
            (
                format!("/Wallpapers/{}.jpg", std::str::from_utf8(PINS[i].slug).unwrap()),
                slice(&all, i),
            )
        })
        .collect();
    assert!(wallpapers(c.store.bytes()) == want, "exactly the three kept, each whole");
}

#[test]
fn every_wallpaper_kept_goes_as_the_collection() {
    let all = collection();
    for kept_answers in [Some(answers(0, ALL)), None] {
        let mut v = FakeVfs::default();
        if let Some(a) = &kept_answers {
            v.put("/nonos/setup/answers", a);
        }
        v.put(COLLECTION, &all);
        let c = gather(&mut v);
        assert_eq!(c.wallpapers.carried, PINS.len());
        let got = wallpapers(c.store.bytes());
        assert_eq!(got.len(), 1);
        assert!(got[0].0 == COLLECTION && got[0].1 == all, "the collection whole");
    }
}
