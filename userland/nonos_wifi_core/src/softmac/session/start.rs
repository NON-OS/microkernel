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

//! A session opened on a finished join: the station engine keyed with the
//! pairwise key in software, so every chip is driven the same way, and the
//! supplicant handed over for the AP's later key handshakes.

use alloc::collections::VecDeque;

use super::state::{Keys, Session};
use crate::regdom::Band;
use crate::rsn::TemporalKey;
use crate::station::{Ccmp, LinkStation};

impl Session {
    pub fn start(
        mac: [u8; 6],
        net: (&[u8], [u8; 6], u8),
        at: (Band, u8),
        keys: Keys,
        now_ms: u64,
    ) -> Self {
        let (name, bssid, akm) = net;
        let pmf = keys.supplicant.pmf();
        let mut station = LinkStation::new(mac);
        let mode = match keys.tk {
            TemporalKey::Ccmp128(tk) => Ccmp::Software { tk },
            TemporalKey::Gcmp256(tk) => Ccmp::Gcmp256 { tk },
        };
        station.associate(bssid, mode);
        station.set_supplicant(keys.supplicant);
        let (mut ssid, len) = ([0u8; 32], name.len().min(32));
        ssid[..len].copy_from_slice(&name[..len]);
        let (band, channel) = at;
        let queue = VecDeque::new();
        Self {
            bssid,
            ssid,
            ssid_len: len as u8,
            akm,
            band,
            channel,
            pmf,
            station,
            queue,
            heard_ms: now_ms,
            kit: None,
        }
    }
}
