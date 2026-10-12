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

//! Every driver the kernel spawns has a row, or claims nothing.

use super::scope::{row, Match};

/// Drivers that reach their devices through another driver and never call
/// MkDeviceClaim: they have no row, and must not gain one unnoticed.
const CLAIMS_NOTHING: [&str; 3] = ["driver.usb_hid0", "driver.usb_msc0", "driver.i2c_hid0"];

fn spawned_drivers() -> Vec<String> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let mut found = Vec::new();
    let mut stack = vec![root.join("hardware"), root.join("userspace")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("readable").flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.file_name().is_some_and(|n| n == "spawn.rs") {
                let text = std::fs::read_to_string(&path).expect("readable");
                if let Some(rest) = text.split("const SERVICE_NAME: &str = \"").nth(1) {
                    let name = rest.split('"').next().expect("closed");
                    if name.starts_with("driver.") {
                        found.push(name.to_string());
                    }
                }
            }
        }
    }
    found
}

#[test]
fn every_driver_the_kernel_spawns_has_a_row_or_claims_nothing() {
    let drivers = spawned_drivers();
    assert!(drivers.len() >= 18, "found only {drivers:?}");
    for d in &drivers {
        let named = CLAIMS_NOTHING.contains(&d.as_str());
        assert!(row(d).is_some() != named, "{d}: a row, or named as claiming nothing");
    }
    for d in CLAIMS_NOTHING {
        assert!(drivers.iter().any(|s| s == d), "{d} is no longer spawned");
    }
}

#[test]
fn every_id_range_is_a_pair_low_to_high() {
    for d in spawned_drivers() {
        for m in row(&d).unwrap_or(&[]) {
            if let Match::Ids(_, ids) = m {
                assert!(!ids.is_empty() && ids.len() % 2 == 0, "{d}: a range without its end");
                assert!(ids.chunks(2).all(|r| r[0] <= r[1]), "{d}: a range high to low");
            }
        }
    }
}
