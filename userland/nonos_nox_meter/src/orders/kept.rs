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


//! ORDER_DELIVERED's bad-chunk answers, kept in memory (v1.4.49): a later
//! ask for the order answers the kept one instead of opening the delivery
//! again. A reboot loses them, and the next ask reopens. An OK needs no
//! entry: the order is then plain and its plaintext is on disk.

use alloc::vec::Vec;

/// Orders whose answer is kept; the oldest goes first.
pub const MAX_KEPT: usize = 8;

#[derive(Default)]
pub struct KeptDelivered {
    rows: Vec<([u8; 32], Vec<u8>)>,
}

impl KeptDelivered {
    pub fn get(&self, order_id: &[u8; 32]) -> Option<&[u8]> {
        self.rows.iter().find(|(id, _)| id == order_id).map(|(_, a)| a.as_slice())
    }

    /// Keep `answer` for the order, in place of any earlier one.
    pub fn keep(&mut self, order_id: [u8; 32], answer: Vec<u8>) {
        self.rows.retain(|(id, _)| *id != order_id);
        if self.rows.len() == MAX_KEPT {
            self.rows.remove(0);
        }
        self.rows.push((order_id, answer));
    }
}
