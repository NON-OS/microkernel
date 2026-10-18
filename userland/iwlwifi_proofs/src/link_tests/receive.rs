// NONOS Operating System (AGPL-3.0-or-later)

//! A received notification's MPDU becomes the Ethernet frame again.

use super::frames::{BSSID, LLC_SNAP, OUR, RX_DESC_LEN};
use crate::gen3::extract_mpdu;
use nonos_wifi_core::station::{Ccmp, LinkStation};

#[test]
fn receive_extracts_the_mpdu_and_recovers_the_ethernet_frame() {
    // A hardware-CCMP chip delivers an already-decrypted MPDU after a descriptor.
    // Build the FromDS data frame the AP sends the station.
    let src: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x09];
    let payload = b"a reply from the access point";
    let fc: u16 = 0x0208; // data frame, FromDS
    let mut mpdu = Vec::new();
    mpdu.extend_from_slice(&fc.to_le_bytes());
    mpdu.extend_from_slice(&[0, 0]); // duration
    mpdu.extend_from_slice(&OUR); // addr1 = receiver (station)
    mpdu.extend_from_slice(&BSSID); // addr2 = transmitter (AP)
    mpdu.extend_from_slice(&src); // addr3 = source
    mpdu.extend_from_slice(&[0, 0]); // sequence control
    mpdu.extend_from_slice(&LLC_SNAP);
    mpdu.extend_from_slice(&[0x08, 0x00]); // ethertype IPv4
    mpdu.extend_from_slice(payload);

    // The firmware posts the frame after a descriptor whose first field is the
    // MPDU length.
    let mut notif = vec![0u8; RX_DESC_LEN];
    notif[0..2].copy_from_slice(&(mpdu.len() as u16).to_le_bytes());
    notif.extend_from_slice(&mpdu);

    let extracted = extract_mpdu(&notif, RX_DESC_LEN).expect("extract the MPDU after the descriptor");
    assert_eq!(extracted, &mpdu[..], "the MPDU is recovered from the notification");

    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    let eth = sta.rx_frame(extracted).expect("recover the ethernet frame");
    assert_eq!(&eth[0..6], &OUR, "destination is the station (FromDS addr1)");
    assert_eq!(&eth[6..12], &src, "source is addr3");
    assert_eq!(&eth[12..14], &[0x08, 0x00], "ethertype preserved");
    assert_eq!(&eth[14..], payload, "payload preserved end to end");
}
