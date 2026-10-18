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

//! Which functions are taken, and the request's fixed sizes.

use nonos_libc::{Bar, BAR_KIND_MMIO, BAR_KIND_PIO};

use super::util::record;
use crate::constants::req::{BODY_MAX, HEADER_BYTES};
use crate::discover::is_match;

#[test]
fn only_vmmdev_with_its_port_and_page_is_taken() {
    assert!(is_match(&record()));
    let mut other = record();
    other.device = 0xBEEF;
    assert!(!is_match(&other), "VirtualBox's graphics adapter");
    let mut swapped = record();
    swapped.bars[0] = Bar { kind: BAR_KIND_MMIO, size: 32 };
    assert!(!is_match(&swapped));
    let mut no_page = record();
    no_page.bars[1] = Bar { kind: BAR_KIND_PIO, size: 32 };
    assert!(!is_match(&no_page));
    let mut one_bar = record();
    one_bar.bar_count = 1;
    assert!(!is_match(&one_bar));
}

#[test]
fn the_header_and_the_largest_body_are_vmmdev_h_s() {
    const { assert!(HEADER_BYTES == 24 && BODY_MAX == 16) };
}
