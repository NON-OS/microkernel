// NONOS Operating System (AGPL-3.0-or-later)
//! Byte, word and quick transfers against the modelled host.

use crate::model::{Device, Model};
use crate::regs::*;
use crate::xfer::{transfer, Kind, Xfer, BLOCK_MAX};

pub fn xfer(addr: u8, read: bool, kind: Kind, cmd: u8, data: &[u8]) -> Xfer {
    let mut d = [0u8; BLOCK_MAX];
    d[..data.len()].copy_from_slice(data);
    Xfer { addr, read, kind, cmd, len: data.len() as u8, data: d }
}

pub fn run(m: &mut Model, x: &Xfer) -> Result<Vec<u8>, i32> {
    let mut out = [0u8; BLOCK_MAX];
    transfer(m, x, &mut out).map(|n| out[..n].to_vec())
}

#[test]
fn a_byte_data_read_addresses_reads_and_gives_the_host_back() {
    let mut m = Model::new(vec![Device::eeprom(0x50)]);
    m.busy_reads = 3;
    assert_eq!(run(&mut m, &xfer(0x50, true, Kind::ByteData, 2, &[])), Ok(vec![2 ^ 0x5A]));
    assert_eq!(m.wrote(HSTADD), vec![0xA1]);
    assert_eq!(m.wrote(HSTCMD), vec![2]);
    assert_eq!(m.wrote(HSTCNT), vec![CNT_BYTE_DATA | CNT_START]);
    // The last write gives the semaphore back and clears what was left.
    assert_eq!(m.writes.last(), Some(&(HSTSTS, STS_INUSE | STS_FLAGS)));
    assert_eq!(m.sts & (STS_INUSE | STS_INTR), 0);
}

#[test]
fn word_writes_and_reads_go_low_byte_first() {
    let mut m = Model::new(vec![Device::eeprom(0x2C)]);
    assert_eq!(run(&mut m, &xfer(0x2C, false, Kind::WordData, 0x10, &[0x34, 0x12])), Ok(vec![]));
    assert_eq!((m.devices[0].mem[0x10], m.devices[0].mem[0x11]), (0x34, 0x12));
    assert_eq!(m.wrote(HSTADD), vec![0x58]);
    assert_eq!(run(&mut m, &xfer(0x2C, true, Kind::WordData, 0x10, &[])), Ok(vec![0x34, 0x12]));
}

#[test]
fn a_byte_send_carries_its_byte_in_the_command_and_a_receive_none() {
    let mut m = Model::new(vec![Device::eeprom(0x51)]);
    assert_eq!(run(&mut m, &xfer(0x51, false, Kind::Byte, 7, &[])), Ok(vec![]));
    assert_eq!(m.devices[0].ptr, 7);
    m.writes.clear();
    assert_eq!(run(&mut m, &xfer(0x51, true, Kind::Byte, 0x99, &[])), Ok(vec![7 ^ 0x5A]));
    assert!(m.wrote(HSTCMD).is_empty());
}

#[test]
fn a_quick_command_writes_no_command_or_data() {
    let mut m = Model::new(vec![Device::eeprom(0x52)]);
    assert_eq!(run(&mut m, &xfer(0x52, false, Kind::Quick, 0x77, &[])), Ok(vec![]));
    assert!(m.wrote(HSTCMD).is_empty() && m.wrote(HSTDAT0).is_empty());
    assert_eq!(m.wrote(HSTCNT), vec![CNT_QUICK | CNT_START]);
}
