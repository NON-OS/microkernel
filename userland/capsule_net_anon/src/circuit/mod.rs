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

//! A circuit: an ordered path of hops, and the onion crypto over it.

mod build;
mod create;
mod destroy;
mod extend;
mod hop;
mod open;
mod seal;
mod state;
pub mod window;

pub use build::build;
pub use create::client_circuit_id;
pub use destroy::destroy;
pub use extend::NextHop;
pub use open::open;
pub use seal::seal;
pub use state::{Circuit, CircuitStage};
