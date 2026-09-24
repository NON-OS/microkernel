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

//! Host proofs for the audio service's wire format.
//!
//! The format lived twice, hand written at both ends, and the two copies had
//! already diverged in what they covered. Now that it lives once, what it says
//! is worth pinning: the server parses these bytes by fixed offset, so a field
//! moving by two bytes would be read as a frequency of forty million.

#[cfg(test)]
mod header_tests;
#[cfg(test)]
mod tone_tests;
