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

//! The relay, flag and weight types, from the real capsule source.

#[path = "../../../capsule_net_anon/src/path/relay.rs"]
mod relay;

#[path = "../../../capsule_net_anon/src/path/weights/mod.rs"]
mod weights;

#[path = "../../../capsule_net_anon/src/path/select/mod.rs"]
mod select;

pub use relay::{Flags, Relay};
pub use select::candidates::candidates;
pub use select::{choose, eligible, Taken};
pub use weights::{weight_for, Position, Weights};

/// `path::draw` is included at the crate root as well, and the real `select`
/// reaches it as `super::draw`. Pointing it at the one instance keeps a single
pub use crate::draw;
