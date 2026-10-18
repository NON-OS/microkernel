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

//! A controller without 64-bit addressing is never given an address above
//! 4 GiB, and one with it is served as it is.

use super::super::model::controller;
use crate::constants::HCCPARAMS1;
use crate::dma::{below_4g, DmaPool};
use crate::error::XhciError;

#[test]
fn a_32_bit_controller_is_never_given_an_address_above_4_gib() {
    // The shim maps every grant above 4 GiB, as a kernel may.
    let pool = DmaPool::new(1, 1).with_ac64(false);
    assert_eq!(pool.alloc(4096).err(), Some(XhciError::ControllerUnsupported));
    DmaPool::new(1, 1).alloc(4096).expect("a 64-bit controller takes it");
    assert!(below_4g(0xFFFF_F000, 0x1000));
    assert!(!below_4g(0xFFFF_F000, 0x1001));
    assert!(!below_4g(u64::MAX, 1));
}

#[test]
fn a_controller_reporting_ac64_is_still_read_as_supported() {
    let bar = controller(0);
    bar.present32(HCCPARAMS1 as usize, 0);
    crate::controller::refuse_unsupported(bar.base()).expect("AC64 clear is served");
}
