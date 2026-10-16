// NONOS Operating System (AGPL-3.0-or-later)

//! The addresses and beacons the join proofs use.

pub(super) const OUR: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
pub(super) const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 0xAA];

// Build a beacon frame for `ssid` advertising RSN, the shape parse_beacon accepts.
pub(super) fn beacon_for(ssid: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&[0x80, 0x00]); // frame control: beacon
    f.extend_from_slice(&[0, 0]); // duration
    f.extend_from_slice(&[0xFF; 6]); // DA broadcast
    f.extend_from_slice(&AP); // SA
    f.extend_from_slice(&AP); // BSSID
    f.extend_from_slice(&[0, 0]); // seq
    f.extend_from_slice(&[0u8; 8]); // timestamp
    f.extend_from_slice(&[0x64, 0]); // beacon interval
    f.extend_from_slice(&[0x11, 0x04]); // capability: ESS + privacy
                                        // SSID element.
    f.push(0);
    f.push(ssid.len() as u8);
    f.extend_from_slice(ssid);
    // DS parameter set: channel 6.
    f.extend_from_slice(&[3, 1, 6]);
    // RSN element (id 48), a minimal WPA2-PSK/CCMP body.
    let rsn: [u8; 20] = [
        0x01, 0x00, // version
        0x00, 0x0F, 0xAC, 0x04, // group cipher CCMP
        0x01, 0x00, 0x00, 0x0F, 0xAC, 0x04, // pairwise CCMP
        0x01, 0x00, 0x00, 0x0F, 0xAC, 0x02, // akm PSK
        0x00, 0x00, // rsn caps
    ];
    f.push(48);
    f.push(rsn.len() as u8);
    f.extend_from_slice(&rsn);
    f
}

// `beacon_for`'s beacon with the RSN element's AKM suite type replaced and
// management frame protection capable (WPA3 requires it).
pub(super) fn beacon_with_akm(akm: u8) -> Vec<u8> {
    let mut b = beacon_for(b"net");
    let at = b.len() - 3; // the AKM suite type, before the two capability octets
    b[at] = akm;
    b[at + 1] = 0x80; // RSN capabilities: MFPC
    b
}
