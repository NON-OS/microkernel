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

//! The search for a USB class driver that is not a network one, an audio
//! device's: no station address is drawn or handed to its binding, so it
//! needs no entropy, and no Crypto authority.

use super::backoff::Backoff;
use super::book::Book;
use super::bound::Bound;
use super::scanner::Scanner;
use crate::run::ClassBindFn;

impl Scanner {
    pub fn class() -> Self {
        Self { xhci: None, station: None, book: Book::default(), backoff: Backoff::new() }
    }

    /// The device, when a port this pass holds one `bind` takes.
    pub fn pass_class<N>(&mut self, tag: &[u8], bind: ClassBindFn<N>) -> Option<Bound<N>> {
        self.search(tag, bind)
    }
}
