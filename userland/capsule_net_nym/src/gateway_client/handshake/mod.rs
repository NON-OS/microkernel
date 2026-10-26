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

//! The gateway registration handshake.

mod derive;
mod ephemeral;
mod finish;
mod frame;
mod init;
mod material;
mod parse_gateway;
mod run;
mod seal_material;
mod sizes;
mod verify_material;
mod wire;
mod ws_wire;

pub use run::run_handshake;
pub use wire::Identity;
pub use ws_wire::WsWire;
