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

//! A station keyed `Ccmp::Hardware` frames plaintext and leaves encryption
//! to the chip's key cache. Such a chip that restarts, or whose key install
//! failed or was never answered, would send that plaintext as it is. So
//! transmit is held, every frame refused, from association until the
//! driver says the chip holds the keys, and again from when it says the
//! chip lost them (a firmware restart, a key cache cleared) until it says
//! they are back. In the clear go only the EAPOL frames a handshake sends
//! before any key is in place. Software protection never waits on the chip.

use super::{Ccmp, LinkStation};

impl LinkStation {
    /// The driver's word on whether the chip holds this link's keys now.
    pub fn chip_keys(&mut self, held: bool) {
        self.chip_keyed = held;
    }

    /// Whether protected transmit waits for the chip's keys.
    pub(super) fn held(&self) -> bool {
        matches!(self.ccmp, Ccmp::Hardware) && !self.chip_keyed
    }
}
