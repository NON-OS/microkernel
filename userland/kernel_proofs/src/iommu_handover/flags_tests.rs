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

//! DMAR's flags and host address width, read (K11): INTR_REMAP is bit 0,
//! X2APIC_OPT_OUT bit 1, and the width is HAW + 1.

use std::sync::Mutex;

use super::dmar_flags::{
    dmar_address_width, dmar_intr_remap, dmar_x2apic_opt_out, record, record_width,
};

/// The flags are one static; the tests that set them take turns.
pub(super) static FLAGS_HELD: Mutex<()> = Mutex::new(());

#[test]
fn each_flag_is_its_own_bit() {
    let _turn = FLAGS_HELD.lock().unwrap_or_else(|e| e.into_inner());
    record(0b0000_0001);
    assert!(dmar_intr_remap() && !dmar_x2apic_opt_out());
    record(0b0000_0010);
    assert!(!dmar_intr_remap() && dmar_x2apic_opt_out());
    record(0b1111_1100);
    assert!(!dmar_intr_remap() && !dmar_x2apic_opt_out(), "no remapping offered");
    record(0);
}

#[test]
fn the_address_width_is_haw_plus_one() {
    let _turn = FLAGS_HELD.lock().unwrap_or_else(|e| e.into_inner());
    record_width(38);
    assert_eq!(dmar_address_width(), Some(39));
    record_width(u8::MAX);
    assert_eq!(dmar_address_width(), Some(u8::MAX), "saturates");
}
