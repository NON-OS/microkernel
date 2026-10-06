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

//! Joining and leaving a network. A connect finds the target's beacon on the air
//! (sending a directed probe only for a network the person marked hidden), runs
//! the whole join through the association driver under the request's policy
//! (WPA3-SAE whenever offered, WPA2 otherwise, never WPA2 for a network saved
//! as WPA3), then installs the negotiated keys into the hardware CAM, hands
//! the data path the supplicant for the AP's later group key handshakes, and
//! records the association. A disconnect tells the AP, drops the association
//! and clears the keys.

use nonos_libc::crypto_random;
use nonos_wifi_core::key::KeyStore;
use nonos_wifi_core::mlme::{Entropy, JoinRequest, MlmeFailure, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::SaeFailure;

use crate::assoc::{self, Outcome};
use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;
use crate::status;

mod hunt;
mod media;
mod probe;
mod request;
pub(crate) mod result;

use hunt::find_beacon;
use media::set_media_connected;
pub(super) use media::set_media_no_link;
use request::parse_connect;
pub(super) use result::ConnectResult;
use result::{
    failure_code, CODE_BAD_REQUEST, CODE_NOT_FOUND, CODE_NO_ENTROPY, CODE_REFUSED, CODE_TIMED_OUT,
};

use super::radio::read_mac;
use super::SCAN_FRAME_MAX;

/// The pairwise key's index. The group key goes in the slot its own index
/// names: with the engine looking group-addressed frames up by the CCMP
/// KeyID, a fixed slot 1 went dark after the AP's first rekey moved it to 2.
const PAIRWISE_KEY_ID: u8 = 0;
/// Milliseconds of uptime to spend joining before giving up. Long enough to
/// cover authentication (SAE adds a round trip), association and the four-way
/// handshake with the association driver's retransmits (an access point resends
/// message 1 about once a second, four times), while the hunt and the join
/// together still return within the panel's 30 s connect IPC timeout.
/// It was a count of receive passes, which shrank in wall time when every core
/// started running and the passes came faster.
const JOIN_BUDGET_MS: u64 = 8_000;
/// The WPA2 join after a WPA3 one that did not finish, on a network offering
/// both: the hunt (two sweeps of 13 channels at 250 ms) and both joins stay
/// inside the panel's 30 s.
const PSK_BUDGET_MS: u64 = 8_000;
/// The CAM entries reserved for group keys, by key index.
const GROUP_KEY_SLOTS: u8 = 4;
/// The AKM suite type of WPA3-SAE.
const AKM_SAE_TYPE: u8 = 8;

/// What the current association installed, so a disconnect clears exactly it,
/// and the network's name and security, which the link op reports.
#[derive(Clone, Copy)]
pub(super) struct Session {
    pub(super) bssid: [u8; 6],
    key_id: u8,
    gtk_id: u8,
    pub(super) ssid: [u8; 32],
    pub(super) ssid_len: u8,
    /// The AKM suite type that ran (2 WPA2-PSK, 6 PSK-SHA256, 8 WPA3-SAE).
    pub(super) akm: u8,
}

impl Session {
    /// Move the group key slot after a rekey the data path installed.
    pub(super) fn set_gtk_id(&mut self, id: u8) {
        self.gtk_id = id;
    }
}

/// Join the network named in the request. The body is the SSID, passphrase and
/// optional flags: `[ssid_len][ssid][pass_len][pass][flags]` (flags bit 0: the
/// network was saved as WPA3, refuse WPA2; bit 1: it is hidden). Returns 0 on
/// success, or a negative code naming where it stopped, with the handshake's
/// progress, the AKM that ran and the AP's own status or reason code.
pub(super) fn connect(
    body: &[u8],
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    regs: &Regs,
    session: &mut Option<Session>,
) -> ConnectResult {
    let Some(req) = parse_connect(body) else {
        return ConnectResult::code(CODE_BAD_REQUEST);
    };
    let our_mac = read_mac(regs);
    // Find the network's beacon (or, hidden, its probe response) on the air,
    // which also tells us its channel.
    let mut beacon = [0u8; SCAN_FRAME_MAX];
    let (found, beacons_heard) = find_beacon(link, regs, req.ssid, req.hidden, our_mac, &mut beacon);
    let Some((beacon_len, channel)) = found else {
        status::debug(b"[rtl8821ce] connect: network not found\n");
        // recv carries how many beacons of any network were heard during the
        // hunt: 0 means the radio heard nothing (receive/tuning problem), a
        // non-zero count means beacons arrive but the target's did not.
        return ConnectResult { recv: beacons_heard, ..ConnectResult::code(CODE_NOT_FOUND) };
    };
    set_rf(regs, channel, Bw::W20);

    // Link the MAC to this BSS before the handshake. The beacon's third address
    // is the BSSID. Until the hardware is told the BSSID and put into managed
    // mode it stays no-link: it hears beacons and the association response
    // (management), but drops the access point's unicast EAPOL data frames, so
    // the four-way handshake never starts. This is why association reached st=3
    // yet no EAPOL was ever counted.
    let mut bssid = [0u8; 6];
    bssid.copy_from_slice(&beacon[16..22]);
    set_media_connected(regs, &bssid);

    // The SNonce goes out in message 2; SAE's rand, mask and stand-in password
    // are drawn separately so nothing secret is derived from it.
    let Some(entropy) = draw_entropy() else {
        return ConnectResult::code(CODE_NO_ENTROPY);
    };
    let policy = if req.wpa3_only { JoinPolicy::WPA3_ONLY } else { JoinPolicy::ANY };
    let join = JoinRequest { our_mac, ssid: req.ssid, passphrase: req.pass, policy, entropy };
    status::debug(b"[rtl8821ce] connect: joining\n");
    let mut now_ms = || nonos_libc::mk_uptime_ms() as u64;
    let beacon = &beacon[..beacon_len];
    let mut report = assoc::run_join(link, &join, beacon, JOIN_BUDGET_MS, &mut now_ms);
    log_progress(&report);
    /*
     * A network that offers WPA3 and WPA2 side by side (transition mode, as
     * many home routers ship) is joined with WPA3 first. When that does not
     * finish, for any reason but the router saying the password is wrong, it
     * is joined again with WPA2, which the router offers and which every WPA2
     * station uses there. Never for a network saved as WPA3.
     */
    if retry_with_psk(&report, req.wpa3_only) {
        let Some(entropy) = draw_entropy() else {
            return ConnectResult::code(CODE_NO_ENTROPY);
        };
        let policy = JoinPolicy::PSK_ONLY;
        let psk = JoinRequest { our_mac, ssid: req.ssid, passphrase: req.pass, policy, entropy };
        status::line(b"[rtl8821ce] connect: WPA3 did not finish; joining with WPA2\n");
        report = assoc::run_join(link, &psk, beacon, PSK_BUDGET_MS, &mut now_ms);
        log_progress(&report);
    }
    let mut r = ConnectResult {
        sent: report.sent,
        recv: report.recv,
        data: report.data,
        eapol: report.eapol,
        probe: report.probe,
        deauth: report.deauth,
        to_us: report.to_us,
        state: report.state,
        akm: report.akm.map_or(0, |a| a.suite_type()),
        ..ConnectResult::default()
    };
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
                link.set_supplicant(supplicant);
                let (mut name, len) = ([0u8; 32], req.ssid.len().min(32));
                name[..len].copy_from_slice(&req.ssid[..len]);
                *session = Some(Session {
                    bssid,
                    key_id: PAIRWISE_KEY_ID,
                    gtk_id,
                    ssid: name,
                    ssid_len: len as u8,
                    akm: akm.suite_type(),
                });
                status::debug(b"[rtl8821ce] connect: associated\n");
                0
            }
        }
        Outcome::Refused => {
            status::debug(b"[rtl8821ce] connect: refused\n");
            let (code, ap_code) = report.failure.map_or((CODE_REFUSED, 0), failure_code);
            r.ap_code = ap_code;
            code
        }
        Outcome::TimedOut => {
            status::debug(b"[rtl8821ce] connect: timed out\n");
            CODE_TIMED_OUT
        }
    };
    // A join that did not end associated leaves the MAC to the scan again.
    if r.code != 0 {
        set_media_no_link(regs);
    }
    r
}

