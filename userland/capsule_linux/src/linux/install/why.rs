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

//! Why an install stopped, as the installer's exit code, so the system and
//! the store can say more than that it failed. The log line before the exit
//! says which package and which check. The codes are kept by what they are
//! about: a package (why_package.rs), a shipped tier's model (why_model.rs),
//! and the route a model's download leaves by (why_route.rs).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Why(i32);

impl Why {
    pub fn code(self) -> i32 {
        self.0
    }
}

#[path = "why_model.rs"]
mod model;
#[path = "why_package.rs"]
mod package;
#[path = "why_route.rs"]
mod route;
