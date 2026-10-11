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

//! under an IOMMU a virtio device is confined only when
//! modern and offering ACCESS_PLATFORM; anything else virtio is refused.
//! The capability walk is bounded and refuses every hostile list.

use super::build::{space, word};
use super::virtio_rule::{admitted, common_cfg, is_modern_virtio};

const V: u16 = 0x1AF4;

#[test]
fn only_a_modern_virtio_device_with_access_platform_is_confined() {
    assert!(admitted(0x8086, 0x10D3, None), "not virtio");
    assert!(admitted(V, 0x0FFF, None) && admitted(V, 0x1080, None), "outside both ranges");
    assert!(
        is_modern_virtio(V, 0x1040) && !is_modern_virtio(V, 0x103F) && !is_modern_virtio(1, 0x1041)
    );
    for transitional in [0x1000, 0x1005, 0x103F] {
        assert!(!admitted(V, transitional, Some(true)), "{transitional:#x}: legacy path");
    }
    for modern in [0x1040, 0x1041, 0x1050, 0x107F] {
        assert!(admitted(V, modern, Some(true)), "{modern:#x}");
        assert!(!admitted(V, modern, Some(false)), "{modern:#x} without bit 33");
        assert!(!admitted(V, modern, None), "{modern:#x} unread");
    }
}

#[test]
fn the_common_window_is_found_past_other_capabilities() {
    // MSI-X at 0x40, a notify window at 0x50, the common window at 0x60.
    let notify = [0x09, 0x60, 16, 2, 4, 0, 0, 0, 0, 0x30, 0, 0];
    let common = [0x09, 0x00, 16, 1, 4, 0, 0, 0, 0, 0x10, 0, 0];
    let b = space(0x40, &[(0x40, &[0x11, 0x50]), (0x50, &notify), (0x60, &common)]);
    assert_eq!(common_cfg(b.clone(), word(b)), Some((4, 0x1000)));
}

#[test]
fn a_hostile_capability_list_finds_nothing() {
    let none = space(0, &[]);
    assert_eq!(common_cfg(none.clone(), word(none)), None, "no list");
    let looped = space(0x40, &[(0x40, &[0x11, 0x40])]);
    assert_eq!(common_cfg(looped.clone(), word(looped)), None, "a list that loops");
    let low = space(0x20, &[]);
    assert_eq!(common_cfg(low.clone(), word(low)), None, "into the header");
    let bad_bar = space(0x40, &[(0x40, &[0x09, 0, 16, 1, 6, 0, 0, 0, 0, 0, 0, 0])]);
    assert_eq!(common_cfg(bad_bar.clone(), word(bad_bar)), None, "BAR 6");
    let ones = |_off: u8| Some(0xFFu8);
    assert_eq!(common_cfg(ones, word(ones)), None, "a device that does not answer");
    let listed = space(0x40, &[(0x40, &[0x09, 0, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0])]);
    let gone = move |off: u8| if off == 0x06 { Some(0) } else { listed(off) };
    assert_eq!(common_cfg(gone.clone(), word(gone)), None, "capabilities bit clear");
}
