// NONOS Operating System (AGPL-3.0-or-later)

//! The connect encoder round-trips and clamps an oversized SSID.

use crate::wifi::network::SSID_MAX;
use crate::wifi::wire::encode_connect;

#[test]
fn connect_request_encodes_ssid_length_ssid_then_passphrase() {
    let req = encode_connect(b"home", b"secret12");
    assert_eq!(req[0], 4, "ssid length");
    assert_eq!(&req[1..5], b"home");
    assert_eq!(&req[5..], b"secret12", "passphrase follows");
}

#[test]
fn connect_request_for_an_open_network_has_no_passphrase() {
    let req = encode_connect(b"guest", b"");
    assert_eq!(req[0], 5);
    assert_eq!(&req[1..], b"guest", "no passphrase bytes for an open network");
}

#[test]
fn connect_request_clamps_an_oversized_ssid() {
    let ssid = [b'a'; 40];
    let req = encode_connect(&ssid, b"pw");
    assert_eq!(req[0] as usize, SSID_MAX, "the SSID length is clamped to the maximum");
    assert_eq!(req.len(), 1 + SSID_MAX + 2, "only the clamped SSID and the passphrase are sent");
}
