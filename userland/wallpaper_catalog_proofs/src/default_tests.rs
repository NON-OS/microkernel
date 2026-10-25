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

//! The wallpaper that stands until someone picks another is one wallpaper in
//! every place that names it: the labels Settings and setup list have one
//! name per pinned picture, the default is among them, and the picture the
//! wallpaper service links in for its first frame is the default's, byte for
//! byte, so the desktop does not change picture when the catalog answers.

use nonos_hash::sha256;
use nonos_wallpapers::PINS;

use super::wallpaper_embedded::EMBEDDED;
use super::wallpaper_labels::{CATALOG_ERA, WALLPAPER_DEFAULT, WALLPAPER_LABELS};
use crate::collection::files;

#[test]
fn every_label_names_one_pinned_picture() {
    assert_eq!(WALLPAPER_LABELS.len(), PINS.len());
    assert_eq!(files().len(), PINS.len(), "the catalog lists what is pinned");
    assert!(WALLPAPER_LABELS.iter().all(|l| !l.is_empty() && l.len() <= 24));
}

#[test]
fn the_first_frame_is_the_default_picture() {
    let pin = &PINS[WALLPAPER_DEFAULT as usize];
    assert_eq!(EMBEDDED.len(), pin.len as usize);
    assert_eq!(sha256(EMBEDDED), pin.sha256);
    assert_eq!(files()[WALLPAPER_DEFAULT as usize].1, EMBEDDED);
}

#[test]
fn the_catalog_names_the_era_its_indices_belong_to() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/wallpapers/catalog.txt");
    let catalog = std::fs::read_to_string(path).unwrap();
    assert!(
        catalog.contains(&format!("# Catalog era {CATALOG_ERA}.")),
        "catalog.txt and CATALOG_ERA agree"
    );
}
