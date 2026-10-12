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
//! The reader taken and given up: OPEN and CLOSE, and the client that held
//! it gone (run.rs). A card left powered by one holder is not handed to the
//! next with its state (a PIN verified, an application selected): every
//! slot the holder powered is powered off when it lets go.

use nonos_libc::mk_pid_alive;

use crate::card::ask;
use crate::ccid::ICC_POWER_OFF;
use crate::device::Dev;
use crate::wire::E_INVAL;

pub fn open(d: &mut Dev, peer: u32, body: &[u8]) -> Result<(), i32> {
    if !body.is_empty() {
        return Err(E_INVAL);
    }
    let was = d.owner.pid;
    d.owner.open(peer, |p| mk_pid_alive(p))?;
    // A holder that died and was not yet let go leaves its cards powered.
    if was != Some(peer) {
        power_off(d);
    }
    // Changes before this holder are not its news.
    d.slots.forget_changes();
    Ok(())
}

pub fn close(d: &mut Dev) {
    power_off(d);
    d.owner.close();
}

fn power_off(d: &mut Dev) {
    let powered: alloc::vec::Vec<u8> = d.slots.each_powered().collect();
    for slot in powered {
        let _ = ask(d, ICC_POWER_OFF, slot, &[]);
        d.slots.power(slot, false);
    }
}
