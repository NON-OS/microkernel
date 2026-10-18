// NONOS Operating System (AGPL-3.0-or-later)
//! A command whose phase was lost on the bus is heard as an attention
//! (disk/recover.rs). A reader clears its unit attention once it fails a
//! command with CHECK CONDITION for it (SPC, UA_INTLCK_CTRL 00b), so when
//! that CSW, or the CSW of the REQUEST SENSE that read the 28h, is lost,
//! nothing says the change again. The unfixed driver kept a card of the same
//! size swapped in with the generation unmoved, and the next write landed.

use crate::answers::passes_check;
use crate::fixture::{class_resets, sense};
use crate::kernel::{generation, write};
use crate::protocol::E_IO;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::read;
use crate::sticks::held;
use crate::xhci::{setup, Answer};

/// A CSW that does not read: the transport is reset.
fn lost_csw() -> Answer {
    Answer::Data(vec![0; 13])
}

/// After the READ the swap failed, the check is clean: card B is the size
/// card A was. It is served afresh, so the kernel can know.
pub(super) fn served_afresh(server: &mut Server) {
    assert_eq!(read(server), E_IO);
    assert!(class_resets() >= 1, "the transport was reset");
    assert!(matches!(server.scanner.medium, Medium::Ready(_)), "{:?}", server.scanner.report);
    assert_eq!(generation(server), 1, "kept with no sign");
    assert_eq!(write(server), 0, "the kernel's next write, once it has read again");
}

#[test]
fn a_change_whose_request_sense_lost_its_csw_raises_the_generation() {
    // The READ fails with CHECK CONDITION; REQUEST SENSE carries the 28h,
    // which the reader has now said, but its CSW is lost.
    let mut server = held(1000);
    let read_fails = [Answer::Csw { residue: 512, status: 1 }];
    let sense_lost = [Answer::Data(sense(0x06, 0x28)), lost_csw()];
    let answers = read_fails.into_iter().chain(sense_lost).chain(passes_check(1000));
    setup(|d| d.answers = answers.collect());
    served_afresh(&mut server);
}

#[test]
fn a_failed_reads_own_csw_lost_raises_the_generation() {
    // The READ's CHECK CONDITION for the pending 28h clears it, and that CSW
    // is lost.
    let mut server = held(1000);
    let answers = [Answer::Data(vec![0; 512]), lost_csw()].into_iter().chain(passes_check(1000));
    setup(|d| d.answers = answers.collect());
    served_afresh(&mut server);
}

#[test]
fn a_command_block_said_to_go_short_raises_the_generation() {
    // driver.xhci0 says the CBW went a byte short. That count is the
    // controller's: the device may have taken all of it and failed the READ
    // for a pending 28h, which clears it. So it is lost like any other phase.
    let mut server = held(1000);
    setup(|d| (d.short_outs, d.answers) = (1, passes_check(1000).into()));
    served_afresh(&mut server);
}
