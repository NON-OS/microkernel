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

//! One bring-up attempt on a framebuffer mapping shorter than the mode.
//!
//! The mode is 1024x768 at 32 bits, 3 MiB. A std-VGA with less video memory
//! has a smaller BAR0 (QEMU `-device VGA,vgamem_mb=1`), and the broker maps
//! exactly the BAR. The clear used to write the mode's 3 MiB whatever the
//! mapping held, so the capsule faulted or wrote over its next mapping.

use nonos_libc::given_back;

use super::adapter::{adapter, enabled, found, DEVICE, GUARD, MODE, MODE_BYTES, REG_LEN};
use super::attempt::run;
use crate::constants::DISPI_LFB_ENABLED;
use crate::error::BgaError;

#[test]
fn a_mode_larger_than_the_framebuffer_mapping_is_refused_before_it_is_set() {
    let mib = 1 << 20;
    let a = adapter(REG_LEN, mib);
    let r = run(found(mib));
    assert_eq!(a.fb.wrote32(mib), GUARD, "the clear ran past the end of the mapping");
    assert!(r.is_err(), "a 3 MiB mode was set on a 1 MiB framebuffer");
    assert_eq!(r.err(), Some(BgaError::ModeTooLarge));
    assert_eq!(enabled(&a), 0, "the refused mode was programmed anyway");
    assert!(given_back().contains(&("device_release", DEVICE)), "the claim was kept");
}

#[test]
fn a_mapping_that_holds_the_mode_exactly_is_set_and_cleared_to_its_end() {
    let a = adapter(REG_LEN, MODE_BYTES);
    a.fb.present32(MODE_BYTES - 4, 0x5EC2_E7ED);
    let d = run(found(MODE_BYTES)).unwrap_or_else(|e| panic!("refused: {e:?}"));
    assert_eq!(a.fb.wrote32(MODE_BYTES - 4), 0, "the last pixel was not wiped");
    assert_eq!(a.fb.wrote32(MODE_BYTES), GUARD, "the clear ran past the end of the mapping");
    assert_ne!(enabled(&a) & DISPI_LFB_ENABLED, 0, "the mode was not enabled");
    let m = d.server.display.mode;
    assert_eq!((m.width, m.height, m.stride), (MODE.0, MODE.1, MODE.0 * 4));
}
