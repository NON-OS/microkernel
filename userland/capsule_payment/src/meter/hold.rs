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

//! The meter's state in memory, and the one way it changes: a new state is
//! sealed and written to disk first, and only then becomes the state.

use alloc::vec::Vec;

use nonos_nox_meter::persist::seal;
use nonos_nox_meter::state::MeterState;
use nonos_payment_proto::{EAGAIN, EINVAL, ENOPERSIST};

use crate::disk::{self, STATE_PATH};
use crate::keccak::Host;
use crate::keys::{self, Keys};

pub struct Meter {
    pub state: MeterState,
    /// Open once the bound root derives and the key is back in the keyring;
    /// until then every op that needs it answers with why.
    pub(super) keys: Option<Keys>,
    /// Whether the file on disk has been read: saving before reading would
    /// overwrite the channels the meter signed on with an empty table.
    pub(super) loaded: bool,
    /// When settle last reported each channel, in uptime milliseconds.
    pub(super) seen: Vec<([u8; 32], u64)>,
}

impl Meter {
    pub fn start() -> Meter {
        let mut m =
            Meter { state: MeterState::default(), keys: None, loaded: false, seen: Vec::new() };
        let _ = m.ready();
        m
    }

    /// The meter's keys, once open and its state read. A boot that keeps
    /// nothing holds no key at all.
    pub fn ready(&mut self) -> Result<Keys, i32> {
        if !disk::kept() {
            return Err(ENOPERSIST);
        }
        if self.keys.is_none() {
            self.keys = Some(keys::open()?);
        }
        let k = self.keys.ok_or(EAGAIN)?;
        if !self.loaded {
            self.load(&k.mac_key)?;
            self.refresh_all();
        }
        Ok(k)
    }

    /// Make `next` the state, once it is on disk.
    pub fn commit(&mut self, next: MeterState) -> Result<(), i32> {
        let k = self.ready()?;
        let file = seal(&Host, &k.mac_key, &next).ok_or(EINVAL)?;
        disk::save(STATE_PATH, &file)?;
        self.state = next;
        Ok(())
    }
}
