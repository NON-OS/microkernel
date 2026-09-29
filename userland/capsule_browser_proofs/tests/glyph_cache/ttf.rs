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

//! The toolkit's own blend, raster, cache store and blit sources, under the
//! module names they use for each other, with their proofs as children.

#[path = "../../../toolkit/src/font/ttf/blend.rs"]
mod blend;
#[path = "../../../toolkit/src/font/ttf/evict.rs"]
mod evict;
#[path = "../../../toolkit/src/font/ttf/raster.rs"]
mod raster;
#[path = "../../../toolkit/src/font/ttf/store.rs"]
mod store;
#[path = "../../../toolkit/src/font/ttf/target.rs"]
mod target;

#[path = "blend_tests.rs"]
mod blend_tests;
#[path = "store_tests.rs"]
mod store_tests;
#[path = "target_tests.rs"]
mod target_tests;
