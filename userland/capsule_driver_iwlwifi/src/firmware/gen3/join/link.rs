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

//! The open port: Ethernet frames to and from the access point through the
//! shared station engine (`nonos_wifi_core::station::LinkStation`), and the
//! `LinkPort` net_core drives it with, the same link protocol it speaks to
//! the RTL8821CE.
//!
//! The station runs `Ccmp::SoftwareTxHwRx`: it protects every transmit
//! frame itself (CCMP header, packet number, MIC) and the firmware sends it
//! as it is; a receive frame the firmware decrypted from its key table is
//! taken as decrypted, and any other protected frame the station decrypts
//! under the pairwise or group key it holds. Every received data frame
//! passes the station's checks (from the BSS, to this station or a group,
//! not fragmented, protected unless EAPOL, not a replay under its key and
//! TID) before the stack sees it. A frame the firmware decrypted reaches
//! here without its MIC (the receive accelerator strips it), so eight bytes
//! stand in for it: the station's hardware path takes the plaintext between
//! the CCMP header and the MIC and never reads the MIC itself.
//!
//! The access point's group key handshakes and repeated message 3s are
//! answered here, protected; a new group key is kept for the serving loop
//! to install. A deauthentication or disassociation from the access point
//! ends the link: unprotected when management frame protection is off, and
//! only decrypted by the firmware (so its MIC held) when it is on, since an
//! unprotected one may then be forged.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use nonos_wifi_core::dot11::auth::{deauth_frame, parse_leave};
use nonos_wifi_core::dot11::ccmp::CCMP_HDR_LEN;
use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::station::{Ccmp, LinkStation, Rx, RxDrop};
use nonos_wifi_core::wpa::supplicant::Supplicant;

use super::super::region::{Clock, Region};
use super::super::rx_data::RxMpdu;
use super::fw::{Fw, Queue};
use crate::regs::Mmio;

/// Ethernet frames held for net_core between its polls.
pub const RX_READY: usize = 32;
/// `WLAN_REASON_DEAUTH_LEAVING`.
const REASON_LEAVING: u16 = 3;
const MIC_LEN: usize = 8;

/// The link's counts, for the link and status replies.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct LinkStats {
    pub tx_ok: u32,
    pub tx_drop: u32,
    pub rx_frames: u32,
    pub rx_eth: u32,
    /// Data frames the station's checks refused.
    pub rx_refused: u32,
    /// Ethernet frames dropped with net_core not polling.
    pub rx_full: u32,
    pub rekeys: u32,
}

pub struct Link {
    pub station: LinkStation,
    pub ready: VecDeque<Vec<u8>>,
    pub pmf: bool,
    /// The reason code of the access point's deauthentication.
    pub left: Option<u16>,
    /// A group key the access point delivered, for the firmware.
    pub new_group_key: Option<(u8, [u8; 16])>,
    pub stats: LinkStats,
    /// The rate words for data frames and for handshake and management ones.
    pub data_rate: u32,
    pub mgmt_rate: u32,
}

/// The reason code if `rx` is the access point's deauthentication or
/// disassociation of this station that the link must take.
pub fn leave_reason(rx: &RxMpdu, us: &[u8; 6], bssid: &[u8; 6], pmf: bool) -> Option<u16> {
    let f = &rx.frame;
    let protected = f.get(1).is_some_and(|b| b & 0x40 != 0);
    match (pmf, protected) {
        (false, false) => parse_leave(f, us, bssid),
        (true, true) if rx.decrypted => {
            // The reason follows the CCMP header the firmware left in place.
            let mut clear = f.get(..24)?.to_vec();
            clear.extend_from_slice(f.get(24 + CCMP_HDR_LEN..)?);
            parse_leave(&clear, us, bssid)
        }
        _ => None,
    }
}

impl Link {
    /// The link to `bssid` keyed with `tk`, taking over the supplicant the
    /// join finished with.
    pub fn new(us: [u8; 6], bssid: [u8; 6], tk: [u8; 16], sup: Supplicant, data_rate: u32, mgmt_rate: u32) -> Self {
        let mut station = LinkStation::new(us);
        station.associate(bssid, Ccmp::SoftwareTxHwRx { tk });
        let pmf = sup.pmf();
        station.set_supplicant(sup);
        Self {
            station,
            ready: VecDeque::new(),
            pmf,
            left: None,
            new_group_key: None,
            stats: LinkStats::default(),
            data_rate,
            mgmt_rate,
        }
    }

