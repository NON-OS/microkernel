// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the scan result collector: it deduplicates by access point, drops
//! hidden networks, caps the list, and encodes in the exact `[count]` then
//! per-network `[signal][flags][ssid_len][ssid]` format the settings panel
//! parses. A real beacon, decoded by the shared `parse_beacon`, becomes a result
//! end to end. The channel-hop and ring reads are hardware timing left for the
//! on-silicon session; the collection and encoding are checked here.

use crate::scan::{beacon_flags, ScanResults};
use nonos_wifi_core::scan_list::{FLAG_WPA2, FLAG_WPA3, MAX_AGE, MAX_RESULTS};
use nonos_wifi_core::dot11::parse::parse_beacon;

// Build a beacon frame that starts at the MAC header: broadcast destination, the
// given BSSID, the fixed fields, an SSID element and, when secured, an RSN one.
fn beacon(bssid: [u8; 6], ssid: &[u8], secured: bool) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&[0x80, 0x00]); // frame control: beacon
    f.extend_from_slice(&[0x00, 0x00]); // duration
    f.extend_from_slice(&[0xff; 6]); // addr1: broadcast
    f.extend_from_slice(&bssid); // addr2
    f.extend_from_slice(&bssid); // addr3 (BSSID)
    f.extend_from_slice(&[0x00, 0x00]); // sequence
    f.extend_from_slice(&[0u8; 8]); // timestamp
    f.extend_from_slice(&[0x64, 0x00]); // beacon interval
    f.extend_from_slice(&[0x11, 0x04]); // capability
    f.push(0); // SSID element id
    f.push(ssid.len() as u8);
    f.extend_from_slice(ssid);
    if secured {
        f.push(48); // RSN element id
        f.push(2);
        f.extend_from_slice(&[0x01, 0x00]);
    }
    f
}

#[test]
fn add_dedupes_by_access_point_and_skips_hidden() {
    let mut r = ScanResults::new();
    let ap = [0x02, 0, 0, 0, 0, 1];
    r.add(ap, b"Home", true);
    r.add(ap, b"Home", true); // same AP again
    r.add([0x02, 0, 0, 0, 0, 2], b"Cafe", false);
    r.add([0x02, 0, 0, 0, 0, 3], b"", false); // hidden
    assert_eq!(r.count(), 2, "the repeat and the hidden network are dropped");
}

#[test]
fn add_caps_at_the_maximum() {
    let mut r = ScanResults::new();
    for i in 0..(MAX_RESULTS as u8 + 5) {
        r.add([0x02, 0, 0, 0, 0, i], b"Net", false);
    }
    assert_eq!(r.count(), MAX_RESULTS, "the list stops at the cap");
}

#[test]
fn encode_matches_the_panel_format() {
    let mut r = ScanResults::new();
    r.add([0x02, 0, 0, 0, 0, 1], b"Home", true);
    r.add([0x02, 0, 0, 0, 0, 2], b"Cafe", false);
    let mut out = [0u8; 128];
    let n = r.encode(&mut out);
    assert_eq!(out[0], 2, "count leads");
    // First network: signal 0, flags secured, ssid_len 4, "Home".
    assert_eq!(out[1], 0, "signal byte");
    assert_eq!(out[2], 0x01, "secured flag set");
    assert_eq!(out[3], 4, "ssid length");
    assert_eq!(&out[4..8], b"Home");
    // Second network follows immediately: signal, flags clear, len 4, "Cafe".
    assert_eq!(out[8], 0, "signal byte");
    assert_eq!(out[9], 0x00, "open network flag clear");
    assert_eq!(out[10], 4);
    assert_eq!(&out[11..15], b"Cafe");
    assert_eq!(n, 15, "count byte plus two 7-byte entries");
}

#[test]
fn a_parsed_beacon_becomes_a_result() {
    let frame = beacon([0x02, 0xAB, 0xCD, 0xEF, 0x00, 0x01], b"TestNet", true);
    let info = parse_beacon(&frame).expect("the beacon parses");
    let mut r = ScanResults::new();
    r.add(info.bssid, info.ssid, info.rsn);
    assert_eq!(r.count(), 1);
    let mut out = [0u8; 64];
    r.encode(&mut out);
    assert_eq!(out[0], 1);
    assert_eq!(out[2] & 0x01, 0x01, "the RSN beacon is reported secured");
    assert_eq!(out[3], 7, "SSID length of TestNet");
    assert_eq!(&out[4..11], b"TestNet");
}

// A beacon with `rsn` as its RSN element body (when given) and the capability
// field `cap`.
fn beacon_rsn(rsn: Option<&[u8]>, cap: u16) -> Vec<u8> {
    let mut f = beacon([0x02, 0, 0, 0, 0, 9], b"Net", false);
    f[34..36].copy_from_slice(&cap.to_le_bytes());
    if let Some(body) = rsn {
        f.push(48);
        f.push(body.len() as u8);
        f.extend_from_slice(body);
    }
    f
}

