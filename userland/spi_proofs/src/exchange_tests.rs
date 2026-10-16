// NONOS Operating System (AGPL-3.0-or-later)
//! A full-duplex transfer in lock step: every byte sent, every answer read
//! in order, a stale frame drained first, and a stalled or dead core ending
//! the transfer instead of hanging it.

use crate::core::exchange;
use crate::model::Ssp;

#[test]
fn every_byte_goes_out_and_every_answer_comes_back_in_order() {
    let ssp = Ssp::new(|b| b ^ 0xFF, 3);
    let tx: Vec<u8> = (0..200).map(|i| i as u8).collect();
    let mut rx = vec![0u8; 200];
    assert_eq!(exchange(&ssp, &tx, &mut rx, || true), Ok(()));
    assert_eq!(ssp.inner.borrow().sent, tx);
    assert!(rx.iter().zip(&tx).all(|(r, t)| *r == t ^ 0xFF));
    assert!(!ssp.inner.borrow().ror, "lock step never overruns");
}

#[test]
fn a_stale_frame_is_drained_before_the_transfer() {
    let ssp = Ssp::new(|b| b, 1);
    ssp.inner.borrow_mut().rx.extend([0xAA, 0xBB]);
    let mut rx = [0u8; 2];
    exchange(&ssp, &[1, 2], &mut rx, || true).unwrap();
    assert_eq!(rx, [1, 2], "not the leftovers");
}

#[test]
fn a_silent_or_dead_core_ends_the_transfer() {
    let ssp = Ssp::new(|b| b, u32::MAX);
    let mut budget = 50;
    let mut rx = [0u8; 4];
    let time = || {
        budget -= 1;
        budget > 0
    };
    assert_eq!(exchange(&ssp, &[1, 2, 3, 4], &mut rx, time), Err(-110));
    let dead = Ssp::new(|b| b, 1);
    dead.inner.borrow_mut().stuck_rne = true;
    assert_eq!(exchange(&dead, &[1], &mut [0], || true), Err(-5));
    assert_eq!(
        exchange(&Ssp::new(|b| b, 1), &[1, 2], &mut [0], || true),
        Err(-5),
        "lengths differ"
    );
}
