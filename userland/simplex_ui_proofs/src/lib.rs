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

//! Host proofs for simplex.ui. The protocol, the model and the transports are
//! the capsule's own files, mounted by `#[path]` under the names the capsule
//! gives them, so their `crate::` paths resolve here as they do there. The
//! window's paint is not mounted: it needs the toolkit's fonts and a surface.

extern crate alloc;

#[path = "../../capsule_simplex_ui/src/link/mod.rs"]
pub mod link;
#[path = "../../capsule_simplex_ui/src/model/mod.rs"]
pub mod model;
#[path = "../../capsule_simplex_ui/src/proto/mod.rs"]
pub mod proto;

#[cfg(test)]
mod channel_tests;
#[cfg(test)]
mod fixture;
#[cfg(test)]
mod flow_tests;
#[cfg(test)]
mod json_tests;
#[cfg(test)]
mod key_tests;
#[cfg(test)]
mod model_tests;
#[cfg(test)]
mod proto_tests;
#[cfg(test)]
mod resp_tests;
#[cfg(test)]
mod start_name_tests;
#[cfg(test)]
mod start_tests;
#[cfg(test)]
mod transcript_tests;
#[cfg(test)]
mod wire_tests;
