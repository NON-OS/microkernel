// NONOS Operating System (AGPL-3.0-or-later)
//! What the check makes of the medium served. A card swapped in once the
//! kernel has read a sector, as the reader says (UNIT ATTENTION 28h), or a
//! new size shows, ends serving for the boot: the kernel chose its disk
//! once, and a write meant for the old card landed on the new one. A card
//! taken out ends it too (card_out_tests.rs).

use crate::fixture::{one_sector, sense};
use crate::protocol::{E_IO, E_NODEV, OP_BLK_WRITE};
use crate::scan::Medium;
use crate::serving::{ask, idle, read};
use crate::sticks::{bound, held};
use crate::xhci::{device, setup, Answer};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

/// A READ the device fails at once with `key` and `asc`, then `after`.
fn fails_read(key: u8, asc: u8, after: Vec<Answer>) {
    let failed = [Answer::Csw { residue: 512, status: 1 }, Answer::Data(sense(key, asc)), OK];
    setup(|d| d.answers = failed.into_iter().chain(after).collect());
}

/// TEST UNIT READY passed, and READ CAPACITY(10) says `blocks` of 512 bytes.
fn ready_at(blocks: u32) -> Vec<Answer> {
    let mut cap = (blocks - 1).to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    vec![OK, Answer::Data(cap), OK]
}

fn write(server: &mut crate::server::serve::Server) -> i32 {
    let mut body = one_sector(7);
    body.extend_from_slice(&[0xAA; 512]);
    ask(server, 0, OP_BLK_WRITE, &body).status
}

#[test]
fn a_write_after_a_card_swap_is_refused() {
    let mut server = held(1000);
    fails_read(0x06, 0x28, ready_at(2000).into_iter().chain([OK]).collect());
    assert_eq!(read(&mut server), E_IO);
    idle(&mut server);
    assert_eq!(write(&mut server), E_NODEV, "a write meant for the old card");
    assert_eq!(device(|d| d.answers.len()), 4, "the new card was asked nothing");
}

#[test]
fn a_card_of_another_size_is_served_no_more() {
    let mut server = bound(1000);
    fails_read(0x03, 0x11, ready_at(2000).into_iter().chain([OK]).collect());
    assert_eq!(read(&mut server), E_IO);
    idle(&mut server);
    assert!(matches!(server.scanner.medium, Medium::Changed));
    assert_eq!((write(&mut server), device(|d| d.answers.len())), (E_NODEV, 1));
}
