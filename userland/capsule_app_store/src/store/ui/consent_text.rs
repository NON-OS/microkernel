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

//! How each stage of enrolment reads to the user.

use crate::store::consent::Consent;

pub fn label(c: Consent) -> &'static [u8] {
    match c {
        Consent::Idle => b"",
        Consent::Typing(_, 0) => b"code is on the console; type it",
        Consent::Typing(..) => b"typing code, Enter to confirm",
        Consent::Granted => b"this machine will run what it installs",
        Consent::Refused => b"enrolment refused",
    }
}
