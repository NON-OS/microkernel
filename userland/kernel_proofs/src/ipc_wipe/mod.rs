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

//! IPC payloads zeroed before they are freed (C8), over the kernel's own
//! message.rs and wipe.rs. hash.rs stands in for the IPC secret.

pub mod hash;
#[path = "../../../../src/ipc/nonos_channel/limits.rs"]
pub mod limits;
#[path = "../../../../src/ipc/nonos_channel/message.rs"]
pub mod message;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod watch;
#[path = "../../../../src/ipc/nonos_channel/wipe.rs"]
pub mod wipe;
