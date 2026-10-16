// NONOS Operating System (AGPL-3.0-or-later)

//! The reference messages and keys, decoded from hex.

use nonos_wifi_core::wpa::ptk::pmk;

pub(super) fn h(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

// The handshake both scenarios run: the J.4.2 PMK, an AA above the SPA and an
// ANonce above the SNonce, so the Min/Max ordering of the PTK input is tested.
pub(super) const AA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x01, 0x00];
pub(super) const SPA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];

pub(super) const SNONCE: [u8; 32] = [0x11; 32];

pub(super) const PTK: &str = "df3e300cab6142d40c893497162867f0d8a066384aaee26a5de3066ad63d36e8
                   30ab7cf57681b9d485e1fc58a9864986";

pub(super) fn anonce() -> [u8; 32] {
    core::array::from_fn(|i| 0x20 + i as u8)
}

pub(super) fn psk_ieee() -> [u8; 32] {
    pmk(b"password", b"IEEE")
}

// Message 1: version 2, Pairwise | Ack, key length 16, replay counter 1.
pub(super) const M1: &str = "0203005f02008a00100000000000000001202122232425262728292a2b2c2d2e2f
                  303132333435363738393a3b3c3d3e3f000000000000000000000000000000000000
                  00000000000000000000000000000000000000000000000000000000000000000000";

// A WPA2/WPA3 transition network: PSK and SAE, MFPC set, MFPR clear, the
// group management cipher named, and an RSNXE advertising hash-to-element.
pub(super) const AP_RSNE_TRANSITION: &str =
    "301e0100000fac040100000fac040200000fac02000fac0880000000000fac06";
pub(super) const AP_RSNXE: &str = "f40120";

// The transition network's handshake as the reference frames it. Message 2:
// header length 0x75, Key Information 0x010a, key length 0, MIC, the RSNE.
pub(super) const M2_TRANSITION: &str = "0203007502010a0000000000000000000111111111111111111111111111111111111111
                             111111111111111111111111110000000000000000000000000000000000000000000000
                             0000000000000000004af97ec63ec8b241115b93068c3bf2d4001630140100000fac0401
                             00000fac040100000fac020000";

// Message 3: Install | Ack | MIC | Secure | Encrypted, replay 2, RSC 0x0705,
// key data wrapping the beacon's RSNE, its RSNXE, a GTK KDE (index 2) and the
// 0xDD 00 padding.
pub(super) const M3_TRANSITION: &str = "020300a70213ca00100000000000000002202122232425262728292a2b2c2d2e2f303132
                             333435363738393a3b3c3d3e3f0000000000000000000000000000000005070000000000
                             000000000000000000949f297155d8e84760161b3a11bb571a00489d01f0fa9ec3810983
                             afbb3624fea4bd42005fa80bcbaf2fd9c470a9572f92c3072e08416553f5fa9326bc3612
                             dd88b12feeae341c6532e41ea19de6011f1536aeae929467a8a6fa";

// Message 4: Key Information 0x030a, replay 2, zero nonce, MIC, no key data.
pub(super) const M4_TRANSITION: &str = "0203005f02030a0000000000000000000200000000000000000000000000000000000000
                             000000000000000000000000000000000000000000000000000000000000000000000000
                             00000000000000000051941757713f30e2c0c22b17c44dd6050000";
