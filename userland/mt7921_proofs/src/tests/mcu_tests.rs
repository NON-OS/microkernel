// NONOS Operating System (AGPL-3.0-or-later)

//! Commands and replies through the model's rings: sequence numbers one to
//! fifteen, never zero; a reply matched by its sequence, a unified reply
//! also by its `cid`, and its status honoured; a silent MCU a timeout; the
//! unasked events told apart as `mt7921_mcu_rx_event` tells them.

use crate::card::Card;
use crate::dma::Dma;
use crate::mac::tokens::Tokens;
use crate::mcu::cmd::{BSS_INFO_UPDATE, MAC_INIT_CTRL, PATCH_SEM_CONTROL};
use crate::mcu::event::Event;
use crate::mcu::{Mcu, McuError};
use crate::model::reply::{event, generic, uni};
use crate::model::rig::{chip, regs};

pub fn card() -> (Card, crate::model::Shared) {
    let c = chip();
    let r = regs(&c);
    let mut dma = Dma::map(1, 1).unwrap();
    dma.reset(&r);
    let (mcu, tokens) = (Mcu::default(), Tokens::default());
    (Card { regs: r, dma, mcu, tokens, woken: 0, keyed: false }, c)
}

#[test]
fn sequence_numbers_run_one_to_fifteen_and_skip_zero() {
    let (mut k, c) = card();
    for _ in 0..20 {
        k.send(MAC_INIT_CTRL, &[1, 0, 0, 0], false).unwrap();
    }
    let seqs: Vec<u8> = c.borrow().mcu.seen.iter().map(|s| s.seq).collect();
    let want: Vec<u8> = (1..=15).chain(1..=5).collect();
    assert_eq!(seqs, want);
}

#[test]
fn a_reply_is_its_commands_and_its_status_counts() {
    let (mut k, c) = card();
    assert_eq!(k.send(PATCH_SEM_CONTROL, &[1, 0, 0, 0], true).unwrap().status_byte(), 2);
    c.borrow_mut().faults.uni_status = 7;
    assert_eq!(k.send(BSS_INFO_UPDATE, &[0; 8], true).err(), Some(McuError::Status(7)));
    c.borrow_mut().faults.silent = true;
    assert_eq!(k.send(MAC_INIT_CTRL, &[0; 4], true).err(), Some(McuError::Timeout));
}

#[test]
fn a_stale_reply_or_another_cids_is_passed_over() {
    let (mut k, c) = card();
    c.borrow_mut().faults.silent = true;
    let _ = k.send(MAC_INIT_CTRL, &[0; 4], false); // seq 1, never answered
    c.borrow_mut().push_rx(0, generic(1, 0x01)); // a late answer to it
    c.borrow_mut().push_rx(0, uni(2, 0x01, 0)); // seq 2, but DEV_INFO's
    c.borrow_mut().push_rx(0, uni(2, 0x02, 0)); // seq 2, BSS_INFO's
    assert!(k.send(BSS_INFO_UPDATE, &[0; 8], true).is_ok(), "the third was its reply");
}

#[test]
fn unasked_events_are_told_from_replies() {
    let un = |e: Vec<u8>| Event::parse(&e).unwrap().unsolicited();
    assert!(un(event(0x13, 4, 0, 0, &[0; 4])), "beacon loss");
    assert!(un(event(0x0d, 4, 0, 0, &[0; 4])), "scan done");
    assert!(un(event(0x01, 0, 0, 0, &[0; 4])), "sequence zero");
    assert!(un(event(0x05, 4, 0, 0x87, &[0; 4])), "a rate report");
    assert!(un(event(0x27, 0, 0x06, 0, &[0; 24])), "the ROC grant");
    assert!(!un(event(0x06, 0, 0, 0, &[0; 4])), "the generic event is a reply");
    assert!(!un(generic(3, 0x01)) && !un(uni(3, 0x02, 0)));
    assert!(Event::parse(&event(0x01, 3, 0, 0, &[])[..35]).is_none(), "shorter than its header");
    let mut frame = event(0x01, 3, 0, 0, &[]);
    frame[3] = 2 << 3;
    assert!(Event::parse(&frame).is_none(), "a frame, not an event");
}