// An RSN element body: CCMP group and pairwise, the given AKM suite types.
fn rsn_body(akms: &[u8]) -> Vec<u8> {
    let mut b = vec![0x01, 0x00, 0x00, 0x0F, 0xAC, 0x04, 0x01, 0x00, 0x00, 0x0F, 0xAC, 0x04];
    b.extend_from_slice(&[akms.len() as u8, 0x00]);
    for &a in akms {
        b.extend_from_slice(&[0x00, 0x0F, 0xAC, a]);
    }
    b.extend_from_slice(&[0x80, 0x00]);
    b
}

#[test]
fn beacon_flags_name_the_personal_security_on_offer() {
    let wpa2 = beacon_flags(&beacon_rsn(Some(&rsn_body(&[2])), 0x0411), 0x0411);
    assert_eq!(wpa2, 0x01 | FLAG_WPA2, "PSK: secured, WPA2");
    let sha = beacon_flags(&beacon_rsn(Some(&rsn_body(&[6])), 0x0411), 0x0411);
    assert_eq!(sha, 0x01 | FLAG_WPA2, "PSK-SHA256 is WPA2-Personal too");
    let wpa3 = beacon_flags(&beacon_rsn(Some(&rsn_body(&[8])), 0x0411), 0x0411);
    assert_eq!(wpa3, 0x01 | FLAG_WPA3, "SAE only: WPA3");
    let both = beacon_flags(&beacon_rsn(Some(&rsn_body(&[2, 8])), 0x0411), 0x0411);
    assert_eq!(both, 0x01 | FLAG_WPA2 | FLAG_WPA3, "a transition network offers both");
    let ent = beacon_flags(&beacon_rsn(Some(&rsn_body(&[1])), 0x0411), 0x0411);
    assert_eq!(ent, 0x01, "Enterprise is secured but neither personal kind");
}

#[test]
fn beacon_flags_without_a_usable_rsn_element_fall_back_to_the_privacy_bit() {
    assert_eq!(beacon_flags(&beacon_rsn(None, 0x0001), 0x0001), 0, "open");
    assert_eq!(beacon_flags(&beacon_rsn(None, 0x0011), 0x0011), 0x01, "privacy bit: secured");
    // A truncated RSN element (an AKM count running past the element) is not
    // read as offering anything.
    let mut cut = rsn_body(&[2, 8]);
    cut.truncate(16);
    assert_eq!(beacon_flags(&beacon_rsn(Some(&cut), 0x0001), 0x0001), 0, "malformed: no claim");
    assert_eq!(beacon_flags(&[0x80, 0x00, 0x00], 0), 0, "a runt frame is open, not a panic");
}

#[test]
fn a_network_heard_again_is_refreshed_with_its_current_flags() {
    let mut r = ScanResults::new();
    let ap = [0x02, 0, 0, 0, 0, 1];
    r.add_bss(ap, b"Home", 0x01 | FLAG_WPA2);
    r.add_bss(ap, b"Home", 0x01 | FLAG_WPA2 | FLAG_WPA3);
    assert_eq!(r.count(), 1);
    let mut out = [0u8; 16];
    r.encode(&mut out);
    assert_eq!(out[2], 0x01 | FLAG_WPA2 | FLAG_WPA3, "the flags follow the latest beacon");
}

#[test]
fn a_network_unheard_for_max_age_sweeps_is_dropped() {
    let mut r = ScanResults::new();
    r.add_bss([0x02, 0, 0, 0, 0, 1], b"Gone", 0);
    r.add_bss([0x02, 0, 0, 0, 0, 2], b"Here", 0);
    for _ in 0..MAX_AGE {
        r.end_sweep();
        r.add_bss([0x02, 0, 0, 0, 0, 2], b"Here", 0);
    }
    assert_eq!(r.count(), 2, "MAX_AGE sweeps unheard is still listed");
    r.end_sweep();
    assert_eq!(r.count(), 1, "one more and it is gone");
    let mut out = [0u8; 16];
    r.encode(&mut out);
    assert_eq!(&out[4..8], b"Here", "the one still heard stays");
}

#[test]
fn a_full_list_makes_room_only_by_replacing_a_network_from_an_earlier_sweep() {
    let mut r = ScanResults::new();
    for i in 0..MAX_RESULTS as u8 {
        r.add_bss([0x02, 0, 0, 0, 0, i], b"Old", 0);
    }
    // In the same sweep the seventeenth is not listed: nothing is stale yet.
    r.add_bss([0x02, 0, 0, 0, 1, 0], b"New", 0);
    let mut out = [0u8; 256];
    let n = r.encode(&mut out);
    assert!(!out[..n].windows(3).any(|w| w == b"New"), "no network from this sweep is evicted");
    // A sweep later every listed network is older than one heard now.
    r.end_sweep();
    r.add_bss([0x02, 0, 0, 0, 1, 0], b"New", 0);
    assert_eq!(r.count(), MAX_RESULTS);
    let n = r.encode(&mut out);
    assert!(out[..n].windows(3).any(|w| w == b"New"), "the new network replaced a stale one");
}

#[test]
fn an_overlong_ssid_is_not_listed() {
    let mut r = ScanResults::new();
    r.add_bss([0x02, 0, 0, 0, 0, 1], &[b'x'; 33], 0);
    assert_eq!(r.count(), 0, "an SSID over 32 octets is malformed");
}
