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

/* Why a mirror did not give a file, in words for the person waiting. */

use alloc::format;
use alloc::string::String;

pub enum Fault {
    /* The route or the mirror did not carry the request. */
    Net(&'static str),
    /* The mirror answered, but not with the file. */
    Status(u16),
    /* The answer was not one this can take the file from. */
    Unusable(&'static str),
}

impl Fault {
    pub fn said(&self) -> String {
        match self {
            Fault::Net(why) | Fault::Unusable(why) => String::from(*why),
            Fault::Status(code) => format!("the mirror answered HTTP {code}"),
        }
    }
}
