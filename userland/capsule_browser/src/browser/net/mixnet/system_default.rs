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

//! The network the browser starts on, read from the policy store.

use super::choice::{choose, Network};

/// Start from the system's default network, which setup asks for and
/// Settings changes. Left on the mixnet when the policy store holds none or
/// cannot be asked; the reader's own choice in the browser still wins after.
pub fn from_system_default() {
    use nonos_policy_proto::{route, Field};
    let default = nonos_policy_client::lookup()
        .and_then(|port| nonos_policy_client::get_u8(port, Field::NetworkRoute));
    match default {
        Some(route::DIRECT) => choose(Network::Direct),
        Some(route::ANYONE) => choose(Network::Anyone),
        Some(_) | None => choose(Network::Nym),
    }
}
