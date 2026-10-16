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

//! Host proofs for the desktop shell's half of the trusted path, on the
//! shell's own rule files (`../capsule_desktop_shell/src/state/`).

pub mod state;

#[cfg(test)]
#[path = "../../compositor/src/display/arrange_wire.rs"]
#[allow(dead_code)]
mod arrange_wire;
/// The compositor's ops and the arrangement's answer, which the table names.
#[cfg(test)]
#[path = "../../compositor/src/protocol/ops.rs"]
#[allow(dead_code)]
mod ncmp;
/// The shell's ops, and its table of those Settings may send on.
#[path = "../../capsule_desktop_shell/src/protocol/ops.rs"]
pub mod protocol;
#[path = "../../capsule_desktop_shell/src/server/handlers/relay_table.rs"]
pub mod relay_table;

#[cfg(test)]
mod attention_tests;
#[cfg(test)]
mod band_tests;
#[cfg(test)]
mod relay_tests;
#[cfg(test)]
mod router_tests;
