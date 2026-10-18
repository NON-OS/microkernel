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

//! The Kani harnesses, compiled only under `cargo kani`.

#[path = "kani_bounds.rs"]
mod kani_bounds;
#[path = "kani_gas.rs"]
mod kani_gas;
#[path = "kani_order.rs"]
mod kani_order;
#[path = "kani_proofs.rs"]
mod kani_proofs;
