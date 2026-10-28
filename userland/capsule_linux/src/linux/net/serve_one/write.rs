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

//! WRITE: the bytes for the guest put in its end of the door, as many as it
//! has room for, so its buffer bounds what the capsule writes.

use alloc::vec::Vec;

use nonos_serve_wire::{took, CLOSED, OK};

use super::conn::Conn;
use crate::linux::net::sock::Socks;

impl Conn {
    /// WRITE `seq`: how many of `bytes` the guest's end took, 0 when it is
    /// full; CLOSED once the guest's end is gone.
    pub fn write(&mut self, t: &mut Socks, seq: u32, bytes: &[u8]) -> (u8, Vec<u8>) {
        if let Some((_, n)) = self.wrote.filter(|w| w.0 == seq) {
            return (OK, took(n).to_vec());
        }
        match t.door_put(self.end, bytes) {
            Ok(n) => {
                let n = n as u32;
                self.wrote = Some((seq, n));
                (OK, took(n).to_vec())
            }
            Err(_) => (CLOSED, Vec::new()),
        }
    }
}
