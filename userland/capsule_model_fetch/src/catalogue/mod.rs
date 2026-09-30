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

/*
 * The NONOS model repository's catalogue: the one this image was built
 * with, its signature checked under the marketplace operator key and every
 * entry checked against the compiled-in pins before it is believed.
 */

mod check;
mod embed;
mod parse;
mod read;
mod types;

pub use embed::load;
pub use types::{Catalogue, File, Tier};
