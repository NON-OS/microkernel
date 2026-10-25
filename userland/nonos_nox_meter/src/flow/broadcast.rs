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

//! A job's transactions, broadcast one at a time once signed; each hash
//! answered moves it to the next, and the last ends it with every hash.

use alloc::vec::Vec;

use super::ask::{Ask, Next};
use super::step::Flow;
use super::work::Work;

impl Flow {
    /// The signed transactions, in Sign's order.
    pub fn on_signed(&mut self, raws: Vec<Vec<u8>>) -> Next {
        self.raws = raws;
        self.raws.reverse();
        self.next_broadcast()
    }

    /// A broadcast's hash, or for a Deliver job the key settle read.
    pub fn on_hash(&mut self, h: &[u8; 32]) -> Next {
        if let Work::Deliver(o) = &self.work {
            return Next::Deliver(o.order_id, *h);
        }
        self.hashes.extend_from_slice(h);
        self.next_broadcast()
    }

    fn next_broadcast(&mut self) -> Next {
        match self.raws.pop() {
            Some(raw) => Next::Ask(Ask::Broadcast(raw)),
            None => Next::Done(core::mem::take(&mut self.hashes)),
        }
    }
}
