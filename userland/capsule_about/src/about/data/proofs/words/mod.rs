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

//! The board's words. Pure, so what the screen says is proven against what
//! was concluded (attest_doc_proofs): no headline can claim more than the
//! verdict under it.

mod duration;
mod report;
mod route;
mod verdict;
mod writer;

pub use duration::{duration, of};
pub use report::{directory, hops};
pub use route::*;
pub use verdict::{headline, network, stage};
