// NONOS Operating System (AGPL-3.0-or-later)
//! A unit that never comes ready, as it answers each command: INQUIRY, a
//! TEST UNIT READY that fails, the sense NOT READY, becoming ready (04h),
//! and START UNIT, for as long as it is asked.

use super::device::Answer;

const TEST_UNIT_READY: u8 = 0x00;
const REQUEST_SENSE: u8 = 0x03;
const INQUIRY: u8 = 0x12;
const START_UNIT: u8 = 0x1B;

/// The answer to bulk IN `phase` of the command `op`; none for any other.
pub(super) fn unready(op: u8, phase: u8) -> Option<Answer> {
    let ok = Answer::Csw { residue: 0, status: 0 };
    match (op, phase) {
        (INQUIRY, 0) => Some(Answer::Data(vec![0; 36])),
        (REQUEST_SENSE, 0) => {
            let mut sense = vec![0u8; 18];
            (sense[0], sense[2], sense[7], sense[12], sense[13]) = (0x70, 0x02, 10, 0x04, 0x01);
            Some(Answer::Data(sense))
        }
        (TEST_UNIT_READY, 0) => Some(Answer::Csw { residue: 0, status: 1 }),
        (INQUIRY | REQUEST_SENSE, 1) | (START_UNIT, 0) => Some(ok),
        _ => None,
    }
}
