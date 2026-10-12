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
//! Looking for a key: first on free ports, then on devices another driver
//! configured and shares; and a key gone, its port tried afresh by the
//! search that found it.

use nonos_usbnet::scan::{Bound, Scanner, SharedScanner};

use crate::bind::{bind, share, Port};

const TAG: &[u8] = b"fido";

pub struct Search {
    own: Scanner,
    shared: SharedScanner,
}

impl Search {
    pub fn new() -> Self {
        Self { own: Scanner::new(), shared: SharedScanner::default() }
    }

    pub fn find(&mut self) -> Option<Bound<Port>> {
        self.own.pass(TAG, bind).or_else(|| self.shared.pass(TAG, share))
    }

    pub fn lost(&mut self, b: &Bound<Port>) {
        if b.nic.shared {
            self.shared.lost(b.xhci, b.port);
        } else {
            self.own.lost(b.xhci, b.port);
        }
    }
}
