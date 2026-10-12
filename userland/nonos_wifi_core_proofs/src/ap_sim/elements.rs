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

//! The addresses, nonces and elements the simulated access point uses, and
//! the key data elements it wraps into message 3 and group message 1.

pub const AA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
pub const SPA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub const ANONCE: [u8; 32] = [0xA1; 32];
pub const SNONCE: [u8; 32] = [0x52; 32];
pub const GTK: [u8; 16] = [0x11; 16];

/// The WPA2-PSK RSNE the station sends (it equals `wpa::RSN_IE`).
pub const STA_RSNE_PSK: [u8; 22] = nonos_wifi_core::wpa::RSN_IE;
/// An AP beacon RSNE for a WPA2/WPA3 transition network: AKMs PSK and SAE,
/// MFPC set.
pub const AP_RSNE_MIXED: [u8; 26] = [
    0x30, 0x18, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x02, 0x00,
    0x00, 0x0f, 0xac, 0x02, 0x00, 0x0f, 0xac, 0x08, 0x80, 0x00,
];
/// The legacy WPA vendor element a mixed WPA/WPA2 AP also signs into message 3.
pub const WPA_IE: [u8; 24] = [
    0xdd, 0x16, 0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50,
    0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02,
];

/// A GTK KDE for key index `id`.
pub fn gtk_kde(id: u8, gtk: &[u8]) -> Vec<u8> {
    let mut kde = vec![0xDD, 6 + gtk.len() as u8, 0x00, 0x0f, 0xac, 0x01, id, 0x00];
    kde.extend_from_slice(gtk);
    kde
}

/// An IGTK KDE for key index 4.
pub fn igtk_kde(igtk: &[u8; 16]) -> Vec<u8> {
    let mut kde = vec![0xDD, 28, 0x00, 0x0f, 0xac, 0x09, 0x04, 0x00, 1, 0, 0, 0, 0, 0];
    kde.extend_from_slice(igtk);
    kde
}
