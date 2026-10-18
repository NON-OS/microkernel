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

//! The RADIO control op (E-72): turn the radio off or on. Off freezes every
//! transmit path — join, data plane and the between-request upkeep — so
//! nothing leaves the air; a held session stays inert and resumes when on.

use super::control::code;
use super::station::SoftStation;
use crate::softmac::radio::{Env, Radio};

impl<R: Radio, E: Env> SoftStation<R, E> {
    /// Body octet 0 turns the radio off, 1 turns it on; anything else is a
    /// bad request. Whether it is on now is read back by the status op.
    pub(super) fn set_radio(&mut self, body: &[u8], out: &mut [u8]) -> usize {
        match body.first() {
            Some(0) => self.off = true,
            Some(1) => self.off = false,
            _ => return code(-1, out),
        }
        code(0, out)
    }
}
