// NONOS Operating System (AGPL-3.0-or-later)

//! The PSK network that requires management frame protection, and its
//! handshake as the reference frames it.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};

use super::vectors::{h, AA};

// PSK only, MFPC and MFPR set; the station answers with the same bits.
pub(super) const AP_RSNE_PROTECTED: &str = "30140100000fac040100000fac040100000fac02cc00";
pub(super) const OWN_RSNE_PROTECTED: &str = "30140100000fac040100000fac040100000fac02c000";

// Message 2 to the reference's message 1, carrying the station's RSNE.
pub(super) const M2_PROTECTED: &str =
    "0203007502010a0000000000000000000111111111111111111111111111111111111111
     111111111111111111111111110000000000000000000000000000000000000000000000
     0000000000000000006967e07e8bb52e0dbac95d3732c8cfc9001630140100000fac0401
     00000fac040100000fac02c000";

// Message 3: replay 2, key data wrapping the RSNE, a GTK KDE and an IGTK KDE.
pub(super) const M3_PROTECTED: &str =
    "020300b70213ca00100000000000000002202122232425262728292a2b2c2d2e2f303132
     333435363738393a3b3c3d3e3f0000000000000000000000000000000005070000000000
     0000000000000000000d2e0b7319f60ddf39fe1d6160e8857b0058a6400fb441baf425dd
     e4876d18810e1ebf1a7f8f72d30de0e4c1956837c95bf7abbeeaa45b7a92fde4b3c31dea
     cf86f9efca197eb166d35cfe13dcb3f6b95eb7e8d48ffb2a09f38b26979d956559460af0
     223f49ebfb4975";

// The network's beacon on SSID "IEEE", channel 6.
pub(super) fn beacon() -> Vec<u8> {
    let mut beacon = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut beacon, frame_control(TYPE_MGMT, 8), [0xFF; 6], AA, AA, 0).unwrap();
    beacon.extend_from_slice(&[0u8; 8]);
    beacon.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    beacon.extend_from_slice(&[0, 4, b'I', b'E', b'E', b'E']);
    beacon.extend_from_slice(&[1, 8, 0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24]);
    beacon.extend_from_slice(&[3, 1, 6]);
    beacon.extend_from_slice(&h(AP_RSNE_PROTECTED));
    beacon
}
