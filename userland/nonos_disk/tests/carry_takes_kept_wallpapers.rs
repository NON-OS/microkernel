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

use fake_vfs::FakeVfs;
use nonos_disk::gather;
use nonos_disk_map::{SECTOR_SIZE, STORE_BASE_LBA};
use nonos_policy_proto::route::NYM;
use nonos_policy_proto::setup_record::{Answers, Host, Name, Record, Tier};
use nonos_policy_proto::wallpapers_kept::ALL;
use nonos_wallpapers::{COLLECTION, PINS};

/// The collection as tools/nonos-wallpaper-pack packs it, from the tree.
fn collection() -> Vec<u8> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/wallpapers");
    let list = std::fs::read_to_string(format!("{dir}/catalog.txt")).unwrap();
    let mut out = Vec::new();
    for line in list.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let file = line.split_once(' ').unwrap().1;
        out.extend(std::fs::read(format!("{dir}/{file}")).unwrap());
    }
    out
}

fn answers(desktop: u8, set: u64) -> Vec<u8> {
    let answers = Answers {
        keyboard_layout: 0,
        timezone: 0,
        wallpaper: desktop,
        username: Name::EMPTY,
        qwen_tier: Tier::EMPTY,
    };
    Record { answers, apps_off: 0, hostname: Host::EMPTY, route: NYM, wallpapers_kept: set }
        .encode()
        .to_vec()
}

/// Every entry in a written store, as name and payload.
fn entries(store: &[u8]) -> Vec<(String, Vec<u8>)> {
    let count = u32::from_le_bytes(store[12..16].try_into().unwrap()) as usize;
    let base = STORE_BASE_LBA * SECTOR_SIZE as u64;
    (0..count)
        .map(|i| {
            let e = &store[32 + 128 * i..32 + 128 * (i + 1)];
            let name = String::from_utf8(e[..96].split(|&b| b == 0).next().unwrap().to_vec()).unwrap();
            let at = (u64::from_le_bytes(e[96..104].try_into().unwrap()) - base) as usize;
            let len = u64::from_le_bytes(e[104..112].try_into().unwrap()) as usize;
            (name, store[at..at + len].to_vec())
        })
        .collect()
}

fn wallpapers(store: &[u8]) -> Vec<(String, Vec<u8>)> {
    entries(store).into_iter().filter(|(n, _)| n.starts_with("/Wallpapers/")).collect()
}

fn slice(all: &[u8], i: usize) -> Vec<u8> {
    let p = &PINS[i];
    all[p.offset as usize..(p.offset + p.len) as usize].to_vec()
}

#[test]
fn only_the_kept_wallpapers_go_each_alone() {
    let all = collection();
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &answers(40, (1 << 3) | (1 << 40) | (1 << 62)));
    v.put(COLLECTION, &all);
    let c = gather(&mut v);
    assert_eq!((c.wallpapers.carried, c.wallpapers.left_out), (3, 0));
    let want: Vec<(String, Vec<u8>)> = [3, 40, 62]
        .into_iter()
        .map(|i| (format!("/Wallpapers/{}.jpg", std::str::from_utf8(PINS[i].slug).unwrap()), slice(&all, i)))
        .collect();
    assert!(wallpapers(c.store.bytes()) == want, "exactly the three kept, each whole");
}

#[test]
fn every_wallpaper_kept_goes_as_the_collection() {
    let all = collection();
    for kept_answers in [Some(answers(55, ALL)), None] {
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

#[test]
fn a_kept_wallpaper_the_device_changed_is_left_out() {
    let mut all = collection();
    all[PINS[3].offset as usize + 100] ^= 1;
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &answers(40, (1 << 3) | (1 << 40)));
    v.put(COLLECTION, &all);
    let c = gather(&mut v);
    assert_eq!((c.wallpapers.carried, c.wallpapers.left_out), (1, 1));
    let names: Vec<String> = wallpapers(c.store.bytes()).into_iter().map(|(n, _)| n).collect();
    assert_eq!(names, [format!("/Wallpapers/{}.jpg", std::str::from_utf8(PINS[40].slug).unwrap())]);
}

#[test]
fn a_disk_an_install_wrote_carries_its_own_kept_files_again() {
    let all = collection();
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &answers(40, 1 << 40));
    let path = format!("/Wallpapers/{}.jpg", std::str::from_utf8(PINS[40].slug).unwrap());
    v.put(&path, &slice(&all, 40));
    let c = gather(&mut v);
    assert_eq!((c.wallpapers.carried, c.wallpapers.left_out), (1, 0));
    assert_eq!(wallpapers(c.store.bytes()), [(path, slice(&all, 40))]);
}
