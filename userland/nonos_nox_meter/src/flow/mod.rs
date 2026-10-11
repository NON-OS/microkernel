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

//! A job's steps, pure: what the meter asks settle next,
//! what it signs, and what the job answers. OPEN START and TOPUP ask GAS,
//! sign their two transactions and broadcast each; CLOSE first reads GRACE.
//! The capsule sends each ask as an ASYNC request and feeds the answer back.

mod after;
mod ask;
mod broadcast;
mod fetch;
mod plans;
mod reads;
mod status;
mod step;
mod words;
mod work;

pub use ask::{Ask, Contracts, Next, Step};
pub use step::Flow;
pub use work::Work;
