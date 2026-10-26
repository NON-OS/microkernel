// NONOS Operating System (AGPL-3.0-or-later)

//! One received frame through the state machine.

use super::classify::{classify, fc_type, RxKind};
use super::eapol::{eapol_from_bss, wrap_eapol};
use super::facts::granted_aid;
use super::radio::Counters;
use alloc::vec::Vec;
use nonos_wifi_core::dot11::header::TYPE_DATA;
use nonos_wifi_core::mlme::Mlme;

// Feed one received frame to the machine and return the frame to transmit next,
// if any (an EAPOL reply is wrapped into an 802.11 data frame, numbered when it
// is sent). Also counts the data and EAPOL frames seen, for diagnosis.
pub(super) fn handle_frame(
    mlme: &mut Mlme,
    rx: &[u8],
    eth: &mut Vec<u8>,
    c: &mut Counters,
) -> Option<Vec<u8>> {
    match classify(rx, eth) {
        RxKind::Mgmt => {
            // A deauthenticate (subtype 0xC) or disassociate (0xA) means the AP
            // set the connection up then dropped it, which is what it does after
            // sending an EAPOL frame it never gets a reply to.
            if !rx.is_empty() {
                let subtype = rx[0] & 0xF0;
                if subtype == 0xC0 || subtype == 0xA0 {
                    c.deauth += 1;
                }
            }
            if let Some(aid) = granted_aid(rx, mlme.bssid()) {
                c.aid = aid;
            }
            mlme.on_mgmt(rx).tx
        }
        // Only the AP's own EAPOL to this station, in the clear, is the
        // handshake; another BSS's or another station's is not ours to answer.
        RxKind::Eapol(_, _) if !eapol_from_bss(rx, &mlme.our_mac(), &mlme.bssid()) => None,
        RxKind::Eapol(start, end) => {
            c.eapol += 1;
            let reply = mlme.on_eapol(&eth[start..end]).tx?;
            wrap_eapol(&reply, mlme.our_mac(), mlme.bssid(), 0)
        }
        RxKind::Other => {
            // Count any data frame the card delivered, EAPOL or not, so an empty
            // EAPOL count can be told apart from no data reaching us at all. For
            // the probe, prefer the first UNPROTECTED data frame (the shape an
            // EAPOL msg1 arrives in), upgrading from an earlier protected capture:
            // the access point's encrypted group traffic is protected, so if the
            // probe stays protected no plaintext EAPOL ever reached us, whereas an
            // unprotected fc points the finger at the parser instead.
            if fc_type(rx) == Some(TYPE_DATA) && rx.len() >= 2 {
                c.data += 1;
                // Receiver address (addr1) is bytes 4..10. Count frames unicast to
                // our own MAC: the EAPOL handshake arrives that way.
                if rx.len() >= 10 && rx[4..10] == mlme.our_mac() {
                    c.to_us += 1;
                }
                let fc = u16::from_le_bytes([rx[0], rx[1]]);
                let unprotected = fc & 0x4000 == 0;
                let have_unprotected = c.probe != 0 && (c.probe & 0x4000) == 0;
                if c.probe == 0 || (unprotected && !have_unprotected) {
                    c.probe = fc as u32 | ((rx.len() as u32 & 0xFFFF) << 16);
                }
            }
            None
        }
    }
}
