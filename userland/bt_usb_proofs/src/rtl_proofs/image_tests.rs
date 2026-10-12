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

//! The image downloaded: the patch, the file's version in its last four
//! octets, then the config; a config the chip can do without may be
//! absent, but never other than its pin.

use super::fixture::{chips, pin, VERSION};
use super::fixture_image::{file, store, CFG, CONFIG, FW, PATCH};
use super::rtl::image::image;
use super::rtl::refused::Refused;

#[test]
fn the_patch_then_the_version_then_the_config() {
    let f = file(10);
    let table = chips(pin(FW, &f), Some(pin(CFG, CONFIG)), false);
    let img = image(&table[0], 1, &mut store(&f, Some(CONFIG))).unwrap();
    let want = [&PATCH[..5], &VERSION.to_le_bytes(), CONFIG].concat();
    assert_eq!(img, want);
}

#[test]
fn an_optional_config_may_be_absent_but_not_wrong() {
    let f = file(10);
    let table = chips(pin(FW, &f), Some(pin(CFG, CONFIG)), false);
    let img = image(&table[0], 1, &mut store(&f, None)).unwrap();
    assert_eq!(img, [&PATCH[..5], &VERSION.to_le_bytes()[..]].concat());
    let wrong = image(&table[0], 1, &mut store(&f, Some(&CONFIG[..9])));
    assert_eq!(wrong, Err(Refused::NotPinned(CFG)));
    let needed = chips(pin(FW, &f), Some(pin(CFG, CONFIG)), true);
    assert_eq!(image(&needed[0], 1, &mut store(&f, None)), Err(Refused::Missing(CFG)));
}
