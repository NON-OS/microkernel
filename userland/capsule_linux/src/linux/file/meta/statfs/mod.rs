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
 * How much room there is, in the shape `statfs` expects.
 *
 * The store's real usage is shared by everything on the machine: read here,
 * it would let a guest watch a sibling write, and it sizes this install. So
 * every guest sees the same plausible figures, and a write that does not fit
 * still fails where it is made, with ENOSPC.
 */

mod calls;
mod fill;

pub use calls::{fstatfs, statfs};
