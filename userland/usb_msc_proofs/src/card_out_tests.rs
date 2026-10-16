// NONOS Operating System (AGPL-3.0-or-later)
//! A card taken out of a reader ends serving for the boot, as a card swapped
//! in does: nothing tells the card put back from another of its size. The
//! reader was let go and, once its wait passed, bound again with whatever
//! card of that size was in it, and that card served as the kernel's disk.

use crate::clock::advance;
use crate::fixture::{one_sector, ready_unit, sense};
use crate::protocol::{E_IO, E_NODEV, E_NOMEDIUM, OP_BLK_WRITE};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::serving::{ask, idle, read};
use crate::sticks::{bound, held};
use crate::xhci::{device, setup, Answer};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

/// A READ the reader fails with no medium, under NOT READY or UNIT
/// ATTENTION (`key`), and the check's TEST UNIT READY that says the same.
fn card_out(key: u8) -> Vec<Answer> {
    let failed = [Answer::Csw { residue: 512, status: 1 }, Answer::Data(sense(key, 0x3A)), OK];
    let check = [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, 0x3A)), OK];
    failed.into_iter().chain(check).collect()
}

/// Another card of the same size put in: INQUIRY, a TEST UNIT READY the
/// reader fails with its medium changed, then ready, and READ CAPACITY.
fn same_size_card() -> Vec<Answer> {
    let mut answers = ready_unit(1000);
    let changed = [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(0x06, 0x28)), OK];
    answers.splice(2..2, changed);
    answers
}

#[test]
fn a_card_taken_out_ends_serving() {
    let mut server = held(1000);
    setup(|d| d.answers = card_out(0x02).into());
    assert_eq!(read(&mut server), E_IO);
    assert!(matches!(server.scanner.medium, Medium::Changed), "the check let it go");
    let report = server.scanner.report;
    assert_eq!((report.stage, report.detail), (Stage::Changed, E_NOMEDIUM));
    for _ in 0..1_000 {
        advance(60);
        assert_eq!(read(&mut server), E_NODEV);
        idle(&mut server);
    }
    let asked = device(|d| (d.enabled, d.disabled.clone()));
    assert_eq!(asked, (1, vec![1]), "the reader given back, and not asked again");
}

#[test]
fn another_card_of_the_same_size_after_a_card_out_is_not_served() {
    for key in [0x02, 0x06] {
        let mut server = bound(1000);
        setup(|d| d.answers = card_out(key).into_iter().chain(same_size_card()).collect());
        assert_eq!(read(&mut server), E_IO, "no medium is not a change: not asked again");
        for _ in 0..1_000 {
            advance(60);
            idle(&mut server);
        }
        let served = matches!(server.scanner.medium, Medium::Ready(_));
        assert!(!served, "another card of the same size was served as the kernel's disk");
        let mut body = one_sector(7);
        body.extend_from_slice(&[0xAA; 512]);
        assert_eq!(ask(&mut server, 0, OP_BLK_WRITE, &body).status, E_NODEV);
    }
}
