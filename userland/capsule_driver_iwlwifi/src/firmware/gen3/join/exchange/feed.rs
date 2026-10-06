// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! A received frame fed to the MLME, counted for the connect reply; the
//! access point's EAPOL in the clear is the only data frame it reads.

use alloc::vec::Vec;

use nonos_wifi_core::dot11::header::{TYPE_DATA, TYPE_MGMT};
use nonos_wifi_core::mlme::Mlme;

use super::super::super::rx_data::RxMpdu;
use super::air::{frame_type, Air};
use super::eapol::{eapol_frame, eapol_payload};

impl Air {
    // Feed one received frame to the MLME; the frame to send back, if any.
    pub fn feed(&mut self, mlme: &mut Mlme, rx: &RxMpdu) -> Option<Vec<u8>> {
        self.p.recv = self.p.recv.saturating_add(1);
        let f = &rx.frame;
        match frame_type(f) {
            Some(TYPE_MGMT) => {
                if f.first().is_some_and(|b| *b == 0xC0 || *b == 0xA0) {
                    self.p.deauth = self.p.deauth.saturating_add(1);
                }
                mlme.on_mgmt(f).tx
            }
            Some(TYPE_DATA) => {
                self.p.data = self.p.data.saturating_add(1);
                let (us, bssid) = (mlme.our_mac(), mlme.bssid());
                if f.get(4..10) == Some(&us[..]) {
                    self.p.to_us = self.p.to_us.saturating_add(1);
                }
                let Some(eapol) = eapol_payload(f, &us, &bssid) else {
                    if self.p.probe == 0 && f.len() >= 2 {
                        self.p.probe = u32::from(u16::from_le_bytes([f[0], f[1]]))
                            | (f.len().min(0xFFFF) as u32) << 16;
                    }
                    return None;
                };
                self.p.eapol = self.p.eapol.saturating_add(1);
                let reply = mlme.on_eapol(eapol).tx?;
                let frame = eapol_frame(&reply, us, bssid, self.seq)?;
                self.seq = self.seq.wrapping_add(1) & 0x0FFF;
                Some(frame)
            }
            _ => None,
        }
    }
}
