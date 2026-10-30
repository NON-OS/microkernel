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

//! nonos_modes' products of sizes and rates held at u32's edges:
//! CVT's sums and narrowings, a link's capacity, a timing's rates against
//! a range descriptor; each against the test's own reading in u128.

mod cvt_edges;
#[path = "../../../../gfx_serve_proofs/src/sweep/edges.rs"]
mod edges;
mod link_edges;
mod range_edges;
