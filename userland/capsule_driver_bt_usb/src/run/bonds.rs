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

//! Where bonds are kept. In the keyring when it keeps this capsule's
//! records (lead unit L33); otherwise in this capsule's memory, and gone
//! when it stops. Either way they last for this boot only (Decision 27):
//! nothing here survives a reboot.

use super::desk::Desk;
use super::draw::{draw16, fresh_irk};
use super::env::CapsuleEnv;
use super::kept::Kept;
use super::say::say;
use crate::hid::Keys;
use crate::nbtc::Board;
use crate::ring::connect;
use nonos_bt_bond::{BondError, LocalIrk, MemoryKeys, Store};
use nonos_libc::mk_exit;

/// The capsule's environment with its bonds open, and this host's IRK:
/// the kept one, or one drawn and kept now.
pub fn environment() -> (CapsuleEnv, LocalIrk) {
    let Ok(bonds) = open() else {
        say(&[b"no bond store could be opened; stopping"]);
        mk_exit(4)
    };
    let env = CapsuleEnv { keys: Keys::default(), desk: Desk::new(Board::new(bonds)) };
    let irk = env.desk.board().bonds.local_irk(draw16).unwrap_or_else(|_| fresh_irk());
    (env, irk)
}

fn open() -> Result<Store<Kept>, BondError> {
    say(&[b"bonds last for this boot only (Decision 27)"]);
    if let Some(ring) = connect() {
        if let Ok(store) = Store::open(Kept::Ring(ring)) {
            say(&[b"bonds kept in the keyring"]);
            return Ok(store);
        }
    }
    // A keyring that is not running, or that cannot list this capsule's
    // records, keeps nothing a restart could find: memory does as well.
    say(&[b"the keyring cannot keep bonds; kept in this capsule until it stops"]);
    Store::open(Kept::Memory(MemoryKeys::default()))
}
