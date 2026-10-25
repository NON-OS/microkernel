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

//! The security a network is joined under, which keys its address with its
//! name: a network of the same name under another security is another
//! network, and sees another address.

use super::bits::{SECURITY_OPEN, SECURITY_PASSPHRASE, SECURITY_WPA3};
use crate::dot11::ies::{beacon_tags, find_element, EID_RSN};
use crate::rsn::{parse_rsne, select, JoinPolicy};
use crate::wpa::akm::Akm;

/// The security a join runs: none, WPA2 (PSK, either hash) or WPA3 (SAE).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Security {
    Open,
    Psk,
    Sae,
}

impl Security {
    /// The octet the derivation takes: the request's security modes', so a
    /// network has one address whichever way its security was learned.
    pub const fn octet(self) -> u8 {
        match self {
            Security::Open => SECURITY_OPEN,
            Security::Psk => SECURITY_PASSPHRASE,
            Security::Sae => SECURITY_WPA3,
        }
    }

    /// The security a join under `policy` runs on the network whose beacon
    /// (or probe response) is `frame`, chosen as the MLME chooses it. A
    /// network the MLME refuses at this frame (no RSN element, a malformed
    /// one, nothing it may select) sends nothing, so the kind it gets here
    /// names an address no frame goes out under.
    pub fn of_beacon(frame: &[u8], policy: JoinPolicy) -> Security {
        let Some(rsne) = beacon_tags(frame).and_then(|t| find_element(t, EID_RSN)) else {
            return Security::Open;
        };
        // The AKM `select` picks does not depend on hash-to-element.
        match parse_rsne(&rsne[2..]).map(|r| select(&r, false, policy)) {
            Some(Ok(sel)) if sel.akm == Akm::Sae => Security::Sae,
            _ => Security::Psk,
        }
    }
}
