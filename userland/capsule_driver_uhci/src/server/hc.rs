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

//! One controller as the host front sees it (server/many).

use super::context::Ctx;
use super::dispatch::dispatch;
use super::many::Hc;
use crate::devices::table::Dev;
use crate::env::{Clock, Io, Mem};
use crate::facts::io::ROOT_PORTS;
use crate::wire::Request;

impl<I: Io, C: Clock, M: Mem> Hc for Ctx<I, C, M> {
    fn ports(&self) -> u8 {
        ROOT_PORTS
    }
    fn serve(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
        dispatch(self, req, body, tx)
    }
    fn hold(&mut self, slot: u8) {
        if let Some(d) = self.table.devs.get_mut(slot as usize - 1) {
            *d = Some(Dev::default());
        }
    }
    fn held(&self, slot: u8) -> u8 {
        let dev = self.table.devs.get(slot as usize - 1).copied().flatten();
        if dev.is_some_and(|d| d.claimed) {
            2
        } else {
            1
        }
    }
    fn release(&mut self, slot: u8) {
        if let Some(d) = self.table.devs.get_mut(slot as usize - 1) {
            *d = None;
        }
    }
}
