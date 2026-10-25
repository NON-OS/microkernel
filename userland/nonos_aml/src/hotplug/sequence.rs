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

//! The firmware half of an eject, in order, once every driver has let go
//! of what it takes: unlock (`_LCK(0)`), undock (`_DCK(0)`), then `_EJ0`.
//! Unlocking first means nothing has changed when it fails. A step that
//! fails after the unlock locks the device again, since it stays in use.
//! A device with no `_EJ0` that `_RMV` calls removable is let go for the
//! user to take out once it is unlocked and undocked.

use super::outcome::{Has, Released, Stopped};

/// The methods, each answering whether it did its part.
pub trait Steps {
    fn lock(&mut self, on: bool) -> bool;
    fn undock(&mut self) -> bool;
    fn eject(&mut self) -> bool;
}

pub fn release(s: &mut dyn Steps, h: Has) -> Result<Released, Stopped> {
    if !h.ej0 && !h.removable {
        return Err(Stopped::NotRemovable);
    }
    if h.lck && !s.lock(false) {
        return Err(Stopped::Unlock);
    }
    let stop = |s: &mut dyn Steps, why: Stopped| -> Result<Released, Stopped> {
        if h.lck {
            let _ = s.lock(true);
        }
        Err(why)
    };
    if h.dock && !s.undock() {
        return stop(s, Stopped::Undock);
    }
    if !h.ej0 {
        return Ok(Released::ForTheUser);
    }
    match s.eject() {
        true => Ok(Released::ByFirmware),
        false => stop(s, Stopped::Eject),
    }
}
