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

//! Every name the kernel registers maps back to its own app.

use super::rig::bases;
use crate::instance::{app_index_of, instance_name, INSTANCE_NAME_MAX, INSTANCE_SLOTS};
use crate::shell_apps::LAUNCHER_APPS;

#[test]
fn every_probed_name_maps_back_to_its_own_app() {
    let mut buf = [0u8; INSTANCE_NAME_MAX];
    for (index, app) in LAUNCHER_APPS.iter().enumerate() {
        for slot in 0..=INSTANCE_SLOTS {
            let name = instance_name(app.service, slot, &mut buf).unwrap();
            assert_eq!(app_index_of(bases(), name), Some(index));
        }
    }
}

/// The names the kernel's signed spawn tables register extra windows under,
/// read from the kernel source, so a table that changes its naming fails here.
#[test]
fn the_kernels_instance_names_are_matched_to_their_app() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../src/userspace");
    let mut seen = 0;
    for entry in std::fs::read_dir(root).unwrap() {
        let spawn = entry.unwrap().path().join("spawn.rs");
        let Ok(text) = std::fs::read_to_string(&spawn) else { continue };
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("name: \"app.") else { continue };
            let name = format!("app.{}", rest.trim_end_matches("\",").trim_end_matches('"'));
            let Some((base, slot)) = name.rsplit_once('.') else { continue };
            if !slot.bytes().all(|b| b.is_ascii_digit()) {
                continue;
            }
            assert!(slot.parse::<u32>().unwrap() <= INSTANCE_SLOTS, "{name}");
            if let Some(index) = LAUNCHER_APPS.iter().position(|a| a.service == base.as_bytes()) {
                assert_eq!(app_index_of(bases(), name.as_bytes()), Some(index), "{name}");
                seen += 1;
            }
        }
    }
    assert!(seen > 10, "found only {seen} instance names");
}
