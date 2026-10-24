// NONOS Operating System (AGPL-3.0-or-later)
//! The plan each protocol makes: what goes in HST_CMD, which data registers
//! are used, and the address byte's read bit.

use crate::regs::*;
use crate::transfer_tests::xfer;
use crate::xfer::plan::{plan, Data};
use crate::xfer::Kind;

#[test]
fn each_protocol_names_its_command_and_data() {
    for (kind, read, command, data) in [
        (Kind::Quick, false, None, Data::None),
        (Kind::Byte, false, Some(9), Data::None),
        (Kind::Byte, true, None, Data::Low),
        (Kind::ByteData, true, Some(9), Data::Low),
        (Kind::WordData, false, Some(9), Data::Pair),
        (Kind::Block, true, Some(9), Data::Buffer),
    ] {
        let bytes: &[u8] = if kind == Kind::WordData { &[1, 2] } else { &[] };
        let p = plan(&xfer(0x50, read, kind, 9, bytes));
        assert_eq!((p.command, p.data), (command, data), "{kind:?} read={read}");
    }
}

#[test]
fn the_address_byte_carries_the_read_bit_and_start_is_set() {
    let p = plan(&xfer(0x2C, true, Kind::WordData, 0, &[]));
    assert_eq!((p.address, p.read), (0x59, true));
    assert_eq!(p.start, CNT_WORD_DATA | CNT_START);
    let p = plan(&xfer(0x2C, false, Kind::Quick, 0, &[]));
    assert_eq!((p.address, p.start), (0x58, CNT_QUICK | CNT_START));
}
