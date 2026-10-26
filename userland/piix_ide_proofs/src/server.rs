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

//! The driver's server files by path, with the request path public.

#[path = "../../capsule_driver_piix_ide/src/server/dispatch.rs"]
mod dispatch;
#[path = "../../capsule_driver_piix_ide/src/server/endpoint.rs"]
pub mod endpoint;
#[path = "../../capsule_driver_piix_ide/src/server/handlers.rs"]
mod handlers;
#[path = "../../capsule_driver_piix_ide/src/server/medium.rs"]
mod medium;
#[path = "../../capsule_driver_piix_ide/src/server/reply.rs"]
mod reply;
#[path = "../../capsule_driver_piix_ide/src/server/runner.rs"]
mod runner;
#[path = "../../capsule_driver_piix_ide/src/server/rw.rs"]
pub mod rw;

pub use runner::{run, serve, RX_LEN, TX_LEN};
