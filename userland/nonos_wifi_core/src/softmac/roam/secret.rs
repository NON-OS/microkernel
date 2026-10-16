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

//! What a session keeps so it can join another AP of its network on its
//! own: the least secret its AKM needs, never more. A session-long copy of
//! the passphrase would undo the drivers' wipe of the connect request and
//! its sealing at rest, so:
//! - PSK and PSK-SHA256 keep the PMK only, which is the same for every AP
//!   of the network;
//! - SAE by hash-to-element keeps the password token only;
//! - SAE by hunting and pecking keeps the password, which it needs.
//!
//! Each is held in a vault, wiped when the session ends however it ends.

use crate::mlme::SECRET_MAX;
use crate::sae::group::point_xy;
use crate::sae::h2e::derive_pt;
use crate::vault::{assert_not_copy, wipe, wipe_value, Vault};
use crate::wpa::akm::Akm;
use crate::wpa::psk::psk;
use nonos_p256::ProjectivePoint;

pub enum Secret {
    Pmk(Vault<32>),
    Pt(Vault<64>),
    Password(Vault<SECRET_MAX>),
}

assert_not_copy!(Secret);

impl Secret {
    /// What a session that joined `ssid` with `pass` under `akm` keeps;
    /// `None` if `pass` cannot make it.
    pub fn new(akm: Akm, h2e: bool, ssid: &[u8], pass: &[u8]) -> Option<Self> {
        match (akm, h2e) {
            (Akm::Sae, false) => Vault::new(pass).map(Self::Password),
            (Akm::Sae, true) => {
                let mut pt = derive_pt(ssid, pass, None)?;
                let mut xy = point_xy(&pt);
                wipe_value(&mut pt, ProjectivePoint::IDENTITY);
                let kept = xy.as_ref().and_then(|b| Vault::new(b)).map(Self::Pt);
                xy.iter_mut().for_each(|b| wipe(b));
                kept
            }
            _ => {
                let mut pmk = psk(pass, ssid)?;
                let kept = Vault::new(&pmk).map(Self::Pmk);
                wipe(&mut pmk);
                kept
            }
        }
    }
}
