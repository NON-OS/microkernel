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

//! What an attempt starts from beyond the network and the request.

use crate::pmksa::Pmksa;
use crate::softmac::roam::Secret;

/// What an attempt starts from beyond the network: a cached PMKSA to offer,
/// the AP the station moves from when it is a reassociation, and what a
/// roaming session kept in place of the passphrase.
#[derive(Default)]
pub struct Extra<'a> {
    pub offer: Option<Pmksa>,
    pub from: Option<[u8; 6]>,
    pub kept: Option<&'a Secret>,
}
