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

//! Host proofs for the browser's HTTP response path, text decoding, inflate
//! and stylesheet folding, run against the capsule's own source.

extern crate alloc;

pub mod browser;

#[cfg(test)]
mod bits;
#[cfg(test)]
mod cases_tests;
#[cfg(test)]
mod charset_tests;
#[cfg(test)]
mod chunked_tests;
#[cfg(test)]
mod css_fold_tests;
#[cfg(test)]
mod framing_tests;
#[cfg(test)]
mod inflate_tests;
#[cfg(test)]
mod keepalive_tests;
#[cfg(test)]
mod partial_tests;
#[cfg(test)]
mod prescan_tests;
#[cfg(test)]
mod sniff_tests;
#[cfg(test)]
mod vectors;
