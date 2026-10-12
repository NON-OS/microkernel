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

//! Proofs for the bond records, the book and the rotation timer. Every
//! module is test-only.

#[cfg(test)]
mod book_limit_tests;
#[cfg(test)]
mod book_tests;
#[cfg(test)]
mod fixture;
#[cfg(test)]
mod local_tests;
#[cfg(test)]
mod pairing_tests;
#[cfg(test)]
mod record_refuse_tests;
#[cfg(test)]
mod record_tests;
#[cfg(test)]
mod rotation_tests;
