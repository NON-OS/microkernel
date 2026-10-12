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

//! Where the next input field of each report starts, in bits after its
//! report ID byte: every report ID numbers its own report (HID 1.11,
//! 6.2.2.7), so each keeps its own count, for at most MAX_REPORTS of them.

use super::limits::{Refused, MAX_REPORTS, MAX_REPORT_BITS};

#[derive(Default)]
pub struct Cursors {
    ids: [u8; MAX_REPORTS],
    bits: [u32; MAX_REPORTS],
    n: usize,
}

impl Cursors {
    /// Take `bits` more of report `id`, returning where they start. Err for
    /// a report past MAX_REPORT_BITS or one report ID too many.
    pub fn claim(&mut self, id: u8, bits: u32) -> Result<u32, Refused> {
        let i = match self.ids[..self.n].iter().position(|&r| r == id) {
            Some(i) => i,
            None if self.n == MAX_REPORTS => return Err(Refused),
            None => {
                self.ids[self.n] = id;
                self.n += 1;
                self.n - 1
            }
        };
        let at = self.bits[i];
        self.bits[i] = at.checked_add(bits).filter(|&end| end <= MAX_REPORT_BITS).ok_or(Refused)?;
        Ok(at)
    }

    /// Whether input fields were declared before any Report ID item: with
    /// report IDs in use, such a report could not be told apart.
    pub fn unnumbered(&self) -> bool {
        self.ids[..self.n].iter().zip(&self.bits).any(|(&id, &bits)| id == 0 && bits > 0)
    }

    /// The longest input report in bytes, with its report ID byte when
    /// `ids`.
    pub fn longest(&self, ids: bool) -> u16 {
        let bytes = self.bits[..self.n].iter().map(|&b| b.div_ceil(8) as u16 + u16::from(ids));
        bytes.max().unwrap_or(0)
    }
}
