// NONOS Operating System (AGPL-3.0-or-later)

//! The service's gate and its privacy: the link protocol answers net.core's
//! owner alone (EPERM to anyone else); the card's factory address is never
//! used on the air or in a command; every serial line is one whole line
//! naming no address and no network.

use nonos_libc::take_lines;

use crate::model::mcu_legacy::FACTORY;
use crate::model::net::{joined, SSID};
use crate::model::wire::{netif, NET_CORE, PANEL};

#[test]
fn the_link_answers_net_core_alone() {
    let (mut st, _c) = joined();
    assert_eq!(netif(&mut st, PANEL, 2, &[]).0, -1, "EPERM to the panel");
    assert_eq!(netif(&mut st, NET_CORE, 2, &[]), (0, vec![1]));
}

fn holds(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

#[test]
fn the_factory_address_never_reaches_the_air_or_a_command() {
    let _ = take_lines();
    let (_st, c) = joined();
    let chip = c.borrow();
    assert!(!chip.sent.is_empty() && !chip.mcu.seen.is_empty());
    assert!(chip.sent.iter().all(|f| !holds(&f.frame, &FACTORY)), "a frame carried it");
    assert!(chip.mcu.seen.iter().all(|s| !holds(&s.body, &FACTORY)), "a command carried it");
}

fn spelled(a: &[u8]) -> Vec<String> {
    let pairs: Vec<String> = a.iter().map(|b| format!("{b:02x}")).collect();
    let all = [pairs.join(":"), pairs.join("-"), pairs.concat()];
    all.iter().flat_map(|s| [s.clone(), s.to_uppercase()]).collect()
}

#[test]
fn every_line_is_whole_and_names_no_address_or_network() {
    let _ = take_lines();
    let (st, c) = joined();
    let ap = c.borrow().air.as_ref().unwrap().ap.bssid;
    let own = nonos_wifi_core::softmac::Radio::mac(st.radio().unwrap());
    let lines = take_lines();
    assert!(lines.len() >= 3, "{lines:?}");
    let ssid = String::from_utf8_lossy(SSID).into_owned();
    for l in &lines {
        assert!(l.starts_with("[mt7921] ") && l.ends_with('\n'), "{l:?}");
        assert_eq!(l.matches('\n').count(), 1, "one line per write: {l:?}");
        assert!(!l.contains(&ssid), "{l:?}");
        for a in [FACTORY, ap, own] {
            assert!(spelled(&a).iter().all(|s| !l.contains(s.as_str())), "{l:?}");
        }
    }
}
