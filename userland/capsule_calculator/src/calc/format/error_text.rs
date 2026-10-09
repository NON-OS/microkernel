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

//! What the readout says in place of a number when a calculation stopped. One
//! word for every cause left the user to guess whether they divided by zero,
//! went past the largest value or asked for something undefined.

use crate::calc::error_kind::ErrorKind;

pub fn error_text(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::DivByZero => "Cannot divide by zero",
        ErrorKind::Overflow => "Result too large",
        ErrorKind::DomainError => "Not defined",
        ErrorKind::None => "",
    }
}
