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

//! The display wire's sweep, shared with gfx_serve_proofs and run here on
//! this driver's own server: any frame a compositor might send, answered
//! as the wire defines (`server_sweep.rs` says what is held).

#[path = "../../../../gfx_serve_proofs/src/sweep/asks.rs"]
pub mod asks;
#[path = "../../../../gfx_serve_proofs/src/sweep/check.rs"]
pub mod check;
#[path = "../../../../gfx_serve_proofs/src/sweep/heap.rs"]
pub mod heap;
pub mod kit;
#[path = "../../../../gfx_serve_proofs/src/sweep/payload.rs"]
pub mod payload;
#[path = "../../../../gfx_serve_proofs/src/sweep/read.rs"]
pub mod read;
#[path = "../../../../gfx_serve_proofs/src/sweep/rng.rs"]
pub mod rng;
#[path = "../../../../gfx_serve_proofs/src/sweep/server_sweep.rs"]
pub mod server_sweep;
mod sweep_tests;
