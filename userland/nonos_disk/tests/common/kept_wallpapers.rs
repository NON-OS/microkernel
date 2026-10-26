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

//! What the kept-wallpaper carry tests build on, shared by path.

use nonos_disk_map::{SECTOR_SIZE, STORE_BASE_LBA};
use nonos_policy_proto::route::NYM;
use nonos_policy_proto::setup_record::{Answers, Host, Name, Record, Tier};
use nonos_wallpapers::PINS;

/// The collection as tools/nonos-wallpaper-pack packs it, from the tree.
pub fn collection() -> Vec<u8> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/wallpapers");
    let list = std::fs::read_to_string(format!("{dir}/catalog.txt")).unwrap();
    let mut out = Vec::new();
    for line in list.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let file = line.split_once(' ').unwrap().1;
        out.extend(std::fs::read(format!("{dir}/{file}")).unwrap());
    }
    out
}

pub fn answers(desktop: u8, set: u64) -> Vec<u8> {
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
pub fn entries(store: &[u8]) -> Vec<(String, Vec<u8>)> {
    let count = u32::from_le_bytes(store[12..16].try_into().unwrap()) as usize;
    let base = STORE_BASE_LBA * SECTOR_SIZE as u64;
    (0..count)
        .map(|i| {
            let e = &store[32 + 128 * i..32 + 128 * (i + 1)];
            let name =
                String::from_utf8(e[..96].split(|&b| b == 0).next().unwrap().to_vec()).unwrap();
            let at = (u64::from_le_bytes(e[96..104].try_into().unwrap()) - base) as usize;
            let len = u64::from_le_bytes(e[104..112].try_into().unwrap()) as usize;
            (name, store[at..at + len].to_vec())
        })
        .collect()
}

pub fn wallpapers(store: &[u8]) -> Vec<(String, Vec<u8>)> {
    entries(store).into_iter().filter(|(n, _)| n.starts_with("/Wallpapers/")).collect()
}

pub fn slice(all: &[u8], i: usize) -> Vec<u8> {
    let p = &PINS[i];
    all[p.offset as usize..(p.offset + p.len) as usize].to_vec()
}
