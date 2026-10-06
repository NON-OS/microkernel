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

//! The handlers behind each operation.

mod client_auth;
mod close;
mod closed;
mod open;
mod path;
mod recv;
mod resolve;
mod send;
mod status;

pub use client_auth::client_auth;
pub use close::{close_circuit, close_stream, end_stream};
pub use open::{not_ready, open};
pub use resolve::resolve;
pub use path::circuit_path;
pub use recv::recv;
pub use send::send;
pub use status::status;
