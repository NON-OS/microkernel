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

//! SERVE_ONE's wire server as it ships: each frame from the one capsule a
//! box serves, answered from the family's socket table through the door
//! end (sock/door.rs), at the crate paths it names in the capsule.

#[path = "../../../../capsule_linux/src/linux/net/serve_one/answer.rs"]
pub mod answer;
#[path = "../../../../capsule_linux/src/linux/net/serve_one/conn.rs"]
pub mod conn;
#[path = "../../../../capsule_linux/src/linux/net/serve_one/life.rs"]
pub mod life;
#[path = "../../../../capsule_linux/src/linux/net/serve_one/read.rs"]
pub mod read;
#[path = "../../../../capsule_linux/src/linux/net/serve_one/server.rs"]
pub mod server;
#[path = "../../../../capsule_linux/src/linux/net/serve_one/write.rs"]
pub mod write;
