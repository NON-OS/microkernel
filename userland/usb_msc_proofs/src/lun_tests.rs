// NONOS Operating System (AGPL-3.0-or-later)
//! A card reader whose served card is in its second slot, asked again after
//! a let-go with a card now in its first: the first slot's unit is not the
//! one served, so the next is asked, and the served card is found there.
//! The unfixed driver refused the whole reader on its first slot.

use crate::fixture::{ready_unit, sense};
use crate::protocol::E_IO;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{read, until_ready};
use crate::sticks::plug;
use crate::xhci::{setup, Answer};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

fn fails(key: u8, asc: u8) -> [Answer; 3] {
    [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, asc)), OK]
}

fn served(server: &Server) -> (u8, u64) {
    let Medium::Ready(d) = server.scanner.medium else { panic!("{:?}", server.scanner.report) };
    (d.lun, d.blocks)
}

#[test]
fn a_reader_is_asked_past_a_slot_that_is_not_the_one_served() {
    let inquiry = || [Answer::Data(vec![0; 36]), OK];
    let empty_first = inquiry().into_iter().chain(fails(0x02, 0x3A)).chain(fails(0x02, 0x3A));
    plug(empty_first.chain(ready_unit(1000)).collect());
    setup(|d| d.max_lun = 1);
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    assert_eq!(served(&server), (1, 1000), "the card in the second slot");
    let check = fails(0x03, 0x11).into_iter().chain(fails(0x02, 0x04)).chain([OK]);
    let again = ready_unit(2000).into_iter().chain(ready_unit(1000));
    setup(|d| d.answers = check.chain(again).collect());
    assert_eq!(read(&mut server), E_IO);
    until_ready(&mut server, 2_000);
    assert_eq!(served(&server), (1, 1000), "past the card put in the first slot");
}

/// The same reader, its served card in the second slot, let go: asked
/// again, the first slot says its medium changed, then that it has none.
/// That change and that empty slot are the first slot's, not the served
/// card's, whose reader serial number they share.
#[test]
fn a_change_said_by_another_slot_of_the_reader_is_that_slots_own() {
    let inquiry = || [Answer::Data(vec![0; 36]), OK];
    let empty_first = inquiry().into_iter().chain(fails(0x02, 0x3A)).chain(fails(0x02, 0x3A));
    plug(empty_first.chain(ready_unit(1000)).collect());
    setup(|d| d.max_lun = 1);
    let mut server = Server::new();
    until_ready(&mut server, 2_000);
    let check = fails(0x03, 0x11).into_iter().chain(fails(0x02, 0x04)).chain([OK]);
    let first = inquiry().into_iter().chain(fails(0x06, 0x28)).chain(fails(0x02, 0x3A));
    let first = first.chain(fails(0x02, 0x3A));
    setup(|d| d.answers = check.chain(first).chain(ready_unit(1000)).collect());
    assert_eq!(read(&mut server), E_IO);
    until_ready(&mut server, 2_000);
    assert_eq!(served(&server), (1, 1000), "the served card, in its own slot");
}
