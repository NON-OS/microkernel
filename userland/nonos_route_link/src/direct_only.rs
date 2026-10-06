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

/*
 * Whether something that can only leave directly may leave at all. A time
 * server asked over UDP, an ICMP echo and a name looked up in the clear have
 * no anonymity network to cross: each one names this machine to whoever
 * answers, and to everyone on the path. They go only when the default
 * network is Direct, the same answer `pick` gives, and an unreadable or
 * unknown default is the Nym mixnet here as there.
 */

use nonos_policy_proto::route;

/*
 * Why a direct-only contact must not be made under `default`, the policy
 * store's answer (None when it could not be asked), or None when Direct is
 * the choice and it may.
 */
pub fn direct_refused(default: Option<u8>) -> Option<&'static str> {
    match default {
        Some(route::DIRECT) => None,
        Some(route::ANYONE) => Some(ANYONE),
        Some(route::NYM) => Some(NYM),
        Some(_) => Some(UNKNOWN),
        None => Some(UNREAD),
    }
}

const NYM: &str = "the chosen network is the Nym mixnet, which is anonymous";
const ANYONE: &str = "the chosen network is the Anyone onion network, which is anonymous";
const UNKNOWN: &str =
    "the chosen network is unknown here, so it is taken as the Nym mixnet, which is anonymous";
const UNREAD: &str =
    "the default network could not be read, so it is taken as the Nym mixnet, which is anonymous";
