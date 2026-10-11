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

//! Who may call which meter op, decided by which registry names the
//! sender's pid owns: the kernel stamps the pid, and the registry is asked
//! forward, name by name, as the keyring's vault gate does. Pure: the capsule
//! supplies `owner_of`.

/// The wallet's endpoints: its first window and the later ones.
pub const WALLET: [&[u8]; 3] = [b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

/// capsule_nox_settle, the only reader of signed vouchers.
pub const SETTLE: [&[u8]; 1] = [b"nox_settle"];

/// Who may ask for a charge: the installer and the app store, for paid
/// installs after the person agreed, and the wallet. Nothing else: Linux
/// guests do not pay through capsule_linux, which hosts untrusted code.
pub const PAYERS: [&[u8]; 5] =
    [b"installer", b"app.store", b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    /// Any sender the kernel named.
    Anyone,
    Wallet,
    Settle,
    Payer,
}

impl Role {
    pub fn names(self) -> &'static [&'static [u8]] {
        match self {
            Role::Anyone => &[],
            Role::Wallet => &WALLET,
            Role::Settle => &SETTLE,
            Role::Payer => &PAYERS,
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
