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

//! The status line a keep that did not land leaves.

use super::keep_plan::Kept;
use crate::wallet::vault::Unsealed;

/// The status line for a keep that did not land, or None when it did.
pub fn kept_status(kept: Kept, from_key: bool) -> Option<&'static [u8]> {
    let s: &'static [u8] = match (kept, from_key) {
        (Kept::Kept, _) => return None,
        (Kept::Live, false) => b"this is a live session: nothing is kept past power off, so this wallet is gone then; write down the phrase",
        (Kept::Live, true) => b"this is a live session: nothing is kept past power off, so this wallet is gone then; keep the private key",
        (Kept::LastStays(Unsealed::DiskFull), false) => b"the disk is full, so this wallet is gone at reboot and any wallet stored before stays: write down the phrase",
        (Kept::LastStays(Unsealed::DiskFull), true) => b"the disk is full, so this wallet is gone at reboot and any wallet stored before stays: keep the private key",
        (Kept::LastStays(_), false) => b"the store would not take this wallet, so it is gone at reboot and any wallet stored before stays: write down the phrase",
        (Kept::LastStays(_), true) => b"the store would not take this wallet, so it is gone at reboot and any wallet stored before stays: keep the private key",
        (Kept::Lost(Unsealed::NoMachineKey), false) => b"no machine key to seal under, so this wallet is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::NoMachineKey), true) => b"no machine key to seal under, so this wallet is gone at reboot: keep the private key",
        (Kept::Lost(Unsealed::Keyring), false) => b"the keyring would not seal this wallet, so it is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::Keyring), true) => b"the keyring would not seal this wallet, so it is gone at reboot: keep the private key",
        (Kept::Lost(Unsealed::DiskFull), false) => b"the disk is full, so this wallet is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::DiskFull), true) => b"the disk is full, so this wallet is gone at reboot: keep the private key",
        (Kept::Lost(Unsealed::Store), false) => b"the store would not keep this wallet, so it is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::Store), true) => b"the store would not keep this wallet, so it is gone at reboot: keep the private key",
    };
    Some(s)
}
