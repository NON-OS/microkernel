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

//! Reaching hosts from the command line over the network the person chose.
//!
//! Anything the terminal fetches, `curl` and `git` included, leaves the way
//! the browser, the wallet and the model fetcher leave: through the Nym
//! mixnet or the Anyone network, or directly only when Direct is the
//! default, so a shell is not the hole in a machine that is otherwise
//! anonymised. The client is nonos_route_link's; the terminal keeps none of
//! its own.

pub mod exchange;
mod stream;

pub use exchange::{Exchange, Poll, Stage};

/// Whether the mixnet proxy is running, and so whether anything that leaves
/// directly is worth pointing out.
pub fn routed() -> bool {
    stream::proxy_available()
}
