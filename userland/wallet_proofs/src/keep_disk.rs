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

//! A disk for the record-keeping tests that logs each write and fails one.

use crate::keep_plan::Records;
use crate::wallet::vault::Unsealed;

/// A disk that records each write and fails the one named.
pub(super) struct Disk {
    pub(super) log: Vec<&'static str>,
    pub(super) fail: Option<&'static str>,
    pub(super) seal: Unsealed,
}

impl Disk {
    pub(super) fn failing(at: Option<&'static str>) -> Disk {
        Disk { log: Vec::new(), fail: at, seal: Unsealed::Store }
    }

    fn write(&mut self, what: &'static str) -> Result<(), Unsealed> {
        self.log.push(what);
        if self.fail == Some(what) {
            Err(self.seal)
        } else {
            Ok(())
        }
    }
}

impl Records for Disk {
    fn clear_vault(&mut self) -> Result<(), Unsealed> {
        self.write("clear vault")
    }
    fn kind(&mut self, _from_key: bool) -> Result<(), Unsealed> {
        self.write("kind")
    }
    fn words(&mut self) -> Result<(), Unsealed> {
        self.write("words")
    }
    fn clear_words(&mut self) -> Result<(), Unsealed> {
        self.write("clear words")
    }
    fn accounts(&mut self) -> Result<(), Unsealed> {
        self.write("accounts")
    }
    fn vault(&mut self) -> Result<(), Unsealed> {
        self.write("vault")
    }
}
