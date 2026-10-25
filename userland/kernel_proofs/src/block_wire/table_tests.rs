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


//! A driver serving the block wire is asked for its disks only once its
//! mirror's endpoint is in the block layer's table, under its own feature,
//! with the reply inbox its spawn registers and its capsule's manifest
//! answers on. A mirror without that row serves disks the kernel never sees.

const TABLE: &str = include_str!("../../../../src/hardware/block_device/wire/table.rs");

macro_rules! row {
    ($mirror:literal, $capsule:literal, $feature:literal) => {
        (
            $mirror,
            $feature,
            include_str!(concat!("../../../../src/hardware/", $mirror, "/endpoint.rs")),
            include_str!(concat!("../../../../src/hardware/", $mirror, "/spawn.rs")),
            include_str!(concat!("../../../", $capsule, "/Capsule.mk")),
        )
    };
}

const ROWS: [(&str, &str, &str, &str, &str); 3] = [
    row!("sdhci_capsule", "capsule_driver_sdhci", "nonos-capsule-driver-sdhci"),
    row!("ufs_capsule", "capsule_driver_ufs", "nonos-capsule-driver-ufs"),
    row!("storvsc_capsule", "capsule_driver_storvsc", "nonos-capsule-driver-storvsc"),
];

fn quoted<'a>(src: &'a str, after: &str) -> &'a str {
    let at = src.find(after).unwrap_or_else(|| panic!("no {after:?}")) + after.len();
    let start = at + src[at..].find('"').expect("no string") + 1;
    &src[start..start + src[start..].find('"').expect("unclosed string")]
}

#[test]
fn every_block_wire_mirror_is_in_the_table_under_its_feature() {
    for (mirror, feature, ..) in ROWS {
        let entry = format!(
            "#[cfg(feature = \"{feature}\")]\n    &crate::hardware::{mirror}::ENDPOINT,"
        );
        assert!(TABLE.contains(&entry), "{mirror} missing from the table");
    }
}

#[test]
fn each_endpoint_replies_where_its_spawn_and_manifest_say() {
    for (mirror, _, endpoint, spawn, manifest) in ROWS {
        let inbox = quoted(endpoint, "const REPLY_INBOX: &str =");
        assert!(spawn.contains("use super::endpoint::REPLY_INBOX;"), "{mirror}'s spawn");
        assert!(!spawn.contains("const REPLY_INBOX"), "{mirror}'s spawn keeps its own inbox");
        let line = manifest.lines().find(|l| l.starts_with("CAPSULE_REPLY_ENDPOINT")).unwrap();
        assert!(line.ends_with(&format!(":{inbox}")), "{mirror}: {inbox} against {line}");
        let service = quoted(spawn, "const SERVICE_NAME: &str =");
        assert!(quoted(endpoint, "shown:").ends_with(&format!("({service})")), "{mirror}");
    }
}
