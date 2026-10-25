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

//! GPIO_EVENT's SIGNAL: one pin's method run, within the controller's
//! rate. The body is what ran, u8: 1 _Exx, 2 _Lxx, 3 _EVT.

use crate::boot::{evaluator, Acpi};
use crate::server::protocol::{EAGAIN, EIO, ENOENT};
use crate::sys::clock::now_ms;
use alloc::vec::Vec;
use nonos_aml::aei::{signal, AeiError, Ran};
use nonos_aml::namespace::Id;

pub fn run(acpi: &mut Acpi, i: usize, dev: Id, pin: u16) -> (i32, Vec<u8>) {
    if !acpi.aei.controllers[i].rate.admit(now_ms()) {
        return (EAGAIN, Vec::new());
    }
    let pins = acpi.aei.controllers[i].pins.clone();
    match signal(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev, &pins, pin) {
        Ok(Ran::Edge) => (0, [1].to_vec()),
        Ok(Ran::Level) => (0, [2].to_vec()),
        Ok(Ran::Evt) => (0, [3].to_vec()),
        Err(AeiError::NotListed) => (ENOENT, Vec::new()),
        Err(AeiError::NoMethod | AeiError::Fault(_)) => (EIO, Vec::new()),
    }
}
