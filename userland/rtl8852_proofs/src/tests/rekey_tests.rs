// NONOS Operating System (AGPL-3.0-or-later)
//! The chip's keys over a link's life on the model: a group rekey's key
//! goes into its own security CAM entry and slot, message 2 out for the
//! chip to seal, and group data under the new key comes up; and no key is
//! left in host memory the card reaches, or named on the serial line.

use super::crypt_tests::{hear, joined};
use crate::ap_sim::{group1, gtk_kde, GTK};
use crate::soft::data_frames::{eapol_from, from_ds_of};
use nonos_wifi_core::dot11::ccmp::encrypt;
use nonos_wifi_core::wpa::akm::Akm;

const NEW_GTK: [u8; 16] = [0x5a; 16];

#[test]
fn a_group_rekey_puts_the_new_key_in_the_chip_and_group_data_comes_up() {
    let (card, mut st, tk) = joined();
    let (sta, bssid, ptk) = {
        let s = card.0.borrow();
        let ap = &s.air.as_ref().unwrap().ap;
        (s.cam.unwrap().0, ap.bssid, ap.ptk.clone().unwrap().1)
    };
    let m1 = eapol_from(bssid, sta, &group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &NEW_GTK)));
    card.0.borrow_mut().air.as_mut().unwrap().inbox.push_back(encrypt(&m1, 9, &tk).unwrap());
    st.tick();
    {
        let s = card.0.borrow();
        assert_eq!((s.sec.entries[&3], s.sec.slots[3]), ((6, NEW_GTK), Some((3, 2))));
        let m2 = s.sent.last().unwrap();
        assert_eq!(m2.wd[8], 6 << 9 | 1 << 8, "message 2 for the chip to seal");
        let got = s.air.as_ref().unwrap().ap.got.last().unwrap().clone();
        assert!(got.starts_with(&[0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e]), "{got:?}");
    }
    let llc = [0xaa, 0xaa, 3, 0, 0, 0, 8, 0, b'g'];
    let mut f = encrypt(&from_ds_of(bssid, [0xff; 6], &llc), 1, &NEW_GTK).unwrap();
    f[27] = 0x20 | 2 << 6;
    card.0.borrow_mut().air.as_mut().unwrap().inbox.push_back(f);
    st.tick();
    let up = super::station::netif(&mut st, 5, &[]).1;
    assert!(up.ends_with(b"g"), "group data under the new key: {up:?}");
    assert!(hear(&card, &mut st, b"still", 10, &tk, 0).ends_with(b"still"));
}

#[test]
fn no_key_is_left_in_host_memory_or_named_on_the_line() {
    let _ = nonos_libc::take_lines();
    let (card, _st, tk) = joined();
    let s = card.0.borrow();
    for m in &s.bus {
        let b = m.buf.borrow();
        assert!(!b.windows(16).any(|w| w == tk || w == GTK), "a key outlived its command");
    }
    let lines = nonos_libc::take_lines();
    assert!(lines.iter().any(|l| l.ends_with(b"pairwise key in the chip\n")), "{lines:?}");
    let hex: String = tk.iter().map(|b| format!("{b:02x}")).collect();
    for l in &lines {
        let text = String::from_utf8_lossy(l).to_lowercase();
        assert!(!text.contains(&hex) && !l.windows(16).any(|w| w == tk), "{text}");
    }
}
