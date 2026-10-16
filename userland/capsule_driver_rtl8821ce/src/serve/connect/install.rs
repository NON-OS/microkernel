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

//! A join's keys installed, or why it did not join.

use nonos_wifi_core::key::KeyStore;

use crate::assoc::{self, Outcome};
use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;
use crate::status;

use super::consts::*;
use super::ends::{counts, missed};
use super::media::set_media_no_link;
use super::result::ConnectResult;
use super::session::Session;

/// What the join came to: on a join, the keys go into the CAM and the
/// session is recorded; the reply carries the handshake's progress either way.
pub(super) fn answer(
    report: assoc::Report,
    ssid: &[u8],
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    regs: &Regs,
    session: &mut Option<Session>,
) -> ConnectResult {
    let mut r = counts(&report);
    r.code = match report.outcome {
        Outcome::Joined { bssid, channel, ptk, gtk, gtk_id, akm, supplicant } => {
            set_rf(regs, channel, Bw::W20);
            if !keys.install_ptk(&ptk, PAIRWISE_KEY_ID, &bssid) {
                -3
            } else if !keys.install_gtk(&gtk, gtk_id) {
                -4
            } else {
                // The keys are in the CAM, so turn the sec engine on for receive
                // decryption (this chip decrypts received frames correctly). The
                // station encrypts transmit frames in software and sends them
                // through the plaintext path the handshake proved works, because
                // the chip's hardware transmit encryption sent frames protected
                // but unencrypted and the access point dropped them.
                crate::sec::enable_sec_engine(regs);
                link.associate(bssid, ptk);
                link.set_supplicant(*supplicant);
                *session = Some(Session::new(bssid, gtk_id, ssid, akm.suite_type()));
                status::debug(b"[rtl8821ce] connect: associated\n");
                0
            }
        }
        outcome => missed(outcome, report.failure, &mut r),
    };
    // A join that did not end associated leaves the MAC to the scan again.
    if r.code != 0 {
        set_media_no_link(regs);
    }
    r
}
