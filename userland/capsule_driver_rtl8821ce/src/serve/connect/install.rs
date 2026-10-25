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
use crate::fw::dma::DmaMem;
use crate::link::{RtlKeys, RtlLink, RxBuffers};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Mmio;
use crate::status;

use super::consts::*;
use super::ends::{counts, missed};
use super::media::set_media_no_link;
use super::result::ConnectResult;
use super::session::Session;

/// What the join came to: on a join, the keys go into the CAM and the
/// session is recorded; the reply carries the handshake's progress either way.
pub(in crate::serve) fn answer<M: Mmio, D: DmaMem, RB: RxBuffers>(
    mut report: assoc::Report,
    ssid: &[u8],
    link: &mut RtlLink<M, D, RB>,
    keys: &mut RtlKeys<M>,
    regs: &M,
    session: &mut Option<Session>,
) -> ConnectResult {
    let mut r = counts(&report);
    // Matched in place: the keys are wiped where they lie when `report` goes.
    r.code = match &mut report.outcome {
        Outcome::Joined { bssid, channel, ptk, gtk, gtk_id, akm, supplicant } => {
            set_rf(regs, *channel, Bw::W20);
            if !keys.install_ptk(&ptk.0, PAIRWISE_KEY_ID, bssid) {
                -3
            } else if !keys.install_gtk(&gtk.0, *gtk_id) {
                -4
            } else {
                // The sec engine decrypts received frames from the CAM. The
                // station encrypts transmit frames in software: the chip's own
                // transmit encryption sent frames marked protected but plain,
                // and the access point dropped them.
                crate::sec::enable_sec_engine(regs);
                link.associate(*bssid, ptk.0);
                if let Some(s) = supplicant.take() {
                    link.set_supplicant(*s);
                }
                *session = Some(Session::new(*bssid, *gtk_id, ssid, akm.suite_type()));
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
