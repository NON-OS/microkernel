// NONOS Operating System (AGPL-3.0-or-later)
//! A command's exchange, on a USB 2 device (READ READY first) and with
//! streams (data at once), and what fails it (another tag's IUs:
//! dropped_tests.rs).

use nonos_uas::{DataDir, Exchange, ScsiCommand, StatusIu, Step};

/// READ(10) of one 512-byte block: the proofs' own SCSI command.
pub struct Read10 {
    pub cdb: [u8; 10],
    pub ended: Option<(u8, Vec<u8>)>,
}

impl Read10 {
    pub fn new() -> Read10 {
        Read10 { cdb: [0x28, 0, 0, 0, 0, 8, 0, 0, 1, 0], ended: None }
    }
}

impl ScsiCommand for Read10 {
    fn cdb(&self) -> &[u8] {
        &self.cdb
    }
    fn dir(&self) -> DataDir {
        DataDir::In
    }
    fn length(&self) -> u32 {
        512
    }
    fn finished(&mut self, status: u8, sense: &[u8]) {
        self.ended = Some((status, sense.to_vec()));
    }
}

const GOOD: StatusIu = StatusIu::Sense { tag: 3, status: 0, sense: &[] };

#[test]
fn usb_2_waits_for_read_ready_then_reads_then_ends() {
    let (mut cmd, mut x) = (Read10::new(), Exchange::new(3, DataDir::In, false));
    assert_eq!(x.sent(), Step::Status);
    assert_eq!(x.status(StatusIu::ReadReady { tag: 3 }, &mut cmd), Step::DataIn);
    assert_eq!(x.moved(), Step::Status);
    assert_eq!(x.status(GOOD, &mut cmd), Step::Done);
    assert_eq!(cmd.ended, Some((0, vec![])));
}

#[test]
fn with_streams_the_data_moves_at_once() {
    let (mut cmd, mut x) = (Read10::new(), Exchange::new(3, DataDir::In, true));
    assert_eq!(x.sent(), Step::DataIn);
    assert_eq!(x.moved(), Step::Status);
    assert_eq!(x.status(StatusIu::ReadReady { tag: 3 }, &mut cmd), Step::Failed(2));
    assert_eq!(Exchange::new(3, DataDir::Out, true).sent(), Step::DataOut);
}

#[test]
fn an_iu_out_of_turn_fails_and_a_response_says_why() {
    let mut cmd = Read10::new();
    let mut x = Exchange::new(3, DataDir::In, false);
    assert_eq!(x.status(StatusIu::WriteReady { tag: 3 }, &mut cmd), Step::Failed(2));
    let failed = StatusIu::Response { tag: 3, code: 0x09 };
    assert_eq!(x.status(failed, &mut cmd), Step::Failed(9));
    assert_eq!(cmd.ended, None);
    let check = StatusIu::Sense { tag: 3, status: 2, sense: &[0x70, 0, 6] };
    assert_eq!(x.status(check, &mut cmd), Step::Done);
    assert_eq!(cmd.ended, Some((2, vec![0x70, 0, 6])), "CHECK CONDITION and its sense");
}
