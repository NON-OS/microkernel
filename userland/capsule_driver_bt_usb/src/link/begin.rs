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

//! The start of a link: a bonded peer is encrypted with its kept key, and
//! any other is paired, offering this host's identity when it connected
//! from a private address.

use super::env::Env;
use super::session::Session;
use super::us::Us;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::le_meta::LeConnectionComplete;

/// The session, or None when pairing could not even start.
pub fn begin<T: Transport, E: Env>(
    host: &mut Host<T>,
    link: &LeConnectionComplete,
    us: &Us,
    env: &mut E,
) -> Result<Option<Session>, HciError> {
    match env.bonded(link.peer_type, &link.peer) {
        Some(record) => Session::resume(host, link, record, env).map(Some),
        None => Ok(Session::open(link, us, env).ok()),
    }
}
