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


//! Where the validators are, before there is a mixnet to ask through.
//!
//! Resolving the name sent the first query of every session out in the clear,
//! to whatever resolver the network handed out, naming the service this
//! machine was about to use. The bootstrap set is pinned by address instead,
//! in this capsule's image, which the policy root enrols. The name stays for
//! the certificate check and the Host line; it is never resolved.

/// Host and IPv4 address, as resolved when this list was written.
const PINNED: &[(&str, [u8; 4])] = &[("validator.nymtech.net", [92, 39, 63, 14])];

/// The pinned address for `host`, or `None`: a host not in the set is not
/// reached at all, because the only alternative is a clearnet lookup.
pub fn address(host: &str) -> Option<[u8; 4]> {
    PINNED.iter().find(|(name, _)| *name == host).map(|(_, ip)| *ip)
}
