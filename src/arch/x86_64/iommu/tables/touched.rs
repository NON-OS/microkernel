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

//! Publishing each table a range of writes touched, once.

use super::publish::publish;

/// A run of leaf writes lands in a few tables, entry after entry. Flushing a
/// whole page per entry would cost a page of flushes per 4 KiB mapped, so a
/// table is published when the run moves past it, and the last one at the end.
#[derive(Default)]
pub struct Touched {
    current: Option<u64>,
}

impl Touched {
    pub fn note(&mut self, table_phys: u64) {
        match self.current {
            Some(t) if t == table_phys => {}
            Some(t) => {
                publish(t);
                self.current = Some(table_phys);
            }
            None => self.current = Some(table_phys),
        }
    }

    pub fn finish(self) {
        if let Some(t) = self.current {
            publish(t);
        }
    }
}
