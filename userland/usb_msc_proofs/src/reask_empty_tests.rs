// NONOS Operating System (AGPL-3.0-or-later)
//! The unit served, asked again after a let-go with no card in it: the card
//! was taken out while the reader was let go, and the one put in after may
//! be another of the same size, which a reader need not say it changed.
//! Serving ends, as for a card taken out under the check (card_out_tests).

use crate::clock::advance;
use crate::fixture::{ready_unit, sense};
use crate::protocol::{E_IO, E_NODEV, E_NOMEDIUM};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::serving::{idle, read};
use crate::sticks::bound;
use crate::xhci::{setup, Answer};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

fn fails(key: u8, asc: u8) -> [Answer; 3] {
    [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, asc)), OK]
}

#[test]
fn the_unit_served_found_empty_when_asked_again_ends_serving() {
    let mut server = bound(1000);
    let check = fails(0x03, 0x11).into_iter().chain(fails(0x02, 0x04)).chain([OK]);
    let empty = [Answer::Data(vec![0; 36]), OK].into_iter().chain(fails(0x02, 0x3A));
    let again = empty.chain(fails(0x02, 0x3A)).chain(ready_unit(1000));
    setup(|d| d.answers = check.chain(again).collect());
    assert_eq!(read(&mut server), E_IO);
    for _ in 0..1_000 {
        advance(60);
        idle(&mut server);
    }
    assert!(matches!(server.scanner.medium, Medium::Changed), "{:?}", server.scanner.report);
    let report = server.scanner.report;
    assert_eq!((report.stage, report.detail), (Stage::Changed, E_NOMEDIUM));
    assert_eq!(read(&mut server), E_NODEV, "the card put in after is not served");
}