/// Tell the AP the station is leaving, drop the association, unbind the MAC
/// from the BSS and clear the keys it installed.
pub(super) fn disconnect(
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    regs: &Regs,
    session: &mut Option<Session>,
) -> i32 {
    link.send_deauth();
    link.deassociate();
    set_media_no_link(regs);
    if let Some(s) = session.take() {
        keys.remove_key(s.key_id, Some(&s.bssid));
        keys.remove_key(s.gtk_id, None);
        // And the other group slots: an AP rekey moves the group key between
        // indices, leaving the old key in its slot.
        for id in (0..GROUP_KEY_SLOTS).filter(|&id| id != s.gtk_id) {
            keys.remove_key(id, None);
        }
    }
    0
}

// Whether a join that ran WPA3 and did not finish is tried again with WPA2:
// not when the network was saved as WPA3, not when it joined, and not when
// the router's confirm said the password is wrong.
fn retry_with_psk(report: &assoc::Report, wpa3_only: bool) -> bool {
    let ran_sae = report.akm.is_some_and(|a| a.suite_type() == AKM_SAE_TYPE);
    let wrong_password = matches!(report.failure, Some(MlmeFailure::Sae(SaeFailure::BadConfirm)));
    !wpa3_only && ran_sae && !wrong_password && !matches!(report.outcome, Outcome::Joined { .. })
}

// Fresh randomness for one join, or `None` if the kernel gave none.
fn draw_entropy() -> Option<Entropy> {
    let mut e = Entropy { snonce: [0u8; 32], sae: [0u8; SAE_ENTROPY] };
    let a = crypto_random(e.snonce.as_mut_ptr(), e.snonce.len());
    let b = crypto_random(e.sae.as_mut_ptr(), e.sae.len());
    (a == e.snonce.len() as i64 && b == e.sae.len() as i64).then_some(e)
}

// The handshake progress, one line a join: frames sent and received, data and
// EAPOL frames seen, deauthentications, and the state reached, so a failed join
// names where it stopped (`log rtl8821ce`). The panel carries the same numbers.
fn log_progress(report: &assoc::Report) {
    // Always said, one line a join: where a join stops is the first thing a
    // failed one on a real machine needs, and nothing secret is in it.
    status::line(b"[rtl8821ce] connect: sent=");
    status::number(report.sent);
    status::line(b" recv=");
    status::number(report.recv);
    status::line(b" data=");
    status::number(report.data);
    status::line(b" to_us=");
    status::number(report.to_us);
    status::line(b" eapol=");
    status::number(report.eapol);
    status::line(b" deauth=");
    status::number(report.deauth);
    status::line(b" state=");
    status::number(report.state as u32);
    status::line(b" probe=0x");
    status::hex16((report.probe >> 16) as u16);
    status::hex16(report.probe as u16);
    status::line(b"\n");
}
