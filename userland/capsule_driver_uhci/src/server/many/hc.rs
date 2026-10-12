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
//! What the front needs of one controller.

use crate::wire::Request;

pub trait Hc {
    /// Its root ports.
    fn ports(&self) -> u8;
    /// One request served as a lone controller would serve it.
    fn serve(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) -> usize;
    /// Slot `slot` held here (enabled, not yet addressed), or let go.
    fn hold(&mut self, slot: u8);
    fn release(&mut self, slot: u8);
}
