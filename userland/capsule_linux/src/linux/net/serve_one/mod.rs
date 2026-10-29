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

//! SERVE_ONE: the one listener a box's policy names, on 127.0.0.1 at its
//! field's port, served to the one capsule its field names and to no one
//! else (docs/design/serve-one.md). The guest's first listen there asks the
//! kernel to register linux.serve.<capsule> for this process, held to that
//! capsule (served.rs). The capsule's frames (nonos_serve_wire) come to this
//! process's inbox, and the serve loop answers them (pump.rs) from the
//! family's own socket table, through a door end the personality holds
//! (server.rs, sock/door.rs), so the guest's accept, read, write, close,
//! poll and select are its family's own.

mod answer;
mod conn;
mod gate;
mod life;
mod owner;
mod pump;
mod read;
mod served;
mod server;
mod write;

pub use gate::gate;
pub use pump::{pump, wake_ms};
