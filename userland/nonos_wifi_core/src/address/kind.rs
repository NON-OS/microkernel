// NONOS Operating System (AGPL-3.0-or-later)

//! The security a network is joined under, which keys its address with its
//! name: a network of the same name under another security is another
//! network, and sees another address.

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
    /// The octet the derivation takes.
    pub(super) const fn octet(self) -> u8 {
        match self {
            Security::Open => 0,
            Security::Psk => 1,
            Security::Sae => 2,
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
