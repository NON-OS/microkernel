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

//! The Pairing Request this host sends.

use crate::method::IoCap;
use crate::pdu::{auth, keys, Pairing};

/// This host's request: a keyboard and a display, bonding with MITM
/// protection and Secure Connections, a 16-octet key, and the responder's
/// identity key asked for so a peer with a private address can be found
/// again. It distributes no key of its own: with Secure Connections the LTK
/// is derived on both sides, not sent (Vol 3, Part H, 3.6.1).
pub const REQUEST: Pairing = Pairing {
    io_cap: IoCap::KeyboardDisplay,
    oob: false,
    auth_req: auth::BONDING | auth::MITM | auth::SC,
    max_key_size: 16,
    init_keys: 0,
    resp_keys: keys::ID,
};
