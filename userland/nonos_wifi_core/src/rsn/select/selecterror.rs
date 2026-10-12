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

//! Why a network cannot be joined as it advertises itself.

/// Why a network cannot be joined as it advertises itself.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectError {
    /// The group or pairwise cipher is not CCMP-128.
    UnsupportedCipher,
    /// No offered AKM is one this station runs (802.1X/Enterprise, FT, OWE).
    UnsupportedAkm,
    /// The network was saved as WPA3 and now offers only WPA2.
    Downgrade,
    /// The capabilities contradict themselves (MFPR without MFPC) or name a
    /// management cipher other than BIP-CMAC-128 while requiring protection.
    BadProtection,
}
