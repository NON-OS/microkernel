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

//! The round trips of the two schemes that do not ship, built only with the
//! `pqc-unvetted` feature (see `crypto::pqc::unvetted`).

#![cfg(feature = "pqc-unvetted")]

mod kat_ntru;
mod kat_sphincs;

pub use kat_ntru::kat_ntru;
pub use kat_sphincs::kat_sphincs;
