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
 * The catalogue this image was built with, by tools/nonos-qwen-tier.py.
 * Empty when the build had no operator seed, which this says in words.
 */

use super::check::check;
use super::types::Catalogue;

static BUILT: &[u8] = include_bytes!("../../../../target/models/catalogue.bin");

pub fn load() -> Result<Catalogue, &'static str> {
    if BUILT.is_empty() {
        return Err("this system was built without a signed model catalogue (no \
                    marketplace operator key), so it has nothing to fetch from; \
                    put a tier on its disk with tools/nonos-qwen-tier.py instead");
    }
    check(BUILT)
}
