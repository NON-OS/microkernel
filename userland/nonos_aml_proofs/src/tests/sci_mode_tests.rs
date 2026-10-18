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

//! The switch into ACPI mode, bounded, and GBL_RLS written without SLP_EN.

use super::sci_rig::regs;
use crate::events::{
    enter_acpi_mode, parse_fadt, Block, EventsError, Pm1Cnt, GBL_RLS, SCI_EN, SLP_EN,
};
use crate::fadt_enc::{q35, PM1_CNT};

#[test]
fn already_in_acpi_mode_writes_nothing() {
    let f = parse_fadt(&q35(276)).unwrap();
    let mut r = regs();
    r.set(PM1_CNT, SCI_EN);
    assert_eq!(enter_acpi_mode(&f, &mut r, 3000), Ok(false));
    assert!(r.writes.is_empty());
}

#[test]
fn acpi_enable_goes_to_smi_cmd_and_the_wait_ends_when_firmware_answers() {
    let f = parse_fadt(&q35(276)).unwrap();
    let mut r = regs();
    r.smi = Some((0xB2, 0x02, PM1_CNT, 3));
    assert_eq!(enter_acpi_mode(&f, &mut r, 3000), Ok(true));
    assert_eq!(r.writes, [(0xB2, 0x02)]);
    assert_eq!(r.slept_ms, 30);
}

#[test]
fn firmware_that_never_answers_is_an_error_after_the_wait() {
    let f = parse_fadt(&q35(276)).unwrap();
    let mut r = regs();
    assert_eq!(enter_acpi_mode(&f, &mut r, 500), Err(EventsError::ModeTimeout));
    assert!(r.slept_ms <= 500 + 10);
    let mut t = q35(276);
    t[48..52].copy_from_slice(&0u32.to_le_bytes());
    let f = parse_fadt(&t).unwrap();
    assert_eq!(enter_acpi_mode(&f, &mut regs(), 500), Err(EventsError::NoModeSwitch));
}

#[test]
fn releasing_the_lock_sets_gbl_rls_and_never_slp_en() {
    let mut r = regs();
    r.set(PM1_CNT, SCI_EN | SLP_EN | (5 << 10));
    let cnt = Pm1Cnt { a: Block { port: PM1_CNT, len: 2 }, b: None };
    assert!(cnt.release_lock(&mut r));
    assert_eq!(r.get(PM1_CNT), SCI_EN | GBL_RLS | (5 << 10));
}
