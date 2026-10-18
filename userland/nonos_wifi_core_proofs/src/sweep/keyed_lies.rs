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
//! The lies told to message 3, one kind per round.

use super::rng::Rng;
use crate::ap_sim::*;
use nonos_wifi_core::eapol::parse::*;
use nonos_wifi_core::wpa::akm::Akm;

/// Lie `which` (0 to 3) about `m3`, the honest message 3 carrying `kd`.
pub fn lie(rng: &mut Rng, which: usize, m3: &[u8], kd: &[u8], ptk: &[u8]) -> Vec<u8> {
    let mut m = m3.to_vec();
    match which {
        0 => {
            let at = rng.below(m.len());
            m[at] ^= 1 + rng.below(255) as u8;
            m
        }
        1 => {
            let n = (m.len() - 99 + 1 + rng.below(100)) as u16;
            m[97..99].copy_from_slice(&n.to_be_bytes());
            m
        }
        2 => {
            // Cut short, or a whole wrap with part of a block after.
            let whole = &m3[99..];
            let wrapped = match rng.below(2) {
                0 => whole[..whole.len() - 1 - rng.below(7)].to_vec(),
                _ => [whole, &rng.span(1, 7)].concat(),
            };
            let info = KEY_INFO_PAIRWISE | KEY_INFO_ACK | KEY_INFO_MIC | KEY_INFO_INSTALL;
            let info = info | KEY_INFO_SECURE | KEY_INFO_ENCRYPTED;
            frame(Akm::Psk, info, 2, &ANONCE, &wrapped, &ptk[..16])
        }
        _ => {
            // The GTK KDE's length, past the key data: authentic, but a lie.
            let mut lying = kd.to_vec();
            let gtk_at = AP_RSNE_MIXED.len() + 1;
            lying[gtk_at] = lying[gtk_at].wrapping_add(1 + rng.below(40) as u8);
            message3(Akm::Psk, 2, &ANONCE, ptk, &lying)
        }
    }
}
