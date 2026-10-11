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

use nonos_nox_meter::persist::{open, FILE_LEN};
use nonos_payment_proto::EAGAIN;

use super::hold::Meter;
use crate::disk::{self, Loaded, STATE_PATH};
use crate::keccak::Host;

impl Meter {
    /// Read the state file, until the store answers. No file, or one that
    /// does not open under the meter's key, starts from the empty state: its
    /// budget allows nothing, and settle's reports raise each channel's floor
    /// again. A forged file is therefore worth nothing to whoever wrote it.
    pub(super) fn load(&mut self, mac_key: &[u8; 32]) -> Result<(), i32> {
        match disk::load(STATE_PATH, FILE_LEN) {
            Loaded::Silent => return Err(EAGAIN),
            Loaded::Absent => {}
            Loaded::Bytes(b) => {
                if let Ok(s) = open(&Host, mac_key, &b) {
                    self.state = s;
                }
            }
        }
        self.loaded = true;
        Ok(())
    }
}
