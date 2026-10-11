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

//! The characters of a NameSeg, section 20.2.2.

/// LeadNameChar: 'A'-'Z' or '_'.
pub const fn is_lead_char(b: u8) -> bool {
    b.is_ascii_uppercase() || b == b'_'
}

/// NameChar: a LeadNameChar or a digit.
pub const fn is_name_char(b: u8) -> bool {
    is_lead_char(b) || b.is_ascii_digit()
}

/// The bytes a NameString can begin with: RootChar, ParentPrefixChar,
/// DualNamePrefix, MultiNamePrefix or a LeadNameChar. NullName (0x00) also
/// begins one, but where it is allowed the byte is ZeroOp in every other
/// position, so the callers that accept it check for it themselves.
pub const fn is_name_start(b: u8) -> bool {
    matches!(b, b'\\' | b'^' | 0x2E | 0x2F) || is_lead_char(b)
}
