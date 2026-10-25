// NONOS Operating System (AGPL-3.0-or-later)
//! Busy hosts, missing devices, errors and timeouts.

use crate::errno::*;
use crate::model::{Device, Model};
use crate::regs::*;
use crate::transfer_tests::{run, xfer};
use crate::xfer::Kind;

fn read(m: &mut Model, addr: u8) -> Result<Vec<u8>, i32> {
    run(m, &xfer(addr, true, Kind::ByteData, 0, &[]))
}

#[test]
fn nobody_at_the_address_is_enxio() {
    assert_eq!(read(&mut Model::new(vec![]), 0x50), Err(E_NXIO));
}

#[test]
fn a_host_idle_before_its_result_shows_is_waited_for() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    m.lag = 2;
    assert_eq!(read(&mut m, 0x50), Ok(vec![0x5A]));
}

#[test]
fn a_busy_host_is_refused_without_a_start() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    (m.sts, m.stuck) = (STS_HOST_BUSY, true);
    assert_eq!(read(&mut m, 0x50), Err(E_BUSY));
    assert!(m.wrote(HSTCNT).is_empty());
}

#[test]
fn flags_a_transfer_left_are_cleared_before_the_next() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    m.sts = STS_DEV_ERR | STS_INTR;
    assert_eq!(read(&mut m, 0x50), Ok(vec![0x5A]));
    assert_eq!(m.wrote(HSTSTS)[0], STS_DEV_ERR | STS_INTR);
}

#[test]
fn a_lost_arbitration_outranks_a_missing_device_outranks_a_kill() {
    for (bits, want) in [
        (STS_BUS_ERR | STS_DEV_ERR | STS_FAILED, E_AGAIN),
        (STS_DEV_ERR | STS_FAILED, E_NXIO),
        (STS_FAILED, E_IO),
    ] {
        let mut m = Model::new(vec![Device::eeprom(0x50)]);
        m.force = Some(bits);
        assert_eq!(read(&mut m, 0x50), Err(want), "{bits:#x}");
    }
}

#[test]
fn a_host_that_never_finishes_is_killed_after_its_timeout() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    m.stuck = true;
    assert_eq!(read(&mut m, 0x50), Err(E_TIMEDOUT));
    assert_eq!(m.wrote(HSTCNT), vec![CNT_BYTE_DATA | CNT_START, CNT_KILL, 0]);
    assert!((200..210).contains(&m.now), "waited {} ms", m.now);
    assert_eq!(m.sts & STS_INUSE, 0, "given back after a timeout too");
}
