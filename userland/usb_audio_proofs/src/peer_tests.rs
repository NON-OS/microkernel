// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's wire numbers are driver.hda0's.

use crate::peers::{hda_limits, hda_ops, sink};
use crate::wire;

#[test]
fn ops_are_numbered_as_driver_hda0_numbers_them() {
    assert_eq!(wire::OP_HEALTHCHECK, hda_ops::OP_HEALTHCHECK);
    assert_eq!(wire::OP_WRITE_PCM, hda_ops::OP_WRITE_PCM);
    assert_eq!(wire::OP_STREAM_START, hda_ops::OP_STREAM_START);
    assert_eq!(wire::OP_STREAM_STOP, hda_ops::OP_STREAM_STOP);
    assert_eq!(wire::OP_OUTPUT_STATUS, hda_ops::OP_OUTPUT_STATUS);
}

#[test]
fn lengths_are_driver_hda0s_and_the_sinks() {
    assert_eq!(wire::STATUS_LEN, hda_limits::STATUS_LEN);
    assert_eq!(wire::MAX_PCM_CHUNK, hda_limits::MAX_PCM_CHUNK);
    assert_eq!(wire::OUTPUT_STATUS_PAYLOAD_LEN, hda_limits::OUTPUT_STATUS_PAYLOAD_LEN);
    assert_eq!(wire::MAX_PCM_CHUNK, sink::MAX_PCM_CHUNK);
    assert_eq!(wire::HDR_LEN, sink::HDR_LEN);
}
