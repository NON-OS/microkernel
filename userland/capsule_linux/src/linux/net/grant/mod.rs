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

//! A guest family's grant of the anonymity network, asked of the front its
//! streams go through (userland/capsule_linux/design/network-grant.md). This
//! capsule never holds Network: the kernel lets it reach net.anon and
//! net.socks5 alone, and each serves its streams only once the family holds
//! a grant there, which the front decides on the kernel's facts: the
//! program's own policy, verified and proved in this family, or the run's
//! box naming a destination, as its argv holds it. One ask a front a run.

mod ask;
mod kept;

pub use ask::{ensure, Front};
pub use kept::keep;

/// What a refused stream says, by network, never by destination.
pub const NOT_GRANTED: &str = "this run's family holds no grant of the network";
