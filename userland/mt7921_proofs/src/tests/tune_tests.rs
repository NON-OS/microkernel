// NONOS Operating System (AGPL-3.0-or-later)

//! Tuning on the model: a channel is the radio's once its request is
//! granted with the request's own token; an ungranted one is aborted and
//! the radio is nowhere; a band the card lacks or 6 GHz is refused; a join
//! holds its channel by a `JOIN` request, renewed before it lapses.

use nonos_libc::mk_idle_ms;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

use crate::model::rig::up;

fn roc_bodies(c: &crate::model::Shared) -> Vec<Vec<u8>> {
    c.borrow().mcu.uni(0x27).iter().map(|s| s.body.clone()).collect()
}

#[test]
fn a_channel_is_had_once_its_own_token_is_granted() {
    let (mut r, c) = up();
    assert!(r.tune(Band::Ghz2, 6));
    assert!(r.tune(Band::Ghz5, 36));
    assert_eq!(c.borrow().mcu.roc, Some(36));
    let b = roc_bodies(&c);
    let tags: Vec<(u8, u8)> = b.iter().map(|x| (x[4], x[9])).collect();
    assert_eq!(tags, [(0, 1), (1, 1), (0, 2)], "acquire 1, its abort, acquire 2");
    assert_eq!((b[0][19], b[2][12]), (1, 2), "a listen; the second on 5 GHz");
}

#[test]
fn an_ungranted_request_is_aborted_and_leaves_the_radio_nowhere() {
    let (mut r, c) = up();
    c.borrow_mut().faults.no_grant = true;
    assert!(!r.tune(Band::Ghz2, 11));
    assert_eq!(c.borrow().mcu.roc, None, "aborted");
    assert_eq!(r.diag()[2], 1, "counted as missed");
    assert!(!r.tune(Band::Ghz6, 5));
}

#[test]
fn a_join_holds_its_channel_by_a_join_request_renewed_before_it_lapses() {
    let (mut r, c) = up();
    assert!(r.tune(Band::Ghz2, 1));
    r.set_bss(Some([0x0a; 6]));
    let b = roc_bodies(&c);
    assert_eq!(b.last().unwrap()[19], 0, "MT7921_ROC_REQ_JOIN");
    let before = roc_bodies(&c).len();
    mk_idle_ms(500);
    r.service(nonos_libc::now());
    assert_eq!(roc_bodies(&c).len(), before, "not yet near its end");
    mk_idle_ms(300);
    r.service(nonos_libc::now());
    let after = roc_bodies(&c);
    assert_eq!(after.len(), before + 2, "aborted and asked again");
    assert_eq!((after.last().unwrap()[10], after.last().unwrap()[19]), (1, 0));
}
