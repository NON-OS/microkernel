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

//! Fixed-size fields out of a PDU's parameters, and PDUs out of them.
//! `parse` has already checked each code's length; these still return
//! `None` rather than panic, so a slip there cannot take the host down.

use alloc::vec::Vec;

pub(super) fn take<const N: usize>(p: &[u8], at: usize) -> Option<[u8; N]> {
    p.get(at..at.checked_add(N)?)?.try_into().ok()
}

/// A PDU whose parameters are an array of the code's exact length, so
/// `frame`'s length check cannot fail.
pub(super) fn pdu(code: u8, params: &[u8]) -> Vec<u8> {
    [&[code][..], params].concat()
}
