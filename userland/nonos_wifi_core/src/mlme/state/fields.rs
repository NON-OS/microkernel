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

//! What the MLME holds through a join.

use super::super::failure::MlmeFailure;
use super::kinds::MlmeState;
use super::request::{SAE_ENTROPY, SECRET_MAX};
use crate::dot11::ies::Rates;
use crate::rsn::{JoinPolicy, Selection};
use crate::sae::SaeStation;
use crate::wpa::supplicant::ie::Ie;
use crate::wpa::supplicant::Supplicant;

#[derive(Clone)]
pub struct Mlme {
    pub(in crate::mlme) state: MlmeState,
    pub(in crate::mlme) our_mac: [u8; 6],
    pub(in crate::mlme) ssid: [u8; 32],
    pub(in crate::mlme) ssid_len: usize,
    pub(in crate::mlme) secret: [u8; SECRET_MAX],
    pub(in crate::mlme) secret_len: usize,
    /// The passphrase did not fit `secret`; any join that needs it fails.
    pub(in crate::mlme) secret_too_long: bool,
    pub(in crate::mlme) policy: JoinPolicy,
    pub(in crate::mlme) snonce: [u8; 32],
    pub(in crate::mlme) sae_entropy: [u8; SAE_ENTROPY],
    pub(in crate::mlme) bssid: [u8; 6],
    pub(in crate::mlme) channel: u8,
    pub(in crate::mlme) seq: u16,
    pub(in crate::mlme) selection: Option<Selection>,
    pub(in crate::mlme) ap_rsne: Ie,
    pub(in crate::mlme) ap_rsnxe: Ie,
    pub(in crate::mlme) rates: Option<Rates>,
    pub(in crate::mlme) sae: Option<SaeStation>,
    pub(in crate::mlme) pmk: [u8; 32],
    pub(in crate::mlme) supplicant: Option<Supplicant>,
    pub(in crate::mlme) failure: Option<MlmeFailure>,
}
