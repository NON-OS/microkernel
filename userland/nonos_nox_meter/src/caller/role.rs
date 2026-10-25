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

//! Who may call which meter op (plan/21), decided by which registry names the
//! sender's pid owns: the kernel stamps the pid, and the registry is asked
//! forward, name by name, as the keyring's vault gate does. Pure: the capsule
//! supplies `owner_of`.

use super::names::{BUYERS, DELIVERERS, INSTALL, PAYERS, SETTLE, STORE, WALLET};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    /// Any sender the kernel named.
    Anyone,
    Wallet,
    Settle,
    Payer,
    Buyer,
    Deliverer,
    Install,
    Store,
}

impl Role {
    pub fn names(self) -> &'static [&'static [u8]] {
        match self {
            Role::Anyone => &[],
            Role::Wallet => &WALLET,
            Role::Settle => &SETTLE,
            Role::Payer => &PAYERS,
            Role::Buyer => &BUYERS,
            Role::Deliverer => &DELIVERERS,
            Role::Install => &INSTALL,
            Role::Store => &STORE,
        }
    }

    /// Whether `sender` (the kernel-stamped pid) plays this role, given the
    /// pid that owns each name. Pid zero is no sender at all.
    pub fn admits(self, sender: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> bool {
        if sender == 0 {
            return false;
        }
        self == Role::Anyone || self.names().iter().any(|n| owner_of(n) == Some(sender))
    }
}
