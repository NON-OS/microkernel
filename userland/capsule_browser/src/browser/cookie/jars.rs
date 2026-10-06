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

//! One jar for each network, and no way to reach one from another.

use super::jar::Jar;
use crate::browser::net::mixnet::Network;

/*
 * A cookie is an identifier the site chose. Sent over Direct, one set over
 * Nym ties the reader's address to everything they did over Nym, which is
 * what the mixnet was there to prevent. The jars are separate values rather
 * than a field on each cookie, so no lookup can match across them by
 * mistake.
 */
pub struct Jars {
    direct: Jar,
    nym: Jar,
    anyone: Jar,
}

impl Default for Jars {
    fn default() -> Self {
        Self::new()
    }
}

impl Jars {
    pub const fn new() -> Jars {
        Jars { direct: Jar::new(), nym: Jar::new(), anyone: Jar::new() }
    }

    pub fn of(&mut self, net: Network) -> &mut Jar {
        match net {
            Network::Direct => &mut self.direct,
            Network::Nym => &mut self.nym,
            Network::Anyone => &mut self.anyone,
        }
    }

    pub fn get(&self, net: Network) -> &Jar {
        match net {
            Network::Direct => &self.direct,
            Network::Nym => &self.nym,
            Network::Anyone => &self.anyone,
        }
    }
}
