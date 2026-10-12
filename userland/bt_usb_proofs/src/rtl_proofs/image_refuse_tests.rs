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

//! Each file the store gives that is not its pin's, missing where it is
//! needed, or pinned yet for another chip or ROM, is refused by name.

use super::fixture::{chips, pin, Mem};
use super::fixture_image::{file, store, CFG, CONFIG, FW};
use super::rtl::image::image;
use super::rtl::refused::{Layout, Refused};

#[test]
fn a_patch_file_not_its_pin_is_refused_before_the_config_is_read() {
    let f = file(10);
    let table = chips(pin(FW, &f), Some(pin(CFG, CONFIG)), true);
    let mut other = f.clone();
    other[20] ^= 0x80;
    let mut m = store(&other, Some(CONFIG));
    assert_eq!(image(&table[0], 1, 0, &mut m), Err(Refused::NotPinned(FW)));
    assert_eq!(m.asked, [FW]);
    let mut longer = f.clone();
    longer.push(0);
    let refused = Err(Refused::NotPinned(FW));
    assert_eq!(
        image(&table[0], 1, 0, &mut store(&longer, Some(CONFIG))),
        refused,
        "one octet more"
    );
    assert_eq!(image(&table[0], 1, 0, &mut Mem::default()), Err(Refused::Missing(FW)));
}

#[test]
fn a_pinned_file_for_another_project_or_rom_is_refused() {
    let f = file(13);
    let table = chips(pin(FW, &f), None, false);
    let refused = Refused::Layout(FW, Layout::Project);
    assert_eq!(image(&table[0], 1, 0, &mut store(&f, None)), Err(refused));
    let ours = file(10);
    let table = chips(pin(FW, &ours), None, false);
    let refused = Refused::Layout(FW, Layout::NoPatch);
    assert_eq!(image(&table[0], 4, 0, &mut store(&ours, None)), Err(refused));
}
