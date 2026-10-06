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

//! The audio server's stream table and the ring each stream feeds, from
//! capsule source. Their own style is allowed on the include rather than
//! restyled here.

#[allow(
    dead_code,
    clippy::new_without_default,
    clippy::len_without_is_empty,
    clippy::needless_range_loop
)]
#[path = "../../../capsule_audio/src/server/ring.rs"]
pub mod ring;
#[allow(dead_code, clippy::new_without_default)]
#[path = "../../../capsule_audio/src/server/streams.rs"]
pub mod streams;
// The request header decode and the replies the server answers with.
#[path = "../../../capsule_audio/src/server/proto.rs"]
pub mod proto;
// How the server reads driver.hda0's replies, its output status among them.
#[allow(dead_code)]
#[path = "../../../capsule_audio/src/sink/wire.rs"]
pub mod wire;
