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

//! The Kali anchor this image pins, checked against Kali's own files: the key
//! file holds exactly the 2025 archive key, and the kali-rolling Release
//! fetched on 2026-09-27 verifies under it, the way an install checks one.

use nonos_openpgp::{dearmor, keys, verify};

use super::service::Service;
use crate::install::deb::release::sums;

const KEY: &[u8] = include_bytes!("../../../capsule_linux/keys/kali/archive-key-2025.asc");
const RELEASE: &[u8] = include_bytes!("../../vectors/kali/Release");
const RELEASE_GPG: &[u8] = include_bytes!("../../vectors/kali/Release.gpg");

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02X}")).collect()
}

#[test]
fn the_pinned_file_is_exactly_the_2025_archive_key() {
    let ring = keys(&dearmor(KEY).expect("armored")).expect("a key");
    let primaries: Vec<String> = ring.iter().map(|k| hex(&k.fingerprint)).collect();
    assert_eq!(primaries[0], "827C8569F2518CC677FECA1AED65462EC8D5E4C5");
    // The key Kali lost access to is not in the anchor.
    assert!(primaries.iter().all(|f| !f.ends_with("ED444FF07D8D0BF6")));
}

#[test]
fn kali_rolling_release_verifies_under_the_pin() {
    let ring = keys(&dearmor(KEY).expect("armored")).expect("a key");
    let sig = dearmor(RELEASE_GPG).unwrap_or_else(|| RELEASE_GPG.to_vec());
    let ok = verify(&Service, &ring, &sig, RELEASE).expect("Kali's Release verifies");
    assert_eq!(hex(&ok.fingerprint), "827C8569F2518CC677FECA1AED65462EC8D5E4C5");
    let mut changed = RELEASE.to_vec();
    changed[10] ^= 1;
    assert!(verify(&Service, &ring, &sig, &changed).is_err(), "a changed Release");
}

#[test]
fn the_release_lists_main_packages_for_amd64() {
    let listed = sums(core::str::from_utf8(RELEASE).expect("text"));
    let paths: Vec<&str> = listed.iter().map(|s| s.path.as_str()).collect();
    assert!(paths.contains(&"main/binary-amd64/Packages.gz"), "{} entries", paths.len());
}
