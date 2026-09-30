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

//! The one kernel type the key header names from outside its module: the
//! Argon2 cost parameters, with the fields and derives
//! `src/crypto/util/argon2/params.rs` gives them.

pub mod util {
    pub mod argon2 {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Params {
            pub m_kib: u32,
            pub t: u32,
            pub p: u32,
        }
    }
}
