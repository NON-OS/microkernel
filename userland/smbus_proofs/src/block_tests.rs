// NONOS Operating System (AGPL-3.0-or-later)
//! Block transfers through the 32-byte buffer.

use crate::errno::{E_NOTSUP, E_PROTO};
use crate::model::{Device, Model};
use crate::regs::*;
use crate::transfer_tests::{run, xfer};
use crate::xfer::Kind;

#[test]
fn a_block_read_takes_the_devices_length_and_turns_the_buffer_off_after() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    assert_eq!(run(&mut m, &xfer(0x50, true, Kind::Block, 0, &[])), Ok(vec![0xAA, 0xBB, 0xCC]));
    assert_eq!(m.wrote(AUXCTL), vec![AUX_E32B, 0]);
    assert_eq!(m.wrote(HSTCNT), vec![CNT_BLOCK_DATA | CNT_START]);
}

#[test]
fn a_block_write_sends_its_length_then_its_bytes() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    let bytes: Vec<u8> = (1..=32).collect();
    assert_eq!(run(&mut m, &xfer(0x50, false, Kind::Block, 9, &bytes)), Ok(vec![]));
    assert_eq!(m.devices[0].written, bytes);
    assert_eq!(m.wrote(HSTDAT0), vec![32]);
}

#[test]
fn a_length_outside_one_to_thirty_two_is_a_protocol_error() {
    for len in [0u8, 33] {
        let mut d = Device::eeprom(0x50);
        d.block = std::iter::once(len).chain(std::iter::repeat_n(1, 33)).collect();
        let mut m = Model::new(vec![d]);
        assert_eq!(run(&mut m, &xfer(0x50, true, Kind::Block, 0, &[])), Err(E_PROTO), "{len}");
        assert_eq!(m.aux & AUX_E32B, 0, "the buffer is turned off after an error too");
    }
}

#[test]
fn a_host_without_the_buffer_refuses_blocks_and_starts_nothing() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    m.e32b = false;
    assert_eq!(run(&mut m, &xfer(0x50, true, Kind::Block, 0, &[])), Err(E_NOTSUP));
    assert!(m.wrote(HSTCNT).is_empty());
}
