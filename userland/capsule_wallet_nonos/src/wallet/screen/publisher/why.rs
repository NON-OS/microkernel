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

//! Why a publisher call was not built from the entry (intent::Why), in
//! words: the entry's own refusals as the binding says them.

use crate::wallet::intent::Why;
use crate::wallet::publish::why as entry_why;

pub fn why(w: Why) -> &'static str {
    match w {
        Why::Entry(r) => entry_why(r),
        Why::BelowMinBond => "that is below the Bond's minimum bond, or nothing",
        Why::BelowUnitPrice => "an escrow listing's bond is at least its unit price",
        Why::NotAnAddress => "that is not one address: 0x and 40 hex digits",
        Why::Checksum => "that mixes cases without the address's checksum; check every digit",
        Why::ZeroAgent => "the zero address delivers nothing",
        Why::Locked => "the bond is still locked",
        Why::NoCredit => "the Bond holds no credit for this account",
        Why::Uri => "the uri is https, at most 256 bytes, with no user, password or port",
        Why::Schema2 => "this is a schema 2 release, which is never signed under v3",
    }
}
