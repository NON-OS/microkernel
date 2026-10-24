// NONOS Operating System (AGPL-3.0-or-later)
//! A block read the device or the bus breaks part way: a NAK, a lost
//! arbitration or a stretch past the bound ends the transfer with its
//! error, and no byte of the block is handed up as if it were whole. A
//! result with both INTR and an error bit is the error. PEC is never
//! turned on, even left on by firmware, and a request asking for it is
//! refused.

use crate::errno::{E_AGAIN, E_NXIO, E_TIMEDOUT};
use crate::model::{Device, Model};
use crate::regs::{AUX_CRC, AUX_E32B, STS_BUS_ERR, STS_DEV_ERR, STS_INTR};
use crate::transfer_tests::{run, xfer};
use crate::xfer::{parse, Kind};

fn block_read(m: &mut Model) -> Result<Vec<u8>, i32> {
    run(m, &xfer(0x50, true, Kind::Block, 0, &[]))
}

#[test]
fn a_block_broken_part_way_is_its_error_never_a_short_block() {
    for (bits, want) in
        [(STS_DEV_ERR, E_NXIO), (STS_BUS_ERR, E_AGAIN), (STS_INTR | STS_DEV_ERR, E_NXIO)]
    {
        let mut m = Model::new(vec![Device::eeprom(0x50)]);
        m.force = Some(bits);
        assert_eq!(block_read(&mut m), Err(want), "{bits:#x}");
        assert_eq!(m.aux & (AUX_E32B | AUX_CRC), 0, "the buffer and CRC off after");
    }
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    m.stuck = true;
    assert_eq!(block_read(&mut m), Err(E_TIMEDOUT), "a stretch past the bound");
}

#[test]
fn pec_is_never_asked_for() {
    assert!(parse(&[0x50, 0b10, 4, 0, 0]).is_none(), "a PEC flag is refused");
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    m.aux = AUX_CRC;
    assert_eq!(block_read(&mut m), Ok(vec![0xAA, 0xBB, 0xCC]));
    let aux = m.wrote(crate::regs::AUXCTL);
    assert!(aux.iter().all(|v| v & AUX_CRC == 0), "CRC off, though firmware left it on");
}
