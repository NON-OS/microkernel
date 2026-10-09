// NONOS Operating System (AGPL-3.0-or-later)
//! The link read on the clock, at most once a second, cached for
//! link_up, and the medium set for it as ax88179_link_reset sets it.

use std::time::{Duration, Instant};

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Nic, Setup};

use crate::bound::bound;
use crate::calls::mac_w;
use crate::chip::Chip;

const PHYSR: Call = Call::In(Setup::new(0xC0, 0x02, 3, 0x11), 2);
const FIFO: Call = Call::In(Setup::new(0xC0, 0x81, 0x8c, 0), 4);

#[test]
fn a_gigabit_link_is_found_its_medium_set_and_the_answer_cached() {
    let chip = Chip::default();
    let (bus, mut nic) = bound(&chip);
    let start = bus.0.borrow().calls.len();
    chip.physr.set(0xa400);
    nic.tick();
    assert!(nic.link_up());
    let calls = bus.0.borrow().calls[start..].to_vec();
    let expected = vec![
        PHYSR,
        mac_w(0x0b, &[0, 0]),
        mac_w(0x0b, &[0xaa, 0x03]),
        FIFO,
        Call::In(Setup::new(0xC0, 0x01, 0x02, 1), 1),
        PHYSR,
        mac_w(0x2e, &[7, 0x4f, 0, 2, 0xff]),
        mac_w(0x22, &[0x3b, 0x01]),
    ];
    assert_eq!(calls, expected);
    let n = bus.0.borrow().calls.len();
    nic.tick();
    (0..100).for_each(|_| assert!(nic.link_up()));
    assert_eq!(bus.0.borrow().calls.len(), n, "no look within the second, none in link_up");
}

#[test]
fn a_dropped_link_and_a_renegotiated_one_are_seen_on_the_next_look() {
    let chip = Chip::default();
    let (bus, mut nic) = bound(&chip);
    chip.physr.set(0xa400);
    nic.tick();
    std::thread::sleep(Duration::from_millis(1_050));
    chip.physr.set(0x6400);
    nic.tick();
    assert!(nic.link_up());
    let last = bus.0.borrow().calls.last().cloned();
    assert_eq!(last, Some(mac_w(0x22, &[0x32, 0x03])), "100 full duplex");
    std::thread::sleep(Duration::from_millis(1_050));
    chip.physr.set(0x0000);
    nic.tick();
    assert!(!nic.link_up());
    assert_eq!(bus.0.borrow().calls.last().cloned(), Some(PHYSR));
}

#[test]
fn a_tx_fifo_that_stays_busy_leaves_the_link_down_after_a_tenth_of_a_second() {
    let chip = Chip { fifo_busy: true, ..Chip::default() };
    let (bus, mut nic) = bound(&chip);
    chip.physr.set(0xa400);
    let t = Instant::now();
    nic.tick();
    let took = t.elapsed();
    assert!(!nic.link_up());
    assert!(took >= Duration::from_millis(100) && took < Duration::from_millis(900), "{took:?}");
    let reads = bus.0.borrow().calls.iter().filter(|c| **c == FIFO).count();
    assert!(reads >= 2, "the FIFO is read again until the deadline");
}
