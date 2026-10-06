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

//! Why a record of the wallet did not reach the disk.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unsealed {
    /// The keyring had no machine key to seal under (ENOENT): a machine
    /// with no TPM, or one whose TPM did not answer this boot.
    NoMachineKey,
    /// The keyring refused to seal, or did not answer.
    Keyring,
    /// The store would not take the bytes, or did not answer.
    Store,
    /// The store has no room for them.
    DiskFull,
}

impl Unsealed {
    /// A store refusal's code, as the store's errno.
    pub fn from_store(code: i32) -> Unsealed {
        if code == -28 {
            Unsealed::DiskFull
        } else {
            Unsealed::Store
        }
    }

    /// A keyring seal's refusal code.
    pub fn from_seal(code: i32) -> Unsealed {
        if code == -2 {
            Unsealed::NoMachineKey
        } else {
            Unsealed::Keyring
        }
    }
}
