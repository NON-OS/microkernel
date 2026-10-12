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

//! USB Type-C connectors through UCSI (the PPM firmware declares as
//! USBC000 / PNP0CA0): each port's connection, power direction and
//! charger, read-only, for the power wire. `nonos_ucsi` holds the protocol;
//! this is the mailbox and the _DSM that reach it.

mod dsm;
mod find;
mod ppm;
mod say;
mod serve;
mod state;

pub use find::find;
pub use serve::{notified, tick};
pub use state::TypeC;
