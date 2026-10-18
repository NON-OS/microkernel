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

//! The OP_ACPI_HID reply on the wire, from the shipping handler: the
//! eight-byte body, the bound declaration's pin and HID_INFO bits on the
//! bound pad only, an empty OK with nothing in reach, the last resort's
//! declared pads each with its own register, and the register setup's
//! probe found the descriptor at.

use nonos_i2cmodel::{Bus, Config, Designware};

use super::acpi::ask;
use super::multi_sku::{bound, FITTED, FITTED_REG};
use crate::bench::{driver, GEMINI_LAKE_HZ};
use crate::regs;
use crate::transaction::{Presence, Reach};

/// The OK status, then the body: address, register, pin, bits, zero.
fn reply(addr: u8, reg: u16, pin: u16, info: u8) -> Vec<u8> {
    let (r, p) = (reg.to_le_bytes(), pin.to_le_bytes());
    vec![0, 0, 0, 0, addr, 0, r[0], r[1], p[0], p[1], info, 0]
}

#[test]
fn each_pad_is_named_in_turn_with_the_bound_pin_and_bits_on_the_bound_pad_only() {
    let (mut d, _a) = bound(Presence::Absent);
    (d.bound_pin, d.bound_info) = (0x0123, 0x05);
    assert_eq!(ask(&d), reply(0x15, 0x0001, 0x0123, 0x05));
    assert_eq!(ask(&d), reply(FITTED, FITTED_REG, 0, 0));
    assert_eq!(ask(&d), reply(0x10, 0x0001, 0, 0));
    assert_eq!(ask(&d), reply(0x15, 0x0001, 0x0123, 0x05), "the turn came round");
}

#[test]
fn with_nothing_in_reach_every_ask_is_an_empty_ok() {
    let _a = regs::attach(Designware::new(Config::LPSS, Bus::default()));
    let d = driver(GEMINI_LAKE_HZ, 0).expect("bring-up");
    for _ in 0..3 {
        assert_eq!(ask(&d), [0, 0, 0, 0]);
    }
}

#[test]
fn the_last_resort_names_each_declared_pad_with_its_own_register_and_no_pin() {
    let _a = regs::attach(Designware::new(Config::LPSS, Bus::default()));
    let d = driver(GEMINI_LAKE_HZ, 0).expect("bring-up");
    d.reach.set(Reach::declared([(0x2C, 0x0020), (0x15, 0x0001)]));
    assert_eq!(ask(&d), reply(0x2C, 0x0020, 0, 0));
    assert_eq!(ask(&d), reply(0x15, 0x0001, 0, 0));
    assert_eq!(ask(&d), reply(0x2C, 0x0020, 0, 0));
}

#[test]
fn a_probe_bound_pad_is_named_with_the_register_the_probe_found() {
    let (mut d, _a) = bound(Presence::HidDescriptor(0x0020));
    (d.bound_pin, d.bound_info) = (7, 1);
    for _ in 0..3 {
        assert_eq!(ask(&d), reply(FITTED, 0x0020, 7, 1));
    }
}
