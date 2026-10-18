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

//! A holder's requests about one pin, named by controller `_UID` and pin:
//! WRITE (2, controller u16, pin u16, level u8) drives an output pin its
//! device's GpioIo names; REARM (3, controller u16, pin u16) unmasks a pin
//! masked for storming. Each answers a status word.

use crate::acpi::holds;
use crate::bank::Bank;
use crate::requests::{word, EINVAL, EIO, ENOENT, EPERM};
use nonos_gpio::{OutputError, Owner};

fn bank<'a>(banks: &'a mut [Bank], req: &[u8]) -> Result<(&'a mut Bank, u16), i32> {
    let (Some(uid), Some(pin)) = (word(req, 1), word(req, 3)) else { return Err(EINVAL) };
    let b = banks.iter_mut().find(|b| b.uid == uid).ok_or(ENOENT)?;
    Ok((b, pin))
}

pub fn write(banks: &mut [Bank], req: &[u8], sender: u32) -> i32 {
    let (b, pin) = match bank(banks, req) {
        Ok(x) => x,
        Err(e) => return e,
    };
    let high = req.get(5).is_some_and(|l| *l != 0);
    match b.router.output(&mut b.ctl, |k| holds(sender, k), pin, high) {
        Ok(()) => 0,
        Err(OutputError::NotYours) => EPERM,
        Err(OutputError::Gpio(_)) => EIO,
    }
}

pub fn rearm(banks: &mut [Bank], req: &[u8], sender: u32) -> i32 {
    let (b, pin) = match bank(banks, req) {
        Ok(x) => x,
        Err(e) => return e,
    };
    match b.router.owner(pin) {
        Some(Owner::Device(k)) if holds(sender, k) => {
            b.router.rearm(&mut b.ctl, pin).map_or(EIO, |_| 0)
        }
        Some(_) => EPERM,
        None => ENOENT,
    }
}
