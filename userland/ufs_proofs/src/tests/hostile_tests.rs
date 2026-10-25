// NONOS Operating System (AGPL-3.0-or-later)
//! A device whose answers lie (JESD220 UPIU headers, as recalled,
//! unverified): a response for another slot or unit, a query answer for
//! another question, a read the device never fills, and a unit larger
//! than its bytes can count. None is taken for the answer the engine asked for.

use crate::model::{Cfg, Lu, Rig};
use crate::ufs::disk::rw::{read, write};
use crate::ufs::error::UfsError;
use crate::ufs::facts::upiu::{NOP_IN, QUERY_RSP, RESPONSE};

fn lying(tt: u8, at: usize, v: u8) -> Option<UfsError> {
    let rig = Rig::new(Cfg::default());
    rig.m.lie.set(Some((tt, at, v)));
    rig.up(false).err()
}

#[test]
fn an_answer_for_another_slot_unit_or_question_is_refused() {
    assert_eq!(lying(NOP_IN, 3, 9), Some(UfsError::Upiu(NOP_IN)), "NOP IN, another tag");
    assert_eq!(lying(QUERY_RSP, 3, 9), Some(UfsError::Upiu(QUERY_RSP)), "query, another tag");
    assert_eq!(lying(QUERY_RSP, 13, 2), Some(UfsError::Upiu(QUERY_RSP)), "another flag");
    assert_eq!(lying(QUERY_RSP, 12, 7), Some(UfsError::Upiu(QUERY_RSP)), "another opcode");
    // REPORT LUNS goes to the well-known LU: an answer from LU 0 is not it.
    assert_eq!(lying(RESPONSE, 2, 0), Some(UfsError::Upiu(RESPONSE)), "another unit");
    assert_eq!(lying(RESPONSE, 3, 9), Some(UfsError::Upiu(RESPONSE)), "command, another tag");
}

#[test]
fn a_read_the_device_never_fills_reads_zeros_not_the_last_write() {
    let rig = Rig::new(Cfg::default());
    let ufs = rig.up(false).unwrap();
    let u = *ufs.unit(0).unwrap();
    write(&rig.m, &ufs, &u, 3, 1, &[0x5A; 4096]).unwrap();
    rig.m.mute.set(Some(0x28));
    let mut back = vec![0xAAu8; 4096];
    read(&rig.m, &ufs, &u, 9, 1, &mut back).expect("GOOD, but no data came");
    assert!(back.iter().all(|&b| b == 0), "the write's bytes leaked into the read");
}

/// A unit whose last LBA reads all ones in READ CAPACITY(16): more blocks
/// than any count of its bytes can hold.
#[test]
fn a_unit_whose_bytes_overflow_is_not_served() {
    let lus = vec![Lu::new(0, 4096, 0), Lu::new(1, 512, 64)];
    let rig = Rig::new(Cfg { lus, ..Cfg::default() });
    let ufs = rig.up(false).expect("the other unit");
    assert_eq!(ufs.unit(0).map(|u| u.lun), Some(1));
    assert!(rig.m.said("LU 0 not served: capacity of 18446744073709551615 blocks"));
}
