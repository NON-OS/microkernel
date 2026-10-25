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

//! GPIO_EVENT (13, verb u8, controller device id u64, pin u16), asked only
//! by the driver holding the controller (the kernel's claim test).
//! - LIST (0): the pins the controller's _AEI lists (`list`), for the
//!   driver to arm each as its descriptor says.
//! - SIGNAL (1, pin): the pin's method runs (`run`). A level pin stays
//!   masked by its driver until the reply.
//!
//! ENOENT for a pin _AEI does not list or a controller with no _AEI;
//! EAGAIN past the controller's rate; EIO when the method fails or there
//! is none; EINVAL for a short request or another verb.

use super::controllers::controller;
use super::list::list;
use super::run::run;
use crate::boot::Acpi;
use crate::server::protocol::{EINVAL, ENOENT};
use crate::sys::function_of::Asker;
use alloc::vec::Vec;

const LIST: u8 = 0;
const SIGNAL: u8 = 1;

pub fn gpio_event(acpi: &mut Acpi, req: &[u8], sender: u32) -> (i32, Vec<u8>) {
    let (Some(&verb), Some(id)) = (req.get(1), req.get(2..10)) else {
        return (EINVAL, Vec::new());
    };
    let id = u64::from_le_bytes(id.try_into().unwrap_or([0; 8]));
    let dev = match crate::dpower::target(acpi, Asker::Holder(sender), id) {
        Ok(d) => d,
        Err(e) => return (e, Vec::new()),
    };
    let Some(i) = controller(acpi, dev) else { return (ENOENT, Vec::new()) };
    let pin = req.get(10..12).map(|p| u16::from_le_bytes([p[0], p[1]]));
    match (verb, pin) {
        (LIST, _) => (0, list(&acpi.aei.controllers[i].pins)),
        (SIGNAL, Some(pin)) => run(acpi, i, dev, pin),
        _ => (EINVAL, Vec::new()),
    }
}
