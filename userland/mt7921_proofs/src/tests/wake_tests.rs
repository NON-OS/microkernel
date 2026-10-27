// NONOS Operating System (AGPL-3.0-or-later)

//! The card kept the driver's (`power::wake`, after `mt76_connac_pm_wake`,
//! `mt792xe_mcu_drv_pmctrl` and `mt792x_wpdma_reinit_cond`): when its
//! firmware takes it back, the next command, frame or service pass takes it
//! again, and re-arms the rings if the WFDMA forgot them; and a card a warm
//! reboot left behind (firmware own held, patch applied, firmware running,
//! WFDMA busy) still comes up.

use nonos_libc::{mk_idle_ms, now, take_lines};
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

use crate::model::doze::DUMMY_CR;
use crate::model::net::joined;
use crate::model::rig::{bring_up, chip, up};
use crate::model::wire::{netif, NET_CORE};
use crate::stage::Stage;

#[test]
fn a_command_takes_the_card_back_first() {
    let (mut r, c) = up();
    take_lines();
    c.borrow_mut().doze(false);
    assert!(r.tune(Band::Ghz2, 6), "the channel request went through");
    assert!(!c.borrow().own_sync, "the driver owns the card again");
    let lines = take_lines().concat();
    assert!(lines.contains("[mt7921] ownership: taken back from the firmware, time 1\n"));
}

#[test]
fn a_service_pass_takes_it_back_and_rearms_a_forgotten_wfdma() {
    let (mut r, c) = up();
    c.borrow_mut().doze(true);
    r.service(now());
    let chip = c.borrow();
    assert!(!chip.own_sync && chip.reg(DUMMY_CR) & 2 != 0, "owned, NEED_REINIT set again");
    assert_eq!(chip.reg(0xd4300 + 4), 64, "the data ring armed again");
    drop(chip);
    assert!(r.tune(Band::Ghz5, 36), "a grant comes back on the re-armed event ring");
}

#[test]
fn a_joined_link_carries_data_across_a_deep_doze() {
    let (mut st, c) = joined();
    c.borrow_mut().doze(true);
    let mut eth = vec![0xff; 6];
    eth.extend_from_slice(&[0x02, 0, 0, 0, 0, 7, 0x08, 0x00]);
    eth.extend_from_slice(b"after the doze");
    assert_eq!(netif(&mut st, NET_CORE, 4, &eth).0, 0);
    let got = c.borrow().air.as_ref().unwrap().ap.got.clone();
    assert!(got.iter().any(|b| b.ends_with(b"after the doze")), "{got:?}");
    c.borrow_mut().air.as_mut().unwrap().ap.chatter = true;
    for _ in 0..50 {
        mk_idle_ms(20);
        st.tick();
        if netif(&mut st, NET_CORE, 5, &[]).1.len() > 18 {
            return;
        }
    }
    panic!("nothing came in after the doze");
}

#[test]
fn a_card_left_warm_by_another_system_comes_up() {
    let c = chip();
    let mut k = c.borrow_mut();
    (k.own_sync, k.mcu.patched, k.mcu.running, k.faults.stuck_busy) = (true, true, true, true);
    drop(k);
    assert!(matches!(bring_up(&c), (Some(_), Stage::Ready)));
    let k = c.borrow();
    assert!(k.mcu.pieces.iter().all(|p| p.cmd != 0x05), "the patch the chip had is not resent");
    assert!(k.mcu.started.is_some(), "the RAM code was started");
}
