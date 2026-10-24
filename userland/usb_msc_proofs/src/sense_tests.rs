// NONOS Operating System (AGPL-3.0-or-later)
//! A sense answer is read from the bytes the device sent. A SYNCHRONIZE
//! CACHE it fails with ILLEGAL REQUEST is taken as nothing to flush; three
//! bytes claiming that, the rest never sent, are no sense and no flush.

use crate::descriptors::MscBinding;
use crate::disk::{sync_cache, Disk};
use crate::protocol::E_IO;
use crate::state::State;
use crate::xhci::{device, script, Answer};

fn disk() -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    let mut d = Disk::bound(1, 1, &b);
    (d.blocks, d.block_len) = (1000, 512);
    d
}

const ILLEGAL_REQUEST: u8 = 0x05;

#[test]
fn a_whole_illegal_request_sense_is_a_cache_with_nothing_to_flush() {
    let mut sense = vec![0u8; 18];
    (sense[0], sense[2], sense[7]) = (0x70, ILLEGAL_REQUEST, 10);
    script(vec![
        Answer::Csw { residue: 0, status: 1 },
        Answer::Data(sense),
        Answer::Csw { residue: 0, status: 0 },
    ]);
    assert_eq!(sync_cache(&disk(), &mut State::new()), Ok(()));
}

#[test]
fn three_bytes_of_sense_under_a_csw_of_no_residue_are_no_sense() {
    script(vec![
        Answer::Csw { residue: 0, status: 1 },
        Answer::Data(vec![0x70, 0, ILLEGAL_REQUEST]),
        Answer::Csw { residue: 0, status: 0 },
    ]);
    assert_eq!(sync_cache(&disk(), &mut State::new()), Err(E_IO));
    assert_eq!(device(|d| d.answers.len()), 0);
}
