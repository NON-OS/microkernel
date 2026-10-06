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
 * The route this boot offers now: the policy store's default network, and
 * which of the anonymity networks have registered their services, as `pick`
 * reads them. Asked for each connection, so a choice changed in Settings
 * holds from the next one on.
 */

use nonos_policy_proto::Field;
use nonos_socket::lookup;

use crate::direct_only::direct_refused;
use crate::pick::{install_route, pick, private_only, Route};

/*
 * The default network as the policy store holds it, or None when the store
 * has not announced itself or did not answer. None is read as the Nym
 * mixnet wherever it is used, never as Direct.
 */
pub fn default_route() -> Option<u8> {
    nonos_policy_client::lookup()
        .and_then(|port| nonos_policy_client::get_u8(port, Field::NetworkRoute))
}

impl Route {
    pub fn chosen() -> Route {
        pick(default_route(), lookup(b"net.socks5"), lookup(b"net.anon"))
    }

    /* The route a wallet's RPC takes (`private_only`): Nym or Anyone, never Direct. */
    pub fn for_wallet() -> Route {
        private_only(Route::chosen(), lookup(b"net.socks5"), lookup(b"net.anon"))
    }

    /* The route an install's download takes (`install_route`): Anyone. */
    pub fn for_installs() -> Route {
        install_route(lookup(b"net.anon"))
    }
}

/*
 * Why a contact that can only leave directly must not be made now, or None
 * when Direct is the default and it may.
 */
pub fn direct_refusal() -> Option<&'static str> {
    direct_refused(default_route())
}
