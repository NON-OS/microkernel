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

//! Choose how to join a network from what its RSN element offers and what the
//! person allowed. SAE is taken whenever the access point offers it (a
//! transition-mode network offers SAE and PSK; joining it with PSK would hand
//! out a handshake an eavesdropper can run a dictionary against), PSK-SHA256 or
//! PSK otherwise. A network saved as WPA3 is joined with SAE or not at all, so
//! an access point that answers to the same name with PSK only is refused as a
//! downgrade instead of being trusted. CCMP-128 is always run and GCMP-256
//! where the radio offers it; a TKIP group cipher would leave broadcast and
//! multicast frames (ARP, DHCP) undecryptable, so such a network is refused
//! with its own reason.

use super::*;

/// What the person allowed for this join.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct JoinPolicy {
    /// WPA2-Personal (PSK, PSK-SHA256) may be used. False for a network saved
    /// as WPA3: then only SAE is acceptable.
    pub allow_psk: bool,
    /// WPA3-Personal (SAE) may be used.
    pub allow_sae: bool,
    /// The ciphers the radio runs beyond CCMP-128.
    pub ciphers: Ciphers,
}

impl JoinPolicy {
    /// The default: SAE when offered, PSK otherwise.
    pub const ANY: JoinPolicy = JoinPolicy::new(true, true);
    /// A network saved as WPA3: SAE only, never downgraded.
    pub const WPA3_ONLY: JoinPolicy = JoinPolicy::new(false, true);
    /// WPA2-PSK only, for a caller that cannot supply SAE's randomness.
    pub const PSK_ONLY: JoinPolicy = JoinPolicy::new(true, false);

    // CCMP-128 alone until the caller says what its radio runs, so a driver
    // that never asks keeps joining exactly as before.
    const fn new(allow_psk: bool, allow_sae: bool) -> Self {
        JoinPolicy { allow_psk, allow_sae, ciphers: Ciphers::CCMP_128_ONLY }
    }

    /// The same policy for a radio that runs `ciphers`.
    pub const fn with_ciphers(self, ciphers: Ciphers) -> Self {
        JoinPolicy { ciphers, ..self }
    }
}
