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

//! Consenting to run what this machine installs.

use nonos_libc::{mk_dev_root_confirm, mk_dev_root_local};

/// The challenge is a 32-bit number, so ten digits is every value it
/// can take and one more would be a typo rather than a longer code.
const MAX_DIGITS: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Consent {
    /// Nothing asked for yet.
    Idle,
    /// A code is on the console and these are the digits typed so far.
    Typing(u32, u8),
    Granted,
    Refused,
}

impl Consent {
    /// The code goes to the console, not to the return value.
    pub fn begin() -> Consent {
        match mk_dev_root_local() {
            0 => Consent::Typing(0, 0),
            _ => Consent::Refused,
        }
    }

    pub fn digit(self, d: u32) -> Consent {
        match self {
            Consent::Typing(v, n) if (n as usize) < MAX_DIGITS => {
                Consent::Typing(v.wrapping_mul(10).wrapping_add(d), n + 1)
            }
            other => other,
        }
    }

    pub fn submit(self) -> Consent {
        let Consent::Typing(code, n) = self else { return self };
        if n == 0 {
            return self;
        }
        match mk_dev_root_confirm(code) {
            n if n < 0 => Consent::Refused,
            _ => Consent::Granted,
        }
    }

}
