// NONOS Operating System (AGPL-3.0-or-later)
//! The claim step that refused and its errno reach the status reply in the
//! two words after the efuse diagnosis, and a later claim that held clears
//! them. These are the only claim facts a laptop with no serial port shows.

use crate::efuse::diag;
use crate::link::LinkStats;
use crate::serve_status::{status_body, STATUS_BODY_LEN};
use crate::setup::claim_note::{clear, note, read, STEP_CLAIM, STEP_MAP, STEP_MEMORY};

fn words(claim: (u32, u32)) -> (u32, u32) {
    let mut out = [0u8; STATUS_BODY_LEN];
    assert_eq!(status_body(4, &LinkStats::default(), &diag(), claim, &mut out), Some(53));
    let w = |at: usize| u32::from_le_bytes([out[at], out[at + 1], out[at + 2], out[at + 3]]);
    (w(37), w(41))
}

#[test]
fn a_refused_step_and_its_errno_are_reported_then_cleared() {
    note(STEP_CLAIM, -16);
    assert_eq!(read(), (1, 16), "the claim's EBUSY, as a positive number");
    assert_eq!(words(read()), (1, 16), "step first, errno second");
    note(STEP_MEMORY, -1);
    assert_eq!(words(read()), (2, 1), "the last refused try is the one kept");
    note(STEP_MAP, -14);
    assert_eq!(words(read()), (3, 14));
    note(STEP_CLAIM, 0);
    assert_eq!(read(), (1, 0), "an epoch of 0 refuses with no errno");
    clear();
    assert_eq!(words(read()), (0, 0), "a claim that held names no step");
}
