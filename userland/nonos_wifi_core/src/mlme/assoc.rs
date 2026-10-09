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

//! Association. The request carries the SSID, the access point's own legacy
//! rates (so its basic rates are always among them), the station's RSNE for
//! the selection and, when SAE runs hash-to-element, the station's RSNXE;
//! message 2 repeats the same RSNE and RSNXE. A response counts only from the
//! chosen BSS to this station. Status 0 starts the four-way handshake with a
//! supplicant that holds the beacon's elements; status 30 ("try again later",
//! sent while the AP checks an earlier association with management frame
//! protection) keeps waiting so the retransmitted request can be answered;
//! any other status fails the join with that code.

use super::failure::MlmeFailure;
use super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::auth::from_bss_to_us;
use crate::dot11::ies::{ap_rates, EID_EXT_SUPPORTED_RATES, EID_SUPPORTED_RATES};
use crate::dot11::mgmt::{assoc_request_elements, element, IE_SSID};
use crate::dot11::parse::parse_assoc_response;
use crate::rsn::build::station_rsne;
use crate::rsn::rsnxe::STATION_RSNXE_H2E;
use crate::rsn::Pmf;
use crate::wpa::supplicant::{Config, Supplicant};

/// Capability information: ESS, and Privacy (an RSN network is a private one).
const CAP_ESS_PRIVACY: u16 = 0x0011;
/// Status 30: association request rejected temporarily; try again later.
const STATUS_REJECTED_TEMPORARILY: u16 = 30;

impl Mlme {
    pub(super) fn assoc_request(&mut self) -> MlmeOutput {
        let Some(sel) = self.selection else {
            return self.fail(MlmeFailure::AssocRejected(0));
        };
        let rates = self.rates.unwrap_or_else(|| ap_rates(&[]));
        let rsne = station_rsne(&sel);
        // Each element is built into its own small buffer, then the request.
        let mut ssid_e = [0u8; 34];
        let mut sr_e = [0u8; 10];
        let mut esr_e = [0u8; 10];
        let (Some(ssid_n), Some(sr_n), Some(esr_n)) = (
            element(&mut ssid_e, 0, IE_SSID, self.ssid()),
            element(&mut sr_e, 0, EID_SUPPORTED_RATES, rates.supported()),
            element(&mut esr_e, 0, EID_EXT_SUPPORTED_RATES, rates.extended()),
        ) else {
            return self.fail(MlmeFailure::AssocRejected(0));
        };
        let esr: &[u8] = if rates.extended().is_empty() { &[] } else { &esr_e[..esr_n] };
        let rsnxe: &[u8] = if sel.sae_h2e { &STATION_RSNXE_H2E } else { &[] };
        let elements: [&[u8]; 5] = [&ssid_e[..ssid_n], &sr_e[..sr_n], esr, &rsne, rsnxe];
        let seq = self.next_seq();
        let mut out = [0u8; 160];
        match assoc_request_elements(&mut out, self.our_mac, self.bssid, CAP_ESS_PRIVACY, seq, &elements) {
            Some(n) => {
                self.state = MlmeState::Associating;
                MlmeOutput::send(out[..n].to_vec())
            }
            None => self.fail(MlmeFailure::AssocRejected(0)),
        }
    }

    pub(super) fn on_assoc_response(&mut self, frame: &[u8]) -> MlmeOutput {
        if !from_bss_to_us(frame, &self.our_mac, &self.bssid) {
            return MlmeOutput::none();
        }
        match parse_assoc_response(frame) {
            Some((0, _aid)) => {
                self.start_handshake();
                MlmeOutput::none()
            }
            Some((STATUS_REJECTED_TEMPORARILY, _)) => MlmeOutput::none(),
            Some((status, _)) => self.fail(MlmeFailure::AssocRejected(status)),
            None => MlmeOutput::none(),
        }
    }

    // Associated: the AP sends message 1 next, which arrives via on_eapol.
    fn start_handshake(&mut self) {
        let Some(sel) = self.selection else {
            self.fail(MlmeFailure::AssocRejected(0));
            return;
        };
        let own_rsne = station_rsne(&sel);
        let sup = Supplicant::configure(&Config {
            pmk: self.pmk,
            aa: self.bssid,
            spa: self.our_mac,
            snonce: self.snonce,
            akm: sel.akm,
            own_rsne: &own_rsne,
            own_rsnxe: sel.sae_h2e.then_some(&STATION_RSNXE_H2E[..]),
            ap_rsne: self.ap_rsne.as_slice(),
            ap_rsnxe: self.ap_rsnxe.as_option(),
            pmf: sel.pmf != Pmf::Off,
        });
        self.supplicant = Some(sup);
        self.state = MlmeState::FourWay;
    }
}
