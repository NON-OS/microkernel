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

//! Modern whenever the device presents it (plan/50 C3): only the modern
//! transport can take ACCESS_PLATFORM, so a transitional function, legacy
//! I/O window and all, is driven modern; the legacy registers are left for
//! a device with nothing else. Presenting it means every structure the
//! modern bring-up maps: common, notify and device configuration alike.

use nonos_virtio::VirtioError;

use super::space::{transitional, Space, BARS};
use crate::transport::choose::modern_caps;

const NOTIFY: u8 = 2;
const DEVICE: u8 = 4;

#[test]
fn a_transitional_function_that_presents_the_modern_structures_is_driven_modern() {
    let caps = modern_caps(&BARS, &mut transitional(true)).unwrap().expect("modern");
    assert_eq!(caps.common.map(|r| (r.bar, r.offset)), Some((4, 0)));
    assert_eq!(caps.notify.map(|r| (r.bar, r.offset)), Some((4, 0x3000)));
    assert_eq!(caps.device.map(|r| (r.bar, r.offset)), Some((4, 0x2000)));
    assert_eq!(caps.notify_multiplier, 4);
}

#[test]
fn a_function_whose_notify_or_device_cannot_be_mapped_keeps_the_legacy_registers() {
    for cfg_type in [NOTIFY, DEVICE] {
        let mut space = transitional(true).in_io_bar(cfg_type);
        assert_eq!(modern_caps(&BARS, &mut space), Ok(None), "cfg_type {cfg_type}");
    }
    let mut neither = transitional(true).in_io_bar(NOTIFY).in_io_bar(DEVICE);
    assert_eq!(modern_caps(&BARS, &mut neither), Ok(None));
}

#[test]
fn only_a_function_with_nothing_else_keeps_the_legacy_registers() {
    assert_eq!(modern_caps(&BARS, &mut transitional(false)), Ok(None));
    assert_eq!(modern_caps(&BARS, &mut transitional(true).in_io_bar(1)), Ok(None));
}

#[test]
fn a_config_space_that_cannot_be_read_stops_the_attempt() {
    let refused = Err(VirtioError::ConfigRead.message());
    assert_eq!(modern_caps(&BARS, &mut Space(None)).map(|c| c.is_some()), refused);
}

#[test]
fn a_modern_only_function_missing_a_structure_is_refused_not_driven_legacy() {
    let modern_only = |mut space: Space| {
        space.0.as_mut().expect("readable")[2..4].copy_from_slice(&0x1041u16.to_le_bytes());
        space
    };
    for cfg_type in [1, NOTIFY, DEVICE] {
        let mut space = modern_only(transitional(true).in_io_bar(cfg_type));
        assert!(modern_caps(&BARS, &mut space).is_err(), "cfg_type {cfg_type}");
    }
    assert!(modern_caps(&BARS, &mut modern_only(transitional(false))).is_err());
    let whole = modern_caps(&BARS, &mut modern_only(transitional(true)));
    assert!(matches!(whole, Ok(Some(_))), "a whole modern-only function is driven");
}
