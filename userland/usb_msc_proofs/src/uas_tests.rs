// NONOS Operating System (AGPL-3.0-or-later)
//! The UAS transport (`disk/uas.rs`) at high speed against a scripted
//! device: the Command IU on the first pair, READ READY on the status pipe
//! before the data moves on the second pair, a Sense IU at the end; and a
//! CHECK CONDITION's sense kept, so REQUEST SENSE asks nothing.

use crate::descriptors::MscBinding;
use crate::disk::{read, Disk};
use crate::protocol::E_IO;
use crate::state::State;
use crate::xhci::{device, script, Answer};

fn disk() -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x85, bulk_out: 0x06, ..MscBinding::default() };
    let mut d = Disk::bound(1, 1, &b);
    (d.blocks, d.block_len, d.uas) = (1000, 512, true);
    d
}

fn sense_iu(status: u8, sense: &[u8]) -> Answer {
    let mut iu = vec![0x03, 0, 0, 1, 0, 0, status, 0, 0, 0, 0, 0, 0, sense.len() as u8, 0, 0];
    iu.extend_from_slice(sense);
    Answer::Data(iu)
}

#[test]
fn a_read_waits_for_read_ready_then_moves_on_the_data_pair() {
    let block: Vec<u8> = (0..512).map(|i| i as u8).collect();
    script(vec![Answer::Data(vec![0x06, 0, 0, 1]), Answer::Data(block.clone()), sense_iu(0, &[])]);
    let mut out = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut out), Ok(()));
    assert_eq!(&out[..], &block[..]);
    let (sent, read_on) = device(|d| (d.sent.clone(), d.read_on.clone()));
    assert_eq!(read_on, [0, 1, 0], "status, data, status");
    let (pair, iu) = &sent[0];
    assert_eq!((*pair, iu.len(), &iu[0..4]), (0, 32, &[0x01, 0, 0, 1][..]));
    assert_eq!((iu[16], &iu[18..22]), (0x28, &[0, 0, 0, 7][..]), "READ(10) of block 7");
}

#[test]
fn a_check_condition_keeps_its_sense_and_request_sense_asks_nothing() {
    let mut s = vec![0u8; 18];
    (s[0], s[2], s[7]) = (0x70, 0x03, 10);
    script(vec![sense_iu(2, &s)]);
    let mut out = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 0, &mut out), Err(E_IO));
    assert_eq!(device(|d| d.sent.len()), 1, "REQUEST SENSE was answered from the Sense IU");
}

#[test]
fn an_iu_out_of_turn_fails_the_command() {
    script(vec![Answer::Data(vec![0x07, 0, 0, 1])]);
    let mut out = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 0, &mut out), Err(E_IO), "WRITE READY for a read");
    script(vec![Answer::Data(vec![0x06, 0, 0, 2])]);
    assert_eq!(read(&disk(), &mut State::new(), 0, &mut out), Err(E_IO), "another tag");
}

#[test]
fn on_streams_the_data_moves_at_once_on_the_tags_stream() {
    let block = vec![7u8; 512];
    script(vec![Answer::Data(block.clone()), sense_iu(0, &[])]);
    let mut d = disk();
    d.streams = true;
    let mut out = [0u8; 512];
    assert_eq!(read(&d, &mut State::new(), 0, &mut out), Ok(()));
    let (sent, read_on) = device(|d| (d.sent.clone(), d.read_on.clone()));
    assert_eq!(read_on, [0x11, 0x10], "data on pair 1, then status on pair 0, both stream 1");
    assert_eq!(sent[0].0, 0, "the command pipe has no streams");
}
