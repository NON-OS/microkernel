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

//! Host proofs for the browser's HTML parser: the WHATWG tokenizer and tree
//! builder the capsule compiles, included unchanged via #[path] under the
//! same module paths, run against known answers and hostile input.
extern crate alloc;

pub mod browser;
pub mod shape;
pub mod tokens;

#[cfg(test)]
mod budget_tests;
#[cfg(test)]
mod content_tests;
#[cfg(test)]
mod entity_tests;
#[cfg(test)]
mod foreign_tests;
#[cfg(test)]
mod fragment_tests;
#[cfg(test)]
mod fuzz_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod input_tests;
#[cfg(test)]
mod legacy_tests;
#[cfg(test)]
mod raw_text_tests;
#[cfg(test)]
mod table_tests;
#[cfg(test)]
mod tokenizer_tests;
#[cfg(test)]
mod tree_tests;
