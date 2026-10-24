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

//! The one-glance answer: is this session attested, and is it anonymous.
//! Pure, and proven on the host (attest_doc_proofs).
//!
//! Each half is Holds only when everything under it holds, Broken when any
//! piece is broken, and Unknown otherwise; Unknown is never drawn as a pass.

mod marks;
mod verdict;

pub use marks::{admitted_mark, boot_mark, proven_mark, route_mark, Boot, Mark};
pub use verdict::{headline, Headline};
