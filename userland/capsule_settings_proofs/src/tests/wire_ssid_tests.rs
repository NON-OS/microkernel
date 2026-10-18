// NONOS Operating System (AGPL-3.0-or-later)

//! An SSID length running past the buffer or over the maximum is refused;
//! one at the maximum is taken.

use super::wire_rig::*;
use crate::wifi::network::SSID_MAX;

#[test]
fn an_ssid_running_past_the_buffer_is_refused() {
    // ssid_len = 10 but only 3 bytes follow.
    let buf = vec![1u8, 100, 0, 10, b'a', b'b', b'c'];
    assert_eq!(collect(&buf).len(), 0, "an SSID past the end is not read");
}

#[test]
fn an_oversized_ssid_length_is_refused() {
    // ssid_len = 40 exceeds SSID_MAX (32); refuse rather than truncate silently.
    let mut buf = vec![1u8, 100, 0, 40];
    buf.extend([b'x'; 40]);
    assert_eq!(collect(&buf).len(), 0, "an over-long SSID is rejected");
}

#[test]
fn a_maximum_length_ssid_is_accepted() {
    let ssid = [b'z'; SSID_MAX];
    let mut buf = vec![1u8];
    buf.extend(entry(255, true, &ssid));
    let nets = collect(&buf);
    assert_eq!(nets.len(), 1);
    assert_eq!(nets[0].ssid(), &ssid[..]);
}
