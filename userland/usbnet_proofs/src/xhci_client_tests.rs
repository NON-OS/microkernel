// NONOS Operating System (AGPL-3.0-or-later)
//! The client side of driver.xhci0 against replies a broken or hostile
//! controller driver could send: each is refused by name, none panics.

use nonos_libc::{calls, play, serial};

use crate::fake_xhci::{answer, reply, SLOT};
use crate::xhci::{address_device, enable_slot, XhciBus, BULK_MAX, E_INVAL, E_IO};
use crate::{Bus, Setup};

pub fn bus() -> XhciBus {
    XhciBus { xhci: 0x4011, slot: SLOT }
}

#[test]
fn a_bulk_out_past_one_page_or_of_nothing_is_refused_before_it_is_sent() {
    let _one = serial();
    play(|req| Some(answer(req)));
    for len in [0, BULK_MAX + 1, 2 * BULK_MAX] {
        assert_eq!(bus().bulk_out(&vec![0u8; len]), Err(E_INVAL), "{len} bytes");
    }
    assert!(calls().is_empty());
    assert!(bus().bulk_out(&vec![0u8; BULK_MAX]).is_ok(), "one whole page goes");
}

#[test]
fn a_reply_shorter_than_its_own_length_word_is_refused() {
    let _one = serial();
    play(|req| {
        let mut r = reply(req, 0, &[18, 0]);
        r[16..20].copy_from_slice(&(4 + 2 + 18u32).to_le_bytes());
        Some(r)
    });
    let got = bus().control_in(Setup::get_descriptor(1, 0), &mut [0u8; 18]);
    assert_eq!(got, Err(E_IO));
}

#[test]
fn a_reply_to_another_request_or_with_no_room_for_its_status_is_refused() {
    let _one = serial();
    play(|req| {
        let mut r = reply(req, 0, &[SLOT, 0, 0, 0]);
        r[12] ^= 1;
        Some(r)
    });
    assert_eq!(enable_slot(0x4011), Err(E_IO), "another request id");
    play(|req| Some(req[..20].to_vec()));
    assert_eq!(enable_slot(0x4011), Err(E_IO), "a header and no status");
    play(|req| Some(reply(req, -16, &[])));
    assert_eq!(address_device(0x4011, SLOT, 1), Err(-16), "a status is passed up");
    play(|_| None);
    assert_eq!(enable_slot(0x4011), Err(E_IO), "no answer in time");
}

#[test]
fn every_call_outwaits_the_controllers_own_transfer_wait() {
    let _one = serial();
    play(|req| Some(answer(req)));
    let _ = enable_slot(0x4011);
    let _ = bus().control_in(Setup::get_descriptor(1, 0), &mut [0u8; 18]);
    let _ = bus().bulk_out(&[0u8; 64]);
    let waits: Vec<u64> = calls().iter().map(|a| a.timeout_ms).collect();
    assert_eq!(waits.len(), 3);
    // driver.xhci0 gives a transfer 5 s (wait_transfer_completion.rs), and
    // the kernel turns a 0 into 5 s: a call no longer than that could give
    // up before the controller says why, and its reply would come late.
    assert!(waits.iter().all(|&w| w > 5_000), "{waits:?}");
}
