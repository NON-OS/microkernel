// NONOS Operating System (AGPL-3.0-or-later)
//! A medium change said before the kernel has read or written a sector: it
//! holds nothing of the medium yet, so the device is asked TEST UNIT READY
//! and READ CAPACITY again and what it holds now is served, its identity
//! recorded afresh. A reader that said so on the first READ after it was
//! bound lost the boot disk; that READ is now answered E_AGAIN, as the
//! kernel's identify asks again on it and passed the stick over on E_IO.
//! Once the kernel holds a sector, a medium change ends serving.

use crate::fixture::sense;
use crate::protocol::{E_AGAIN, E_IO, E_NODEV};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::serving::read;
use crate::sticks::{bound, held};
use crate::xhci::{setup, Answer};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };

fn capacity(blocks: u32) -> Answer {
    let mut c = (blocks - 1).to_be_bytes().to_vec();
    c.extend_from_slice(&512u32.to_be_bytes());
    Answer::Data(c)
}

/// A command the device fails with sense `key` and `asc`.
fn fails(key: u8, asc: u8) -> [Answer; 3] {
    [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, asc)), OK]
}

/// The first READ fails with `read`; the check then hears `check`, and the
/// card is ready at `blocks`.
fn first_read(read: [Answer; 3], check: Vec<Answer>, blocks: u32) -> Vec<Answer> {
    let ready = [OK, capacity(blocks), OK];
    read.into_iter().chain(check).chain(ready).collect()
}

#[test]
fn a_medium_change_on_the_first_read_is_served_afresh() {
    let mut server = bound(1000);
    setup(|d| d.answers = first_read(fails(0x06, 0x28), vec![], 2000).into());
    assert_eq!(read(&mut server), E_AGAIN, "asked again, not passed over");
    let Medium::Ready(d) = server.scanner.medium else { panic!("{:?}", server.scanner.report) };
    assert_eq!(d.blocks, 2000, "at the size it has now");
    setup(|d| d.answers = [Answer::Data(vec![0; 512]), OK].into_iter().collect());
    assert_eq!(read(&mut server), 0);
}

#[test]
fn a_power_on_then_a_medium_change_before_the_first_read_is_served_afresh() {
    let mut server = bound(1000);
    let check = fails(0x06, 0x29).into_iter().chain(fails(0x06, 0x28)).collect();
    setup(|d| d.answers = first_read(fails(0x03, 0x11), check, 1000).into());
    assert_eq!(read(&mut server), E_IO);
    assert!(matches!(server.scanner.medium, Medium::Ready(_)), "{:?}", server.scanner.report);
}

#[test]
fn a_medium_change_once_a_sector_was_read_ends_serving() {
    let after_reset: Vec<Answer> = fails(0x06, 0x29).into_iter().chain(fails(0x06, 0x28)).collect();
    for (first, check) in [(fails(0x06, 0x28), vec![]), (fails(0x03, 0x11), after_reset)] {
        let mut server = held(1000);
        setup(|d| d.answers = first_read(first, check, 1000).into());
        assert_eq!(read(&mut server), E_IO);
        assert_eq!(server.scanner.report.stage, Stage::Changed);
        assert_eq!(read(&mut server), E_NODEV, "a read meant for the old card");
    }
}
