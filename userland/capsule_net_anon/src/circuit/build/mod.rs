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

//! Building a circuit over an open link, one hop per reply.
//!
//! Nothing here waits. A request for the next hop is sent, the circuit keeps
//! the handshake it is waiting on, and the reply is finished whenever the
//! link delivers it. A build that waited for each reply stopped the serve
//! loop for up to fifteen seconds a hop.

mod answer;
mod ask;
mod error;
mod reply;
mod timing;

pub use answer::answer;
pub use ask::{ask_create, ask_extend};
pub use error::BuildError;
pub use timing::HOP_MS;
