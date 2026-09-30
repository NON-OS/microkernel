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

/*
 * Bytes through a `net.anon` stream. A send the circuit's window cannot
 * take yet is tried again; a read with nothing waiting asks for what came.
 */

use nonos_libc::mk_idle_ms;

use super::anon::AnonLink;
use super::anon_call::{call, OK, RECV, RX_EMPTY, SEND, WOULD_BLOCK};

impl AnonLink {
    pub fn write_all(&mut self, mut data: &[u8]) -> Result<(), ()> {
        while !data.is_empty() {
            let chunk = &data[..data.len().min(16 * 1024)];
            match call(self.port, SEND, &[&self.id[..], chunk].concat())? {
                (OK, n) if n.len() == 4 => {
                    let sent = u32::from_le_bytes([n[0], n[1], n[2], n[3]]) as usize;
                    data = &data[sent.min(chunk.len())..];
                }
                (WOULD_BLOCK, _) => {
                    mk_idle_ms(20);
                }
                _ => return Err(()),
            };
        }
        Ok(())
    }

    pub fn read(&mut self, into: &mut [u8]) -> Result<usize, ()> {
        if self.pending.is_empty() && !self.closed {
            match call(self.port, RECV, &self.id)? {
                (OK, bytes) => self.pending = bytes,
                (RX_EMPTY, _) => {}
                _ => self.closed = true,
            }
        }
        if self.pending.is_empty() {
            return if self.closed { Err(()) } else { Ok(0) };
        }
        let n = into.len().min(self.pending.len());
        into[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        Ok(n)
    }
}