    /// Take one received frame; the frame to send back, if any.
    pub fn receive(&mut self, rx: RxMpdu) -> Option<Vec<u8>> {
        if !self.station.is_associated() {
            return None;
        }
        self.stats.rx_frames = self.stats.rx_frames.wrapping_add(1);
        let (us, bssid) = (self.station.mac(), self.station.bssid());
        if let Some(reason) = leave_reason(&rx, &us, &bssid, self.pmf) {
            self.station.deassociate();
            self.left = Some(reason);
            return None;
        }
        let mut frame = rx.frame;
        if rx.decrypted {
            frame.extend_from_slice(&[0u8; MIC_LEN]);
        }
        match self.station.receive(&frame, rx.decrypted) {
            Rx::Ethernet(eth) => {
                if self.ready.len() >= RX_READY {
                    self.stats.rx_full = self.stats.rx_full.wrapping_add(1);
                } else {
                    self.stats.rx_eth = self.stats.rx_eth.wrapping_add(1);
                    self.ready.push_back(eth);
                }
                None
            }
            Rx::Handshake { frame, group_key } => {
                if group_key.is_some() {
                    self.new_group_key = group_key;
                    self.stats.rekeys = self.stats.rekeys.wrapping_add(1);
                }
                frame
            }
            // Management frames (beacons above all) are the common case.
            Rx::Dropped(RxDrop::Malformed) => None,
            Rx::Dropped(_) => {
                self.stats.rx_refused = self.stats.rx_refused.wrapping_add(1);
                None
            }
        }
    }

    /// The deauthentication to send when leaving, protected under the
    /// pairwise key when management frame protection is on.
    pub fn deauth(&mut self) -> Option<Vec<u8>> {
        if !self.station.is_associated() {
            return None;
        }
        let frame = deauth_frame(self.station.mac(), self.station.bssid(), 0, REASON_LEAVING);
        if self.pmf {
            self.station.protect_mgmt(&frame)
        } else {
            Some(frame)
        }
    }
}

/// The link as net_core drives it: the firmware and the open port.
pub struct Port<'p, 'a, 'd, M: Mmio, R: Region + ?Sized, C: Clock> {
    pub fw: &'p mut Fw<'a, 'd, M, R, C>,
    pub link: &'p mut Link,
}

impl<M: Mmio, R: Region + ?Sized, C: Clock> Port<'_, '_, '_, M, R, C> {
    /// Take what the firmware posted and answer what calls for it.
    pub fn service(&mut self) {
        self.fw.pump();
        while let Some(rx) = self.fw.inbox.frames.pop_front() {
            if let Some(reply) = self.link.receive(rx) {
                let rate = self.link.mgmt_rate;
                let _ = self.fw.send(Queue::Data, &reply, rate);
            }
        }
    }
}

impl<M: Mmio, R: Region + ?Sized, C: Clock> LinkPort for Port<'_, '_, '_, M, R, C> {
    fn mac(&self) -> Option<[u8; 6]> {
        self.link.station.is_associated().then(|| self.link.station.mac())
    }

    fn link_up(&self) -> bool {
        self.link.station.is_associated()
    }

    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize> {
        self.service();
        while let Some(eth) = self.link.ready.pop_front() {
            // A frame larger than the caller's buffer is dropped, never cut.
            if let Some(dst) = out.get_mut(..eth.len()) {
                dst.copy_from_slice(&eth);
                return Some(eth.len());
            }
            self.link.stats.rx_full = self.link.stats.rx_full.wrapping_add(1);
        }
        None
    }

    fn send_tx(&mut self, frame: &[u8]) -> bool {
        let sent = match self.link.station.tx_frame(frame) {
            Some(mpdu) => self.fw.send(Queue::Data, &mpdu, self.link.data_rate),
            None => false,
        };
        let s = &mut self.link.stats;
        if sent {
            s.tx_ok = s.tx_ok.wrapping_add(1);
        } else {
            s.tx_drop = s.tx_drop.wrapping_add(1);
        }
        sent
    }
}
