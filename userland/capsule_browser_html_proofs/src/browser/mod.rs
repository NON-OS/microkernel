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

//! The parser's two module trees, compiled from the capsule's source: `dom`
//! holds the tree builder and the arena it fills, `html` the input decoder,
//! the tokenizer and the character reference table.

#[path = "../../../capsule_browser/src/browser/dom/mod.rs"]
pub mod dom;
#[path = "../../../capsule_browser/src/browser/html/mod.rs"]
pub mod html;
