// NONOS Operating System (AGPL-3.0-or-later)
//! Channel set (`rtw8852b_set_channel` inside `rtw89_set_channel_help`):
//! RF path A lands on the channel asked, in its band, at 20 MHz; the MAC's
//! queues are stopped over the firmware mailbox and given back as they
//! were; 6 GHz and anything outside 1-14 and 36-177 is refused.

use crate::chan::Chan;
use crate::model::{rf_tuned, rig};
use crate::radio::Station;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

const FIVE: [u8; 25] = [
    36, 40, 44, 48, 52, 56, 60, 64, 100, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140, 144,
    149, 153, 157, 161, 165,
];

#[test]
fn every_channel_lands_on_rf_path_a_at_20_mhz() {
    let (card, mut chip) = rig::ready();
    let all = (1..=14).map(|c| (false, c)).chain(FIVE.iter().map(|&c| (true, c)));
    for (five, ch) in all {
        chip.tune_to(Chan::new(five, ch).unwrap()).unwrap();
        let s = card.0.borrow();
        assert_eq!(rf_tuned(&s), Some((five, ch)));
        let v = s.rf[0][0x18];
        assert_eq!((v >> 10) & 3, 3, "20 MHz");
        assert_eq!(v & 0x1_0000 != 0, five, "band bit");
        assert_eq!(s.rf[1][0x18] & 0x1ff, v & 0x1ff, "path B with it");
    }
    assert_eq!(card.0.borrow().violations, Vec::<String>::new());
}

#[test]
fn the_queues_are_stopped_for_the_switch_and_given_back() {
    let (card, mut chip) = rig::ready();
    let n = card.0.borrow().mailbox.len();
    chip.tune_to(Chan::new(false, 11).unwrap()).unwrap();
    let s = card.0.borrow();
    let m = &s.mailbox[n..];
    assert_eq!(m.len(), 2);
    assert_eq!((m[0][0] & 0x7f, m[0][0] >> 16, m[0][1] & 0xffff), (5, 0, 0xffff), "stop all");
    assert_eq!((m[1][0] & 0x7f, m[1][0] >> 16), (5, 0xffff), "back as they were");
    assert_eq!(s.word(0xc348) & 0xffff, 0xffff);
}

#[test]
fn a_switch_the_firmware_does_not_answer_fails_and_is_counted() {
    let (card, chip) = rig::ready();
    let mut sta = Station::new(chip);
    card.0.borrow_mut().faults.mute = true;
    assert!(!sta.tune(Band::Ghz2, 6));
    assert_eq!(sta.counts.tune_fail, 1);
    // Still on dm_init's default channel, and the driver claims no channel.
    assert_eq!(rf_tuned(&card.0.borrow()), Some((false, 1)), "nothing changed on the RF");
    assert_eq!(sta.chip.chan, None);
}

#[test]
fn channels_outside_the_bands_are_refused() {
    for (five, ch) in [(false, 0), (false, 15), (true, 35), (true, 178)] {
        assert_eq!(Chan::new(five, ch), None);
    }
    let (_, chip) = rig::ready();
    let mut sta = Station::new(chip);
    assert!(!sta.tune(Band::Ghz6, 5), "no 6 GHz");
    assert!(!sta.tune(Band::Ghz2, 36));
    assert_eq!(sta.counts.tune_fail, 0, "a refusal is not a chip failure");
}
