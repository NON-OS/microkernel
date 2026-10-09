/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! One scanned WiFi network as the panel shows it: the network name, a signal
//! strength for sorting and display, whether it needs a passphrase, and which
//! personal security it offers (WPA2, WPA3, or both on a transition network).

pub const SSID_MAX: usize = 32;

#[derive(Clone, Copy)]
pub struct ScanNetwork {
    ssid: [u8; SSID_MAX],
    ssid_len: usize,
    pub signal: u8,
    pub secured: bool,
    /// Offers WPA2-Personal (PSK).
    pub wpa2: bool,
    /// Offers WPA3-Personal (SAE).
    pub wpa3: bool,
}

impl ScanNetwork {
    /// No network: the value unused slots hold.
    pub const EMPTY: Self = Self {
        ssid: [0u8; SSID_MAX],
        ssid_len: 0,
        signal: 0,
        secured: false,
        wpa2: false,
        wpa3: false,
    };

    pub fn new(ssid: &[u8], signal: u8, secured: bool) -> Self {
        let mut buf = [0u8; SSID_MAX];
        let len = ssid.len().min(SSID_MAX);
        buf[..len].copy_from_slice(&ssid[..len]);
        Self { ssid: buf, ssid_len: len, signal, secured, wpa2: false, wpa3: false }
    }

    /// The same network with the personal security it offers.
    pub fn offering(self, wpa2: bool, wpa3: bool) -> Self {
        Self { wpa2, wpa3, ..self }
    }

    pub fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len]
    }
}

impl Default for ScanNetwork {
    fn default() -> Self {
        Self::EMPTY
    }
}
