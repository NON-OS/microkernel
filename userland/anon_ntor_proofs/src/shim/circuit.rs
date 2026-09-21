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

//! The real per hop crypto and onion layering, included as the capsule compiles it.

#[path = "../../../capsule_net_anon/src/circuit/extend/mod.rs"]
pub mod extend;

#[path = "../../../capsule_net_anon/src/circuit/hop.rs"]
pub mod hop;

#[path = "../../../capsule_net_anon/src/circuit/open.rs"]
pub mod open;

#[path = "../../../capsule_net_anon/src/circuit/destroy.rs"]
pub mod destroy;

#[path = "../../../capsule_net_anon/src/circuit/seal.rs"]
pub mod seal;

#[path = "../../../capsule_net_anon/src/circuit/window.rs"]
pub mod window;

pub use destroy::destroy;
pub use extend::{extend2_body, extended2_reply, NextHop};
pub use hop::Hop;
pub use open::{open, Opened};
pub use seal::seal;
