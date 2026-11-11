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

//! The live proof board, sampled as a session with a Nym route up and three
//! capsules that hold Network.

#[path = "about_double_board.rs"]
mod board;
#[path = "../../capsule_about/src/about/data/proofs/census.rs"]
pub mod census;
#[path = "../../capsule_about/src/about/data/proofs/session.rs"]
pub mod session;
#[path = "../../capsule_about/src/about/data/proofs/words.rs"]
pub mod words;

use alloc::vec::Vec;

use nonos_route_proof::{Latest, RouteVerdict};

pub use board::read;
use session::Boot;

#[derive(Clone, Copy)]
pub struct Holder {
    pub pid: u32,
    pub name: [u8; 16],
    pub name_len: usize,
    pub measurement: [u8; 32],
    pub authority: u8,
}

pub struct Snapshot {
    pub boot: Boot,
    pub census: Option<census::Census>,
    pub holders: Vec<Holder>,
    pub chosen: Option<u8>,
    pub board: Option<(Latest, Latest)>,
    pub route: RouteVerdict,
}
