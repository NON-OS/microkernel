// NONOS Operating System (AGPL-3.0-or-later)
//! The waits, summed from the tables themselves: a whole bind's worst case
//! fits inside the bind budget nonos_usbnet gives it, and a link change's
//! inside a fifth of a second, since it holds the thread net.core is
//! answered on.

use nonos_usbnet::xhci::BIND_MS;

use crate::helpers::CODES;
use crate::rtl::autoneg::AUTONEG;
use crate::rtl::link_down::LINK_DOWN;
use crate::rtl::link_up::link_up;
use crate::rtl::model::identify;
use crate::rtl::op::Op;
use crate::rtl::start::start;
use crate::rtl::usb_speed::UsbSpeed;
use crate::rtl::wake::wake;

/// The longest the steps can pause, every wait run out.
fn worst_ms(ops: &[Op]) -> u64 {
    ops.iter()
        .map(|op| match *op {
            Op::Until(_, _, _, w) => w.tries as u64 * w.every_ms as u64,
            Op::Pause(ms) => ms as u64,
            _ => 0,
        })
        .sum()
}

#[test]
fn a_bind_run_to_every_bound_fits_its_budget() {
    for code in CODES {
        let m = identify(code).unwrap();
        for speed in [UsbSpeed::Full, UsbSpeed::High, UsbSpeed::Super] {
            let all = [wake(m, speed), AUTONEG.to_vec(), start(m)].concat();
            let ms = worst_ms(&all);
            assert!(ms <= 9_500, "{code:04x}: {ms} ms of waits");
            assert!(ms < BIND_MS, "{code:04x}: over the bind budget");
        }
    }
}

#[test]
fn a_link_change_holds_the_service_thread_a_fifth_of_a_second_at_most() {
    assert!(worst_ms(&LINK_DOWN) <= 200);
    for code in CODES {
        let m = identify(code).unwrap();
        assert!(worst_ms(&link_up(m, UsbSpeed::Super, 0x13)) <= 200, "{code:04x}");
    }
}

#[test]
fn every_wait_looks_at_least_once_and_pauses_between_looks() {
    for code in CODES {
        let m = identify(code).unwrap();
        let all =
            [wake(m, UsbSpeed::Super), AUTONEG.to_vec(), start(m), LINK_DOWN.to_vec()].concat();
        for op in all {
            if let Op::Until(t, mask, accept, w) = op {
                assert!(w.tries >= 1 && w.every_ms >= 1, "{t:?}");
                assert!(accept.iter().all(|a| a & !mask == 0), "{t:?}: a value the mask hides");
            }
        }
    }
}
