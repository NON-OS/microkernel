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

//! What a guest is told from net.anon's answers, in the terms connect_out.rs
//! uses for net.sockets': no transport, no circuit or no directory is
//! ENETUNREACH; a refusal is ECONNREFUSED; no answer, or one this capsule
//! cannot read, is EIO. A stream table full for this capsule is ENOBUFS.
//! Every reply is checked before it is believed, so a short, overlong or
//! unknown one ends in an errno and never in a panic.

mod data;
pub mod open;
mod opened;

pub use data::{got, sent};
pub use open::{dotted, open_body};
pub use opened::opened;
