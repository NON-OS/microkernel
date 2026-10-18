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

//! Host stand-in for the capsule's Store: the serial it holds and what it installs.

/// The serial, signature verdict and publisher verdicts of one install.
pub type Installed = (u64, bool, Vec<bool>);

#[derive(Default)]
pub struct Store {
    installs: Vec<Installed>,
}

impl Store {
    pub fn last_serial(&self) -> u64 {
        self.installs.last().map_or(0, |i| i.0)
    }

    pub fn install(&mut self, index: u64, signature_verified: bool, publisher: Vec<bool>) {
        self.installs.push((index, signature_verified, publisher));
    }

    pub fn installs(&self) -> &[Installed] {
        &self.installs
    }
}
