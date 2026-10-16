// NONOS Operating System (AGPL-3.0-or-later)
//! Each way a held READ's CSW can fail to say how the command ended is a
//! lost phase (disk/bot.rs), heard as an attention: the device may have
//! failed the READ for a pending 28h, which clears it, so the medium is
//! served afresh and the generation raised. A CSW that does say how it
//! ended, good but with bytes left over, is a medium error: no CHECK
//! CONDITION cleared anything, and the medium is kept as it was.

use super::lost_tests::served_afresh;
use crate::answers::passes_check;
use crate::fixture::one_sector;
use crate::kernel::{generation, write};
use crate::protocol::{E_IO, OP_BLK_READ};
use crate::scan::Medium;
use crate::serving::{answer, idle, read};
use crate::sticks::held;
use crate::xhci::{setup, Answer};

/// The READ's 512 bytes, then `csw`, then a clean check at the same size.
fn read_then(csw: Answer) -> Vec<Answer> {
    [Answer::Data(vec![0; 512]), csw].into_iter().chain(passes_check(1000)).collect()
}

#[test]
fn a_held_reads_csw_with_another_tag_raises_the_generation() {
    let mut csw = b"USBS".to_vec();
    csw.extend_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
    csw.extend_from_slice(&[0, 0, 0, 0, 1]);
    let mut server = held(1000);
    setup(|d| d.answers = read_then(Answer::Data(csw)).into());
    served_afresh(&mut server);
}

#[test]
fn a_held_reads_phase_error_csw_raises_the_generation() {
    let mut server = held(1000);
    setup(|d| d.answers = read_then(Answer::Csw { residue: 0, status: 2 }).into());
    served_afresh(&mut server);
}

#[test]
fn a_held_reads_csw_that_never_comes_after_the_data_moved_raises_the_generation() {
    let mut server = held(1000);
    setup(|d| d.answers = [Answer::Data(vec![0; 512])].into());
    let status = answer(&mut server, 0, OP_BLK_READ, &one_sector(7)).status;
    assert_eq!(status, E_IO, "driver.xhci0 gave up waiting for it");
    setup(|d| d.answers = passes_check(1000).into());
    idle(&mut server);
    assert!(matches!(server.scanner.medium, Medium::Ready(_)), "{:?}", server.scanner.report);
    assert_eq!(generation(&mut server), 1, "kept with no sign");
    assert_eq!(write(&mut server), 0, "the kernel's next write, once it has read again");
}

#[test]
fn a_held_read_said_good_with_bytes_left_over_is_a_medium_error_not_a_change() {
    let mut server = held(1000);
    setup(|d| d.answers = read_then(Answer::Csw { residue: 512, status: 0 }).into());
    assert_eq!(read(&mut server), E_IO);
    assert!(matches!(server.scanner.medium, Medium::Ready(_)), "{:?}", server.scanner.report);
    assert_eq!(generation(&mut server), 0, "kept as it was");
}
