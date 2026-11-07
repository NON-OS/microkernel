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

//! The image and its chunk list, held from outside the crate: one test
//! crate, its helpers shared by every part.

#[path = "image/common.rs"]
mod common;
#[path = "image/chunk_list.rs"]
mod chunk_list;
#[path = "image/head_refusals.rs"]
mod head_refusals;
#[path = "image/plan_order.rs"]
mod plan_order;
#[path = "image/round_trip.rs"]
mod round_trip;
#[path = "image/sweep.rs"]
mod sweep;
