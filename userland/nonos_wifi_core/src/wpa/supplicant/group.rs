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

//! The group key handshake (IEEE Std 802.11-2020, 12.7.7). An access point
//! replaces its group key on a timer (hostapd's default is a day; many
//! consumer routers do it hourly) and sends every associated station group
//! message 1 with the new GTK wrapped under the KEK. A station that never
//! answers is deauthenticated after the retries, which on a link that ran fine
//! for an hour reads as Wi-Fi dropping by itself. Here the message is verified
//! under the KCK, its key data unwrapped, and group message 2 sent back; the
//! new key is reported for installation unless it is the key already in use,
//! which is never reinstalled.

use super::keys::{parse_group_keys, rsc_pn, wipe, UNWRAPPED_MAX};
use super::state::Supplicant;
use super::step::StepOutput;
use crate::eapol::mic::verify_mic_kind;
use crate::eapol::parse::{EapolKey, KEY_INFO_ENCRYPTED, KEY_INFO_SECURE};

/// The Key Information bits every group message 1 carries.
const GROUP1_BITS: u16 = KEY_INFO_SECURE | KEY_INFO_ENCRYPTED;

impl Supplicant {
    pub(super) fn on_group1(&mut self, frame: &[u8], key: &EapolKey<'_>) -> StepOutput {
        if key.key_info & GROUP1_BITS != GROUP1_BITS
            || !verify_mic_kind(self.akm.mic_kind(), self.kck(), frame)
        {
            return StepOutput::none();
        }
        self.rx_replay = Some(key.replay_counter);
        let mut plain = [0u8; UNWRAPPED_MAX];
        let keys = self
            .unwrap_key_data(key.key_data, &mut plain)
            .and_then(|n| parse_group_keys(&plain[..n]).map(|(_, k)| k));
        wipe(&mut plain);
        // An authentic message whose key data does not hold a usable group key
        // is not answered; the AP retries and in the end deauthenticates, which
        // is the right outcome for a key the station cannot use.
        let Some(mut keys) = keys else {
            return StepOutput::none();
        };
        keys.rsc = rsc_pn(&key.key_rsc);
        let changed = self.store_group_keys(&keys);
        StepOutput { reply: self.group2(&key.replay_counter), new_group_key: changed }
    }
}
