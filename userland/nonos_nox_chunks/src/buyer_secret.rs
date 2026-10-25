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

//! A Buyer's salts as rule 5 keeps them: wiped when it drops, and never
//! shown by Debug.

use super::Buyer;

impl Buyer {
    /// Both salts zeroed, as dropping the Buyer does.
    pub fn wipe(&mut self) {
        self.refund_salt.fill(0);
        self.salt.fill(0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

impl Drop for Buyer {
    fn drop(&mut self) {
        self.wipe();
    }
}

impl core::fmt::Debug for Buyer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Buyer")
            .field("deadline", &self.deadline)
            .field("refund_to", &self.refund_to)
            .field("refund_salt", &"redacted")
            .field("salt", &"redacted")
            .finish()
    }
}
